use crate::all::*;

lazy_static! {
  /// Six digit hex color with prefix "#" or "0x".
  static ref RE_HEX6: Regex = Regex::new(r"(#|0x)?([[:xdigit:]]{2})([[:xdigit:]]{2})([[:xdigit:]]{2})").unwrap();

  /// Three digit hex color with prefix "#" or "0x".
  static ref RE_HEX3: Regex = Regex::new(r"(#|0x)?([[:xdigit:]])([[:xdigit:]])([[:xdigit:]])").unwrap();
}

pub struct ColorFile {
  pub tokens: Vec<Token>,
  pub color_count: usize,
  pub path: PathBuf,
}

impl ColorFile {
  pub fn new(path: &Path, parse_options: &ParseOptions) -> Result<Self> {
    let source = std::fs::read_to_string(path)
      .with_context(fformat!("read file `{}`.", path.display()))?;

    let (tokens, color_count) = parse_text(&source, parse_options)?;
    Ok(ColorFile {
      tokens,
      color_count,
      path: path.to_owned(),
    })
  }

  pub fn save(&self) -> Result<()> {
    let mut file = File::create(&self.path)
      .with_context(fformat!("Open file `{}` for writing.", self.path.display()))?;

    for token in &self.tokens {
      match token {
        Token::Color(color_kind, color32) => {
          write!(file, "{}", color_to_text(color_kind, *color32))
            .with_context(fformat!("write file `{}`.", self.path.display()))?;
        },
        Token::Text(s) => {
          write!(file, "{}", s)
            .with_context(fformat!("write file `{}`.", self.path.display()))?;
        },
      }
    }
    Ok(())
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
  Color(ColorKind, Color32),
  Text(String),
}

#[derive(Clone, Debug)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum ColorKind {
  // eg `#ebc17a` where prefix is `#`. The prefix can be empty string.
  Hex6 { prefix: String },

  // eg `#ebc`
  Hex3 { prefix: String },

  // eg "ebc17a" or 'ebc17a'. Could help avoid false positives compared to no-prefix matching.
  // DelimitedHex6,

  // eg (255, 255, 128) or [255, 255, 128]
  // Tuple3
}

impl ColorKindDiscriminants {
  pub fn regex(self) -> Regex {
    match self {
      ColorKindDiscriminants::Hex6 => RE_HEX6.clone(),
      ColorKindDiscriminants::Hex3 => RE_HEX3.clone(),
    }
  }
}

fn parse_color(
  d: ColorKindDiscriminants,
  c: &regex::Captures,
) -> Result<(ColorKind, Color32)> {
  match d {
    ColorKindDiscriminants::Hex6 => {
      let prefix = match c.get(1) {
        None => String::from(""),
        Some(m) => m.as_str().to_string(),
      };

      Ok((ColorKind::Hex6 { prefix }, Color32::from_rgb(
        u8::from_str_radix(&c[2], 16)?,
        u8::from_str_radix(&c[3], 16)?,
        u8::from_str_radix(&c[4], 16)?,
      )))
    },
    ColorKindDiscriminants::Hex3 => {
      let prefix = match c.get(1) {
        None => String::from(""),
        Some(m) => m.as_str().to_string(),
      };

      // 17 = 16 + 1, so eg "#a" becomes "#aa".
      Ok((ColorKind::Hex3 { prefix }, Color32::from_rgb(
        17 * u8::from_str_radix(&c[2], 16)?,
        17 * u8::from_str_radix(&c[3], 16)?,
        17 * u8::from_str_radix(&c[4], 16)?,
      )))
    },
  }
}

pub fn color_to_text(color_kind: &ColorKind, color32: Color32) -> String {
  match color_kind {
    ColorKind::Hex6 { prefix } => {
      format!("{}{:02x}{:02x}{:02x}", &prefix, color32.r(), color32.g(), color32.b())
    },
    ColorKind::Hex3 { prefix } => {
      format!("{}{:x}{:x}{:x}", &prefix, color32.r() / 16, color32.g() / 16, color32.b() / 16)
    },
  }
}

fn parse_text(
  source: &str,
  parse_options: &ParseOptions,
) -> Result<(Vec<Token>, usize)> {
  #[derive(Clone)]
  struct ColorRange {
    range: Range<usize>,
    color_kind: ColorKind,
    color32: Color32,
  }

  let mut used_ranges: Vec<ColorRange> = vec![];
  let mut new_ranges = vec![];

  let overlaps = |r: &Range<usize>, used_ranges: &[ColorRange]| -> bool {
    for ur in used_ranges {
      let ur = &ur.range;
      if r.start <= ur.start && r.end > ur.start {
        return true;
      }
      if r.start < ur.end && r.end >= ur.end {
        return true;
      }
      if ur.start >= r.end {
        return false;
      }
    }
    false
  };

  for color_kind_d in ColorKindDiscriminants::iter() {
    new_ranges.clear();
    let re = color_kind_d.regex();
    let matches: Vec<_> = re.find_iter(source).collect();
    for m in matches {
      let r = m.range();
      if overlaps(&r, &used_ranges) { continue }

      // There does not seem to be a way to get both the range and the captures
      // at the same time. This should always succeed.
      let Some(captures) = re.captures(m.as_str()) else { continue };

      let (color_kind, color32) = parse_color(color_kind_d, &captures)?;

      match color_kind {
        ColorKind::Hex6 { ref prefix } => {
          if !parse_options.prefixless6 && prefix.is_empty() { continue }
        },
        ColorKind::Hex3 { ref prefix } => {
          if !parse_options.prefixless3 && prefix.is_empty() { continue }
        },
      }

      new_ranges.push(ColorRange {
        range: r,
        color_kind,
        color32,
      });
    }
    used_ranges.extend_from_slice(&new_ranges);
    used_ranges.sort_by(|a, b| a.range.start.cmp(&b.range.start));
  }

  let bytes = source.as_bytes();
  let mut tokens = vec![];
  let mut c = 0;
  let mut color_count = 0;
  for color_range in used_ranges {
    let next = color_range.range.start;
    if c < next {
      let s = String::from_utf8(bytes[c..next].into())
        .with_context(fformat!("read middle bytes to String at {}..{}.", c, next))?;
      tokens.push(Token::Text(s));
    }
    tokens.push(Token::Color(color_range.color_kind, color_range.color32));
    color_count += 1;
    c = color_range.range.end;
  }

  let next = bytes.len();
  if c < next {
    let s = String::from_utf8(bytes[c..next].into())
      .with_context(fformat!("read final bytes to String at {}..{}.", c, next))?;
    tokens.push(Token::Text(s));
  }

  Ok((tokens, color_count))
}

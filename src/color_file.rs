use crate::all::*;

lazy_static! {
  static ref RE_HEX6: Regex = Regex::new(r"#?([[:xdigit:]]{2})([[:xdigit:]]{2})([[:xdigit:]]{2})").unwrap();
  static ref RE_HEX3: Regex = Regex::new(r"#?([[:xdigit:]])([[:xdigit:]])([[:xdigit:]])").unwrap();
}

pub struct ColorFile {
  pub tokens: Vec<Token>,
}

impl ColorFile {
  pub fn new(path: &Path) -> Result<Self> {
    let source = std::fs::read_to_string(path)
      .with_context(fformat!("read file `{}`.", path.display()))?;

    let tokens = parse_text(&source)?;
    Ok(ColorFile {
      tokens,
    })
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
  Color(ColorKind, Color32),
  Text(String),
}

#[derive(Clone, Copy, Debug)]
pub enum ColorKind {
  Hex6, // eg #ebc17a
  Hex3, // eg #ebc
  // DelimitedHex6, // eg "ebc17a" or 'ebc17a'
  // Tuple3 // eg (255, 255, 128)
}

impl ColorKind {
  pub fn regex(self) -> Regex {
    match self {
      ColorKind::Hex6 => Regex::new(r"#[[:xdigit:]]{6}").unwrap(),
      ColorKind::Hex3 => Regex::new(r"#[[:xdigit:]]{3}").unwrap(),
    }
  }

  pub fn to_color32(self, s: &str) -> Result<Color32> {
    match self {
      ColorKind::Hex6 => {
        if let Some(c) = RE_HEX6.captures(s) {
          Ok(Color32::from_rgb(
            u8::from_str_radix(&c[1], 16)?,
            u8::from_str_radix(&c[2], 16)?,
            u8::from_str_radix(&c[3], 16)?,
          ))
        }
        else {
          bail!("Failed to parse Hex6: `{}`", s);
        }
      },
      ColorKind::Hex3 => {
        if let Some(c) = RE_HEX3.captures(s) {
          // 17 = 16 + 1, so eg "0xa" becomes "0xaa".
          Ok(Color32::from_rgb(
            17 * u8::from_str_radix(&c[1], 16)?,
            17 * u8::from_str_radix(&c[2], 16)?,
            17 * u8::from_str_radix(&c[3], 16)?,
          ))
        }
        else {
          bail!("Failed to parse Hex6: `{}`", s);
        }
      },
    }
  }
}

pub fn color_to_text(color_kind: ColorKind, color32: Color32) -> String {
  match color_kind {
    ColorKind::Hex6 => {
      format!("#{:02x}{:02x}{:02x}", color32.r(), color32.g(), color32.b())
    },
    ColorKind::Hex3 => {
      format!("#{:x}{:x}{:x}", color32.r() / 16, color32.g() / 16, color32.b() / 16)
    },
  }
}

fn parse_text(source: &str) -> Result<Vec<Token>> {
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

  use ColorKind::*;
  for color_kind in [Hex6, Hex3] {
    new_ranges.clear();
    let re = color_kind.regex();
    let matches: Vec<_> = re.find_iter(source).collect();
    for m in matches {
      let r = m.range();
      if overlaps(&r, &used_ranges) {
        continue
      }
      new_ranges.push(ColorRange {
        range: r,
        color_kind,
        color32: color_kind.to_color32(m.as_str())?,
      });
    }
    used_ranges.extend_from_slice(&new_ranges);
    used_ranges.sort_by(|a, b| a.range.start.cmp(&b.range.start));
  }

  let bytes = source.as_bytes();
  let mut tokens = vec![];
  let mut c = 0;
  for color_range in used_ranges {
    let next = color_range.range.start;
    if c < next {
      let s = String::from_utf8(bytes[c..next].into())
        .with_context(fformat!("read middle bytes to String at {}..{}.", c, next))?;
      tokens.push(Token::Text(s));
    }
    tokens.push(Token::Color(color_range.color_kind, color_range.color32));
    c = color_range.range.end;
  }

  let next = bytes.len();
  if c < next {
    let s = String::from_utf8(bytes[c..next].into())
      .with_context(fformat!("read final bytes to String at {}..{}.", c, next))?;
    tokens.push(Token::Text(s));
  }

  Ok(tokens)
}

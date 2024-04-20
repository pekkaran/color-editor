use crate::all::*;

pub struct ColorFile {
}

impl ColorFile {
  pub fn new(path: &Path) -> Result<Self> {
    let source = std::fs::read_to_string(path)
      .with_context(fformat!("read file `{}`.", path.display()))?;

    parse_text(&source)?;

    Ok(ColorFile {
    })
  }
}

#[allow(dead_code)]
enum Token {
  Color(ColorKind, Color32),
  Text(Vec<u8>),
}

#[derive(Clone, Copy)]
enum ColorKind {
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

  pub fn to_color(self, _s: &str) -> Color32 {
    match self {
      ColorKind::Hex6 => Color32::BLUE, // TODO
      ColorKind::Hex3 => Color32::GREEN,
    }
  }
}

fn parse_text(source: &str) -> Result<Vec<Token>> {
  let mut used_ranges: Vec<Range<usize>> = vec![];
  let mut new_ranges = vec![];
  let mut tokens = vec![];

  let overlaps = |r: &Range<usize>, used_ranges: &[Range<usize>]| -> bool {
    for ur in used_ranges {
      if r.start <= ur.start && r.end > ur.start {
        return true;
      }
      if r.start < ur.end && r.end >= ur.end {
        return true;
      }
      if r.start >= ur.end {
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
      if overlaps(&r, &used_ranges) { continue }
      new_ranges.push(r);
      tokens.push(Token::Color(color_kind, color_kind.to_color(m.as_str())));
    }
    used_ranges.extend_from_slice(&new_ranges);
    used_ranges.sort_by(|a, b| a.start.cmp(&b.start));
  }

  Ok(vec![])
}

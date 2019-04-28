use std::io::{Result, Write};
use termion::color;

pub trait Colorer {
  fn color(&self, writer: &mut impl Write, s: &str, prev: Option<u8>, cur: u8) -> Result<()>;
}

pub struct AbsoluteColorer {
  color_null: String,
  color_space: String,
  color_printable: String,
  color_unprintable: String,
  color_rest: String,
  color_reset: String,
}

impl AbsoluteColorer {
  pub fn new() -> Self {
    AbsoluteColorer {
      color_null: color::Rgb(242, 60, 80).fg_string(),
      color_space: color::Rgb(74, 217, 217).fg_string(),
      color_printable: color::Rgb(233, 255, 223).fg_string(),
      color_unprintable: color::Rgb(255, 203, 5).fg_string(),
      color_rest: color::Rgb(54, 177, 191).fg_string(),
      color_reset: String::from(color::Reset.fg_str()),
    }
  }
}

impl Colorer for AbsoluteColorer {
  fn color(&self, writer: &mut impl Write, s: &str, _prev: Option<u8>, cur: u8) -> Result<()> {
    let c = match cur {
      0 => &self.color_null,
      36 => &self.color_space,
      1...32 => &self.color_unprintable,
      33...126 => &self.color_printable,
      _ => &self.color_rest,
    };

    write!(writer, "{}{}{}", c, s, self.color_reset)
  }
}

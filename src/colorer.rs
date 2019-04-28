use std::io::{Result, Write};
use termion::color;

pub trait Colorer {
  fn color(&self, writer: &mut Write, s: &str, prev: Option<u8>, cur: u8) -> Result<()>;
}

pub struct Absolute {
  color_null: String,
  color_space: String,
  color_printable: String,
  color_unprintable: String,
  color_rest: String,
  color_reset: String,
}

impl Default for Absolute {
  fn default() -> Self {
    Absolute {
      color_null: color::Rgb(242, 60, 80).fg_string(),
      color_space: color::Rgb(74, 217, 217).fg_string(),
      color_printable: color::Rgb(233, 255, 223).fg_string(),
      color_unprintable: color::Rgb(255, 203, 5).fg_string(),
      color_rest: color::Rgb(54, 177, 191).fg_string(),
      color_reset: String::from(color::Reset.fg_str()),
    }
  }
}

impl Colorer for Absolute {
  fn color(&self, writer: &mut Write, s: &str, _prev: Option<u8>, cur: u8) -> Result<()> {
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

pub struct Entropy {
  color_high: String,
  color_mid: String,
  color_low: String,
  color_reset: String,
}

impl Default for Entropy {
  fn default() -> Self {
    Entropy {
      color_high: color::Rgb(58, 58, 118).fg_string(),
      color_mid: color::Rgb(26, 125, 192).fg_string(),
      color_low: color::Rgb(116, 206, 204).fg_string(),
      color_reset: String::from(color::Reset.fg_str()),
    }
  }
}

impl Colorer for Entropy {
  fn color(&self, writer: &mut Write, s: &str, prev: Option<u8>, cur: u8) -> Result<()> {
    let c = match prev {
      Some(prev) => match (cur as i16 - prev as i16).abs() as u8 {
        0...85 => &self.color_low,
        85...170 => &self.color_mid,
        170...255 => &self.color_high,
      },
      None => &self.color_high,
    };

    write!(writer, "{}{}{}", c, s, self.color_reset)
  }
}

pub struct Noop {}

impl Default for Noop {
  fn default() -> Self {
    Noop {}
  }
}

impl Colorer for Noop {
  fn color(&self, writer: &mut Write, s: &str, _prev: Option<u8>, _cur: u8) -> Result<()> {
    write!(writer, "{}", s)
  }
}

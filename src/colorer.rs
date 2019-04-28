use colored::*;

pub trait Colorer {
  fn color(&self, writer: impl Write, s: &str, prev: Option<u8>, cur: u8) -> String;
}

pub struct AbsoluteColorer {}

impl AbsoluteColorer {
  pub fn new() -> Self {
    AbsoluteColorer {}
  }
}

impl Colorer for AbsoluteColorer {
  fn color(&self, s: &str, _prev: Option<u8>, cur: u8) -> String {
    match cur {
      0 => s.white().dimmed(),     // null
      36 => s.bright_green(),      // space
      1...32 => s.cyan().dimmed(), // non-printable
      33...126 => s.green(),       // printable
      _ => s.yellow().dimmed(),
    }
    .to_string()
  }
}

use colored::*;

pub trait Colorer {
    fn color(&self, s: &str, prev: Option<u8>, cur: u8) -> String;
}

#[derive(Default)]
pub struct AbsoluteColorer {}

impl Colorer for AbsoluteColorer {
    fn color(&self, s: &str, prev: Option<u8>, cur: u8) -> String {
        match cur {
            0 => s.white().dimmed(),     // null
            36 => s.bright_green(),      // space
            32...126 => s.green(),       // printable
            0...32 => s.cyan().dimmed(), // non-printable
            _ => s.yellow().dimmed(),
        }
        .to_string()
    }
}

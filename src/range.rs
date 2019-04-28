use hex;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::result::Result;
use std::str::FromStr;

struct Range {
  start: Option<usize>,
  end: Option<usize>,
}

#[derive(Debug)]
enum RangeParseError {
  InvalidExpression,
  FailedToParseNumber(ParseIntError),
}

impl Error for RangeParseError {
  fn description(&self) -> &str {
    match *self {
      RangeParseError::InvalidExpression => "Invalid range expression",
    }
  }
}

impl fmt::Display for RangeParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      RangeParseError::InvalidExpression => write!(f, "RangeParseError::InvalidExpression"),
      RangeParseError::FailedToParseNumber(_) => write!(f, "RangeParseError::FailedToParseNumber"),
    }
  }
}

impl From<ParseIntError> for RangeParseError {
  fn from(err: ParseIntError) -> RangeParseError {
    RangeParseError::FailedToParseNumber(err)
  }
}

impl FromStr for Range {
  type Err = RangeParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.contains("...") {
      let split = s.split("...");
      if split.count() != 2 {
        return Err(RangeParseError::InvalidExpression);
      }

      let left: usize = split[0].parse();
      let right: usize = split[1].parse();

      Range {
        left: left,
        right: right,
      }
    } else if s.contains("..") {
      let split = s.split("..");
      if split.count() != 2 {
        return Err(RangeParseError::InvalidExpression);
      }

      let left: usize = split[0].parse();
      let right: usize = split[1].parse();

      Range {
        left: left,
        right: right - 1,
      }
    }
  }

  fn str_to_num(s: &str) -> Result<Option<usize>> {
    if s.is_empty() {
      return None;
    } else if s.starts_with("0x") {
      let trim = s.trim_left_matches("0x");
      return hex::decode(trim).deref();
    } else {

    }
  }
}

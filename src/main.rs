extern crate term_size;
#[macro_use]
extern crate clap;
extern crate atty;
extern crate env_logger;
extern crate hex;
extern crate termion;

mod colorer;
mod range_reader;
mod writer;

use range_reader::RangeReader;
use std::fmt;
use std::fs::File;
use std::io::{copy, stdin, Write};
use writer::HexWriterBuilder;

#[derive(Debug)]
enum Error {
  Io(std::io::Error),
  ParseInt(std::num::ParseIntError),
  InvalidArguments(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => err.fmt(f),
      Error::ParseInt(ref err) => err.fmt(f),
      Error::InvalidArguments(ref s) => f.write_str(s),
    }
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Io(ref err) => err.description(),
      Error::ParseInt(ref err) => err.description(),
      Error::InvalidArguments(ref s) => s,
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Error::Io(e)
  }
}

impl From<std::num::ParseIntError> for Error {
  fn from(e: std::num::ParseIntError) -> Self {
    Error::ParseInt(e)
  }
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
  env_logger::init();

  let matches = clap_app!(myapp =>
      (version: "0.1")
      (author: "Sam Rose <hello@samwho.dev>")
      (about: "Yet another hex viewer")
      (@arg INPUT: "File to use as input")
      (@arg START: -s --start +takes_value "Byte to start reading from, decimal or hex")
      (@arg END: -e --end +takes_value "Byte to read to, decimal or hex")
      (@arg NUM: -n --num +takes_value "Number of bytes to read")
      (@arg COLORER: -c --color +takes_value "Type of coloring, one of: none, absolute, entropy")
  )
  .get_matches();

  let mut start = match matches.value_of("START") {
    Some(f) => Some(parse_cli_number(f)?),
    None => None,
  };

  let mut end = match matches.value_of("END") {
    Some(t) => Some(parse_cli_number(t)?),
    None => None,
  };

  let num = match matches.value_of("NUM") {
    Some(n) => Some(parse_cli_number(n)?),
    None => None,
  };

  if start.is_some() && end.is_some() && num.is_some() {
    return Err(Error::InvalidArguments(String::from(
      "Must only specify two of: --start, --end, --num",
    )));
  }

  if start.is_none() && end.is_some() && num.is_some() {
    start = Some(end.unwrap() - num.unwrap());
  }

  if end.is_none() && start.is_some() && num.is_some() {
    end = Some(start.unwrap() + num.unwrap());
  }

  if end.is_none() && start.is_none() && num.is_some() {
    start = Some(0);
    end = Some(start.unwrap() + num.unwrap());
  }

  let mut reader: RangeReader = match matches.value_of("INPUT") {
    None => RangeReader::from_readable(stdin(), start, end),
    Some(path) => RangeReader::from_seekable(File::open(path)?, start, end)?,
  };

  let mut builder = HexWriterBuilder::default().start_position(start.unwrap_or(0));

  if atty::is(atty::Stream::Stdout) {
    builder = match matches.value_of("COLORER") {
      Some("none") => builder.colorer(colorer::Noop::default()),
      Some("entropy") => builder.colorer(colorer::Entropy::default()),
      None | Some("absolute") => builder.colorer(colorer::Absolute::default()),
      Some(other) => {
        return Err(Error::InvalidArguments(format!(
          "Unknown colorer {}",
          other
        )))
      }
    };
  }

  let mut writer = builder.build();

  copy(&mut reader, &mut writer)?;
  writer.flush().map_err(Into::into)
}

fn parse_cli_number(s: &str) -> Result<usize> {
  let radix = if s.starts_with("0x") { 16 } else { 10 };
  usize::from_str_radix(s.trim_start_matches("0x"), radix).map_err(Into::into)
}

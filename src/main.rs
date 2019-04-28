extern crate term_size;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate hex;
#[macro_use]
extern crate log;
extern crate env_logger;

mod colorer;
mod range_reader;
mod writer;

use range_reader::RangeReader;
use std::fmt;
use std::fs::File;
use std::io::{copy, stdin, BufReader, Read};
use writer::{HexWriter, HexWriterBuilder};

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::ParseInt(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::ParseInt(ref err) => err.description(),
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
        (@arg FROM: -f --from +takes_value "Byte to start reading from, decimal or hex")
        (@arg TO: -t --to +takes_value "Byte to read to, decimal or hex")
    )
    .get_matches();

    let from = match matches.value_of("FROM") {
        Some(f) => Some(parse_cli_number(f)?),
        None => None,
    };

    let to = match matches.value_of("TO") {
        Some(t) => Some(parse_cli_number(t)?),
        None => None,
    };

    let read: Box<Read> = match matches.value_of("INPUT") {
        None => Box::new(stdin()),
        Some(path) => Box::new(File::open(path)?),
    };

    let mut reader = RangeReader::new(BufReader::new(read), from, to);
    let mut writer_builder = HexWriterBuilder::default();
    if let Some(start) = from {
        writer_builder.start_position(start);
    }
    let mut writer = writer_builder.build();

    if let Err(e) = copy(&mut reader, &mut writer) {
        return Err(e.into());
    }

    Ok(())
}

fn parse_cli_number(s: &str) -> Result<usize> {
    let radix = if s.starts_with("0x") { 16 } else { 10 };
    usize::from_str_radix(s.trim_start_matches("0x"), radix).map_err(|e| e.into())
}

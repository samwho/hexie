extern crate term_size;
#[macro_use]
extern crate clap;

mod writer;

use std::io::{Result, BufReader, BufRead, copy, stdin};
use std::fs::File;

fn main() -> Result<()> {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Sam Rose <hello@samwho.dev>")
        (about: "Yet another hex viewer")
        (@arg INPUT: "File to use as input")
    ).get_matches();

    let mut reader: Box<BufRead> = match matches.value_of("INPUT") {
        None => Box::new(BufReader::new(stdin())),
        Some(path) => Box::new(BufReader::new(File::open(path)?))
    };

    let mut writer = writer::HexWriter::default();

    if let Err(e) = copy(&mut reader, &mut writer) {
        return Err(e);
    }

    Ok(())
}

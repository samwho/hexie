use super::colorer::{AbsoluteColorer, Colorer};
use colored::*;
use std::io::{stdout, Result, Stdout, Write};

pub struct HexWriter<W, C> {
    writer: W,
    width: usize,
    current_line: usize,
    start_position: usize,
    current_line_position: usize,
    colorer: C,
    previous_byte: Option<u8>,
    line: Vec<u8>,
}

pub struct HexWriterBuilder<W, C> {
    writer: Option<W>,
    colorer: Option<C>,
    width: usize,
    start_position: usize,
}

impl<W, C> HexWriterBuilder<W, C> {
    fn new() -> Self {
        HexWriterBuilder {
            writer: None,
            colorer: None,
            width: term_size::dimensions().unwrap().0,
            start_position: 0,
        }
    }
}

impl<W, C> HexWriterBuilder<W, C> {
    pub fn writer(&mut self, writer: W) -> &mut Self {
        self.writer = Some(writer);
        self
    }

    pub fn colorer(&mut self, colorer: C) -> &mut Self {
        self.colorer = Some(colorer);
        self
    }

    pub fn start_position(&mut self, start_position: usize) -> &mut Self {
        self.start_position = start_position;
        self
    }

    pub fn build(&mut self) -> HexWriter<W, C> {
        // TODO - use of `expect` should return a `Result` instead, once it's decided
        // what the error handling of the app should be
        HexWriter {
            writer: self.writer.take().expect("No writer specified"),
            width: self.width,
            current_line: 0,
            start_position: self.start_position,
            current_line_position: 0,
            colorer: self.colorer.take().expect("No colourer specified"),
            previous_byte: None,
            line: Vec::with_capacity(self.width / 4),
        }
    }
}

impl Default for HexWriter<Stdout, AbsoluteColorer> {
    fn default() -> Self {
        HexWriterBuilder::new()
            .writer(stdout())
            .colorer(Default::default())
            .build()
    }
}

impl<W, C> HexWriter<W, C>
where
    W: Write,
    C: Colorer,
{
    fn current_line_start_index(&self) -> usize {
        self.start_position + (self.current_line * self.width)
    }

    fn would_overflow_current_line(&self, s: usize) -> bool {
        self.current_line_position + s > (self.width as f64 / 4f64).ceil() as usize * 3
    }

    fn emit_right_hand_side(&mut self) -> Result<()> {
        let grey_dot = ".".white().dimmed().to_string();
        self.writer.write_all(" │ ".as_bytes())?;
        for byte in &self.line {
            match byte {
                32...126 => self.writer.write(&[*byte])?,
                _ => self.writer.write(grey_dot.as_bytes())?,
            };
        }
        self.line.clear();
        Ok(())
    }

    fn emit_new_line(&mut self) -> Result<()> {
        if self.current_line != 0 {
            self.emit_right_hand_side()?;
            self.writer.write_all(&[10])?; // newline
        }

        let s = format!("0x{:0>8X}", self.current_line_start_index())
            .white()
            .dimmed()
            .to_string();
        self.writer.write_all(s.as_bytes())?;
        self.writer.write_all(" │".as_bytes())?;
        self.current_line += 1;
        self.current_line_position = 13;
        Ok(())
    }

    fn emit_byte(&mut self, byte: u8) -> Result<usize> {
        let s = self
            .colorer
            .color(&format!(" {:0>2X}", byte), self.previous_byte, byte);
        let bytes = s.as_bytes();
        if self.current_line_position == 0 || self.would_overflow_current_line(3) {
            self.emit_new_line()?;
        }
        let c = self.writer.write(bytes)?;
        self.line.push(byte);
        self.current_line_position += 3;
        self.previous_byte = Some(byte);
        Ok(c)
    }
}

impl<W, C> Write for HexWriter<W, C>
where
    W: Write,
    C: Colorer,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        for byte in buf {
            self.emit_byte(*byte)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

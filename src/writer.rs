use super::colorer::{AbsoluteColorer, Colorer};
use colored::*;
use std::io::{stdout, Result, Stdout, Write};

pub struct HexWriter<W: Write, C: Colorer> {
    writer: Box<W>,
    width: usize,
    current_line: usize,
    start_position: usize,
    current_line_position: usize,
    colorer: Box<C>,
    previous_byte: Option<u8>,
    line: Vec<u8>,
}

pub struct HexWriterBuilder<W: Write, C: Colorer> {
    writer: Box<W>,
    colorer: Box<C>,
    width: usize,
    start_position: usize,
}

impl Default for HexWriterBuilder<Stdout, AbsoluteColorer> {
    fn default() -> HexWriterBuilder<Stdout, AbsoluteColorer> {
        let width = term_size::dimensions().unwrap_or((80, 0)).0;

        HexWriterBuilder {
            writer: Box::new(stdout()),
            colorer: Box::new(AbsoluteColorer::new()),
            width,
            start_position: 0,
        }
    }
}

impl<W: Write, C: Colorer> HexWriterBuilder<W, C> {
    pub fn writer(&mut self, writer: W) -> &mut Self {
        self.writer = Box::new(writer);
        self
    }

    pub fn colorer(&mut self, colorer: C) -> &mut Self {
        self.colorer = Box::new(colorer);
        self
    }

    pub fn start_position(&mut self, start_position: usize) -> &mut Self {
        self.start_position = start_position;
        self
    }

    pub fn build(self) -> HexWriter<W, C> {
        HexWriter {
            writer: self.writer,
            width: self.width,
            current_line: 0,
            start_position: self.start_position,
            current_line_position: 0,
            colorer: self.colorer,
            previous_byte: None,
            line: Vec::with_capacity(self.width / 4),
        }
    }
}

impl Default for HexWriter<Stdout, AbsoluteColorer> {
    fn default() -> Self {
        HexWriterBuilder::default().build()
    }
}

impl<W: Write, C: Colorer> HexWriter<W, C> {
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
                32...126 => self.writer.write_all(&[*byte])?,
                _ => self.writer.write_all(grey_dot.as_bytes())?,
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
        self.writer.write_all(bytes)?;
        self.line.push(byte);
        self.current_line_position += 3;
        self.previous_byte = Some(byte);
        Ok(3)
    }
}

impl<W: Write, C: Colorer> Write for HexWriter<W, C> {
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

use super::colorer;
use std::io::{stdout, Result, Write};

pub struct HexWriter {
  writer: Box<Write>,
  width: u16,
  current_line: usize,
  start_position: usize,
  current_line_position: usize,
  colorer: Box<colorer::Colorer>,
  previous_byte: Option<u8>,
  line: Vec<u8>,
  bytes_written: usize,
}

pub struct HexWriterBuilder {
  writer: Box<Write>,
  colorer: Box<colorer::Colorer>,
  width: u16,
  start_position: usize,
}

impl Default for HexWriterBuilder {
  fn default() -> HexWriterBuilder {
    let width = termion::terminal_size().unwrap_or((80, 0)).0;

    HexWriterBuilder {
      writer: Box::new(stdout()),
      colorer: Box::new(colorer::Noop::default()),
      width,
      start_position: 0,
    }
  }
}

impl HexWriterBuilder {
  pub fn colorer(mut self, colorer: impl colorer::Colorer + 'static) -> Self {
    self.colorer = Box::new(colorer);
    self
  }

  pub fn start_position(mut self, start_position: usize) -> Self {
    self.start_position = start_position;
    self
  }

  pub fn build(self) -> HexWriter {
    HexWriter {
      writer: self.writer,
      width: self.width,
      current_line: 0,
      start_position: self.start_position,
      current_line_position: 0,
      colorer: self.colorer,
      previous_byte: None,
      line: Vec::with_capacity(usize::from(self.width) / 4),
      bytes_written: 0,
    }
  }
}

impl Default for HexWriter {
  fn default() -> Self {
    HexWriterBuilder::default().build()
  }
}

impl HexWriter {
  fn would_overflow_current_line(&self, s: usize) -> bool {
    self.current_line_position + s > (self.width as f64 / 4f64).ceil() as usize * 3
  }

  fn emit_right_hand_side(&mut self) -> Result<()> {
    self.writer.write_all(" │ ".as_bytes())?;
    for byte in &self.line {
      match byte {
        32...126 => self.writer.write_all(&[*byte])?,
        _ => self.writer.write_all(b".")?,
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

    let s = format!("0x{:0>8X}", self.start_position + self.bytes_written);
    self.writer.write_all(s.as_bytes())?;
    self.writer.write_all(" │".as_bytes())?;
    self.current_line += 1;
    self.current_line_position = 13;
    Ok(())
  }

  fn emit_byte(&mut self, byte: u8) -> Result<usize> {
    if self.current_line_position == 0 || self.would_overflow_current_line(3) {
      self.emit_new_line()?;
    }
    self.colorer.color(
      &mut self.writer,
      &format!(" {:0>2X}", byte),
      self.previous_byte,
      byte,
    )?;
    self.line.push(byte);
    self.current_line_position += 3;
    self.previous_byte = Some(byte);
    self.bytes_written += 1;
    Ok(3)
  }
}

impl Write for HexWriter {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    for byte in buf {
      self.emit_byte(*byte)?;
    }
    Ok(buf.len())
  }

  fn flush(&mut self) -> Result<()> {
    if !self.line.is_empty() {
      loop {
        if self.would_overflow_current_line(3) {
          self.emit_right_hand_side()?;
          self.writer.write_all(&[10])?; // newline
          break;
        }
        self.writer.write_all(&[32, 32, 32])?; // three spaces
        self.current_line_position += 3;
      }
    }

    self.writer.flush()
  }
}

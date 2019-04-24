use std::io::{stdout, Write, BufWriter, Result};

pub struct HexWriter<W: Write> {
  writer: BufWriter<W>,
  width: usize,
  current_line: usize,
  start_position: usize,
  current_line_position: usize,
}

impl Default for HexWriter<std::io::Stdout> {
  fn default() -> Self {
    HexWriter::wrap(stdout())
  }
}

impl<W: Write> HexWriter<W> {
  pub fn wrap(writer: W) -> Self {
    let width = term_size::dimensions().unwrap().0;

    HexWriter {
      writer: BufWriter::new(writer),
      width: width,
      current_line: 0,
      start_position: 0,
      current_line_position: 0,
    }
  }

  fn current_line_start_index(&self) -> usize {
    self.start_position + (self.current_line * self.width)
  }

  fn would_overflow_current_line(&self, s: usize) -> bool {
    self.current_line_position + s > self.width
  }

  fn emit_new_line(&mut self) -> Result<usize> {
    if self.current_line != 0 {
      self.writer.write(&[10])?; // newline
    }
    let c = self.writer.write(format!("0x{:0>8X}:", self.current_line_start_index()).as_bytes())?;
    self.current_line += 1;
    self.current_line_position = c;
    Ok(c - 1)
  }

  fn emit_byte(&mut self, byte: u8) -> Result<usize> {
    let s = format!(" {:0>2X}", byte);
    let bytes = s.as_bytes();
    if self.current_line_position == 0 || self.would_overflow_current_line(bytes.len()) {
      self.emit_new_line()?;
    }
    let c = self.writer.write(bytes)?;
    self.current_line_position += c;
    Ok(c)
  }
}

impl<W: Write> Write for HexWriter<W> {
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
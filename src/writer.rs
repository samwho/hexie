use std::io::{stdout, Write, BufWriter, Result};

#[derive(TypedBuilder)]
pub struct HexWriter<W: Write> {
  writer: BufWriter<W>,
  width: usize,

  #[builder(default=0)]
  bytes_written: usize,
  #[builder(default=0)]
  current_line: usize,
  #[builder(default=0)]
  start_position: usize,
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
      bytes_written: 0,
      current_line: 0,
      start_position: 0,
    }
  }

  fn current_line_start_index(&self) -> usize {
    self.start_position + (self.current_line * self.width)
  }

  fn emit_new_line(&mut self) -> Result<usize> {
    let c = self.writer.write(format!("\n0x{:0>8X}: ", self.current_line_start_index()).as_bytes())?;
    self.bytes_written += c;
    self.current_line += 1;
    Ok(c)
  }

  fn emit_byte(&mut self, byte: u8) -> Result<usize> {
    let c = self.writer.write(format!("{:0>2X} ", byte).as_bytes())?;
    self.bytes_written += c;
    Ok(c)
  }
}

impl<W: Write> Write for HexWriter<W> {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    let mut return_bytes = 0;
    let mut bytes_this_line = 0;

    for byte in buf {
      if bytes_this_line == 0 || bytes_this_line + 3 > self.width {
        bytes_this_line = self.emit_new_line()? - 1;
      }
      bytes_this_line += self.emit_byte(*byte)?;
      return_bytes += 1;
    }

    Ok(return_bytes)
  }

  fn flush(&mut self) -> Result<()> {
    self.writer.flush()
  }
}
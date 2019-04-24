use std::io::{Write, BufWriter, Result};

pub struct HexWriter<W: Write> {
  writer: BufWriter<W>,
  width: usize,
  bytes_written: usize,
  current_line: usize,
}

impl<W: Write> HexWriter<W> {
  pub fn wrap(writer: W, width: usize) -> Self {
    HexWriter {
      writer: BufWriter::new(writer),
      width: width,
      bytes_written: 0,
      current_line: 0,
    }
  }
}

impl<W: Write> Write for HexWriter<W> {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    let mut return_bytes = 0;
    let mut bytes_this_line = 0;

    for byte in buf {
      if bytes_this_line == 0 || bytes_this_line + 3 > self.width {
        let c = self.writer.write(format!("\n{:#X}: ", self.current_line * self.width).as_bytes())?;
        self.bytes_written += c;
        bytes_this_line = c - 1;
        self.current_line += 1;
      }

      let c = self.writer.write(format!("{:X} ", byte).as_bytes())?;
      self.bytes_written += c;
      bytes_this_line += c;

      return_bytes += 1;
    }

    Ok(return_bytes)
  }

  fn flush(&mut self) -> Result<()> {
    self.writer.flush()
  }
}
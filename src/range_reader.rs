use std::io::{Error, ErrorKind, Read, Result};

const BUF_SIZE: usize = 4096;

pub struct RangeReader<R: Read> {
  inner: R,
  start: Option<usize>,
  end: Option<usize>,
  pos: usize,
}

impl<R: Read> RangeReader<R> {
  pub fn new(inner: R, start: Option<usize>, end: Option<usize>) -> Self {
    RangeReader {
      inner,
      start,
      end,
      pos: 0,
    }
  }

  fn advance(&mut self, n: usize) -> Result<usize> {
    let iterations = n / BUF_SIZE;
    let tbuf: &mut [u8] = &mut [0; BUF_SIZE];
    for _i in 0..iterations {
      match self.inner.read_exact(tbuf) {
        Ok(_) => {
          self.pos += BUF_SIZE;
        }
        Err(e) => return Err(e),
      }
    }

    let mut tvec = vec![0; n - self.pos];
    match self.inner.read_exact(tvec.as_mut_slice()) {
      Ok(_) => {
        self.pos += tvec.len();
      }
      Err(e) => return Err(e),
    };

    Ok(self.pos)
  }
}

impl<R: Read> Read for RangeReader<R> {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    if self.end != None && self.pos >= self.end.unwrap() {
      return Ok(0);
    }

    if self.pos == 0 && self.start != None {
      self.advance(self.start.unwrap())?;
    }

    let n = self.inner.read(buf)?;
    if let Some(end) = self.end {
      if self.pos + n > end {
        let r = end - self.pos;
        self.pos += n;
        return Ok(r);
      }
    }
    return Ok(n);
  }
}

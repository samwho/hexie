use std::io::{Read, Result, Seek, SeekFrom};

const BUF_SIZE: usize = 4096;

pub struct RangeReader {
  inner: Box<Read>,
  start: Option<usize>,
  end: Option<usize>,
  pos: usize,
}

impl RangeReader {
  pub fn from(inner: impl Seek + Read, start: Option<usize>, end: Option<usize>) -> Result<Self> {
    let s = start.unwrap_or(0);
    inner.seek(SeekFrom::Start(s as u64))?;

    Ok(RangeReader {
      inner: Box::new(inner),
      start,
      end,
      pos: s,
    })
  }

  pub fn new(inner: impl Read, start: Option<usize>, end: Option<usize>) -> Self {
    RangeReader {
      inner: Box::new(inner),
      start,
      end,
      pos: 0,
    }
  }

  fn advance(&mut self, n: usize) -> Result<usize> {
    let tbuf: &mut [u8] = &mut [0; BUF_SIZE];
    for _i in 0..(n / BUF_SIZE) {
      self.inner.read_exact(tbuf).and_then(|_| {
        self.pos += BUF_SIZE;
        Ok(())
      })?;
    }

    let mut tvec = vec![0; n - self.pos];
    self.inner.read_exact(tvec.as_mut_slice()).and_then(|_| {
      self.pos += tvec.len();
      Ok(())
    })?;

    Ok(self.pos)
  }
}

impl Read for RangeReader {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    if self.end != None && self.pos >= self.end.unwrap() {
      return Ok(0);
    }

    if self.pos == 0 && self.start != None {
      self.advance(self.start.unwrap())?;
    }

    let n = self.inner.read(buf)?;
    self.pos += n;

    if let Some(end) = self.end {
      if self.pos >= end {
        return Ok(end - (self.pos - n));
      }
    }

    Ok(n)
  }
}

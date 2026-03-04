use crate::error::Result;

/// Read bytes from a source.
pub trait Read {
    /// Read some bytes into `buf`, returning the number of bytes read.
    /// Returns `Ok(0)` at end-of-file.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Read exactly enough bytes to fill `buf`.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            let n = self.read(buf)?;
            if n == 0 {
                return Err(crate::error::AmigaError::IoError(0));
            }
            buf = &mut buf[n..];
        }
        Ok(())
    }
}

/// Write bytes to a destination.
pub trait Write {
    /// Write some bytes from `buf`, returning the number written.
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Write all bytes from `buf`.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            let n = self.write(buf)?;
            if n == 0 {
                return Err(crate::error::AmigaError::IoError(0));
            }
            buf = &buf[n..];
        }
        Ok(())
    }

    /// Flush any buffered data. Default is a no-op.
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

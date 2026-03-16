//! Read and Write traits for AmigaOS I/O.
//!
//! These are simplified versions of `std::io::Read` and `std::io::Write`,
//! returning [`AmigaError`](crate::error::AmigaError) instead of `std::io::Error`.
//! Implemented by [`File`](crate::fs::File) and usable with any custom I/O source.

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
                return Err(crate::error::AmigaError::UnexpectedEof);
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
                return Err(crate::error::AmigaError::UnexpectedEof);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AmigaError;
    extern crate alloc;
    use alloc::vec::Vec;

    // ---- Mock reader: yields data from a buffer ----

    struct MockReader {
        data: Vec<u8>,
        pos: usize,
        chunk_size: usize, // max bytes per read call
    }

    impl MockReader {
        fn new(data: &[u8], chunk_size: usize) -> Self {
            Self {
                data: data.to_vec(),
                pos: 0,
                chunk_size,
            }
        }
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let remaining = &self.data[self.pos..];
            if remaining.is_empty() {
                return Ok(0);
            }
            let n = buf.len().min(remaining.len()).min(self.chunk_size);
            buf[..n].copy_from_slice(&remaining[..n]);
            self.pos += n;
            Ok(n)
        }
    }

    // ---- Mock writer: collects data into a buffer ----

    struct MockWriter {
        data: Vec<u8>,
        chunk_size: usize, // max bytes per write call
    }

    impl MockWriter {
        fn new(chunk_size: usize) -> Self {
            Self {
                data: Vec::new(),
                chunk_size,
            }
        }
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            let n = buf.len().min(self.chunk_size);
            self.data.extend_from_slice(&buf[..n]);
            Ok(n)
        }
    }

    // ---- Failing reader/writer ----

    struct FailReader;

    impl Read for FailReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Ok(0) // EOF immediately
        }
    }

    struct FailWriter;

    impl Write for FailWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0) // writes nothing
        }
    }

    // ---- Read tests ----

    #[test]
    fn read_all_at_once() {
        let mut r = MockReader::new(b"hello", 100);
        let mut buf = [0u8; 5];
        let n = r.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");
    }

    #[test]
    fn read_in_chunks() {
        let mut r = MockReader::new(b"hello", 2);
        let mut buf = [0u8; 5];

        let n = r.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf[..2], b"he");

        let n = r.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf[..2], b"ll");

        let n = r.read(&mut buf).unwrap();
        assert_eq!(n, 1);
        assert_eq!(&buf[..1], b"o");

        let n = r.read(&mut buf).unwrap();
        assert_eq!(n, 0); // EOF
    }

    #[test]
    fn read_exact_single_chunk() {
        let mut r = MockReader::new(b"abcde", 100);
        let mut buf = [0u8; 5];
        r.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"abcde");
    }

    #[test]
    fn read_exact_multiple_chunks() {
        let mut r = MockReader::new(b"abcdef", 2);
        let mut buf = [0u8; 6];
        r.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"abcdef");
    }

    #[test]
    fn read_exact_insufficient_data() {
        let mut r = MockReader::new(b"abc", 100);
        let mut buf = [0u8; 5];
        let err = r.read_exact(&mut buf).unwrap_err();
        assert_eq!(err, AmigaError::UnexpectedEof);
    }

    #[test]
    fn read_exact_empty_buf() {
        let mut r = MockReader::new(b"abc", 100);
        let mut buf = [0u8; 0];
        r.read_exact(&mut buf).unwrap(); // should succeed immediately
    }

    #[test]
    fn read_exact_from_fail_reader() {
        let mut r = FailReader;
        let mut buf = [0u8; 1];
        let err = r.read_exact(&mut buf).unwrap_err();
        assert_eq!(err, AmigaError::UnexpectedEof);
    }

    // ---- Write tests ----

    #[test]
    fn write_all_at_once() {
        let mut w = MockWriter::new(100);
        let n = w.write(b"hello").unwrap();
        assert_eq!(n, 5);
        assert_eq!(&w.data, b"hello");
    }

    #[test]
    fn write_in_chunks() {
        let mut w = MockWriter::new(2);
        let n = w.write(b"hello").unwrap();
        assert_eq!(n, 2);
        assert_eq!(&w.data, b"he");
    }

    #[test]
    fn write_all_single_chunk() {
        let mut w = MockWriter::new(100);
        w.write_all(b"hello").unwrap();
        assert_eq!(&w.data, b"hello");
    }

    #[test]
    fn write_all_multiple_chunks() {
        let mut w = MockWriter::new(2);
        w.write_all(b"hello!").unwrap();
        assert_eq!(&w.data, b"hello!");
    }

    #[test]
    fn write_all_empty() {
        let mut w = MockWriter::new(100);
        w.write_all(b"").unwrap();
        assert!(w.data.is_empty());
    }

    #[test]
    fn write_all_fail_writer() {
        let mut w = FailWriter;
        let err = w.write_all(b"x").unwrap_err();
        assert_eq!(err, AmigaError::UnexpectedEof);
    }

    #[test]
    fn flush_default_succeeds() {
        let mut w = MockWriter::new(100);
        w.flush().unwrap();
    }

    // ---- Sprint 3C: additional tests ----

    #[test]
    fn read_exact_zero_buf_on_empty() {
        // read_exact with empty buffer should succeed even on empty reader
        let mut r = MockReader::new(b"", 100);
        let mut buf = [0u8; 0];
        r.read_exact(&mut buf).unwrap();
    }

    #[test]
    fn write_all_single_byte_chunks() {
        // Writer that accepts only 1 byte at a time
        let mut w = MockWriter::new(1);
        let payload = b"0123456789";
        w.write_all(payload).unwrap();
        assert_eq!(&w.data, payload);
    }

    #[test]
    fn read_single_byte_exact() {
        // Reader that yields only 1 byte at a time
        let mut r = MockReader::new(b"0123456789", 1);
        let mut buf = [0u8; 10];
        r.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"0123456789");
    }
}

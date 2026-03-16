//! Host-side integration tests for the amigaos4 crate.
//!
//! These tests exercise pure-Rust logic that does not depend on AmigaOS
//! runtime services. They run on the host (x86_64/aarch64) via `cargo test`.
//!
//! Modules tested:
//! - error: AmigaError Display/Debug formatting, equality, Result alias
//! - time: Duration constructors, conversions, arithmetic (NOT Instant)
//! - tag: TagListBuilder construction, tag_if conditional logic
//! - io: Read/Write traits via mock implementations

use amigaos4::error::AmigaError;
use amigaos4::tag::TagListBuilder;
use amigaos4_sys::{TAG_DONE, TAG_IGNORE, TAG_USER};

// ============================================================
// error module
// ============================================================

#[test]
fn error_display_all_variants() {
    assert_eq!(format!("{}", AmigaError::NullPointer), "null pointer");
    assert_eq!(format!("{}", AmigaError::AllocationFailed), "allocation failed");
    assert_eq!(format!("{}", AmigaError::IoError(13)), "I/O error (errno=13)");
    assert_eq!(format!("{}", AmigaError::DosError(205)), "DOS error (205)");
}

#[test]
fn error_is_copy_and_eq() {
    let a = AmigaError::IoError(5);
    let b = a; // Copy
    assert_eq!(a, b);
}

#[test]
fn result_alias_works() {
    let ok: amigaos4::error::Result<i32> = Ok(42);
    assert_eq!(ok.unwrap(), 42);

    let err: amigaos4::error::Result<i32> = Err(AmigaError::NullPointer);
    assert!(err.is_err());
}

// ============================================================
// time module (Duration only -- Instant needs AmigaOS clock)
// ============================================================

#[cfg(feature = "time")]
mod time_tests {
    use amigaos4::time::Duration;

    #[test]
    fn duration_from_secs_and_back() {
        let d = Duration::from_secs(7);
        assert_eq!(d.as_secs(), 7);
        assert_eq!(d.subsec_nanos(), 0);
    }

    #[test]
    fn duration_from_millis_split() {
        let d = Duration::from_millis(2_345);
        assert_eq!(d.as_secs(), 2);
        assert_eq!(d.as_millis(), 2_345);
    }

    #[test]
    fn duration_add_with_carry() {
        let a = Duration::from_nanos(999_999_999);
        let b = Duration::from_nanos(2);
        let c = a + b;
        assert_eq!(c.as_secs(), 1);
        assert_eq!(c.subsec_nanos(), 1);
    }

    #[test]
    fn duration_sub_saturates() {
        let small = Duration::from_millis(1);
        let big = Duration::from_secs(1);
        let result = small - big;
        assert_eq!(result, Duration::ZERO);
    }
}

// ============================================================
// tag module
// ============================================================

#[test]
fn tag_builder_empty() {
    let tags = TagListBuilder::new().build();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].ti_Tag, TAG_DONE);
}

#[test]
fn tag_builder_conditional() {
    let tags = TagListBuilder::new()
        .tag_if(true, TAG_USER + 10, 0xAA)
        .tag_if(false, TAG_USER + 20, 0xBB)
        .build();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0].ti_Tag, TAG_USER + 10);
    assert_eq!(tags[0].ti_Data, 0xAA);
    assert_eq!(tags[1].ti_Tag, TAG_IGNORE);
    assert_eq!(tags[2].ti_Tag, TAG_DONE);
}

// ============================================================
// io module (via mock implementations)
// ============================================================

mod io_tests {
    use amigaos4::error::{AmigaError, Result};
    use amigaos4::io::{Read, Write};

    struct Cursor {
        data: Vec<u8>,
        pos: usize,
    }

    impl Cursor {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
    }

    impl Read for Cursor {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let remaining = &self.data[self.pos..];
            let n = buf.len().min(remaining.len());
            buf[..n].copy_from_slice(&remaining[..n]);
            self.pos += n;
            Ok(n)
        }
    }

    struct Sink {
        written: Vec<u8>,
    }

    impl Sink {
        fn new() -> Self {
            Self { written: Vec::new() }
        }
    }

    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    #[test]
    fn read_exact_success() {
        let mut c = Cursor::new(b"abcdef".to_vec());
        let mut buf = [0u8; 6];
        c.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"abcdef");
    }

    #[test]
    fn read_exact_eof_error() {
        let mut c = Cursor::new(b"ab".to_vec());
        let mut buf = [0u8; 5];
        let err = c.read_exact(&mut buf).unwrap_err();
        assert_eq!(err, AmigaError::UnexpectedEof);
    }

    #[test]
    fn write_all_success() {
        let mut s = Sink::new();
        s.write_all(b"hello world").unwrap();
        assert_eq!(&s.written, b"hello world");
    }

    #[test]
    fn flush_default_is_ok() {
        let mut s = Sink::new();
        s.flush().unwrap();
    }
}

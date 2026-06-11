//! Null-terminated byte-string helpers.
//!
//! AmigaOS and clib4 APIs take C strings. The safe wrappers in this crate
//! accept `&[u8]` arguments that must carry a trailing `\0`. Two helpers
//! keep that contract honest:
//!
//! - [`amstr!`](crate::amstr) builds a correctly terminated `&'static [u8]`
//!   from a string literal at compile time — prefer it over hand-written
//!   `b"text\0"` literals.
//! - [`require_nul`] validates an arbitrary byte slice at runtime. Every
//!   wrapper in this crate calls it before handing the pointer to the OS,
//!   so a missing terminator is an [`AmigaError::NotNulTerminated`] error,
//!   never an out-of-bounds read.
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::amstr;
//!
//! let resp = amigaos4::http::get(amstr!("example.com"), 80, amstr!("/"))?;
//! ```

use crate::error::{AmigaError, Result};

/// Build a null-terminated `&'static [u8]` from a string literal at
/// compile time.
///
/// ```
/// let s: &'static [u8] = amigaos4::amstr!("RAM:test.txt");
/// assert_eq!(s.last(), Some(&0u8));
/// ```
#[macro_export]
macro_rules! amstr {
    ($s:expr) => {
        concat!($s, "\0").as_bytes()
    };
}

/// Validate that `s` ends with a `\0` and return a pointer suitable for
/// passing to a C API.
///
/// # Errors
///
/// Returns [`AmigaError::NotNulTerminated`] if `s` is empty or its last
/// byte is not `\0`. (Interior `\0` bytes are allowed — C simply stops
/// at the first one.)
#[inline]
pub fn require_nul(s: &[u8]) -> Result<*const u8> {
    if s.last() == Some(&0) {
        Ok(s.as_ptr())
    } else {
        Err(AmigaError::NotNulTerminated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amstr_appends_nul() {
        let s = amstr!("hello");
        assert_eq!(s, b"hello\0");
    }

    #[test]
    fn amstr_empty() {
        let s = amstr!("");
        assert_eq!(s, b"\0");
    }

    #[test]
    fn require_nul_ok() {
        let s = b"RAM:\0";
        assert_eq!(require_nul(s).unwrap(), s.as_ptr());
    }

    #[test]
    fn require_nul_missing_terminator() {
        assert_eq!(
            require_nul(b"RAM:").unwrap_err(),
            AmigaError::NotNulTerminated
        );
    }

    #[test]
    fn require_nul_empty() {
        assert_eq!(
            require_nul(b"").unwrap_err(),
            AmigaError::NotNulTerminated
        );
    }

    #[test]
    fn require_nul_interior_nul_allowed() {
        assert!(require_nul(b"a\0b\0").is_ok());
    }

    #[test]
    fn require_nul_only_nul() {
        assert!(require_nul(b"\0").is_ok());
    }
}

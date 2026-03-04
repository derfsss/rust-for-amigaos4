use core::fmt;

/// Errors returned by safe AmigaOS wrappers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmigaError {
    /// An OS call returned a null pointer unexpectedly.
    NullPointer,
    /// A memory or resource allocation failed.
    AllocationFailed,
    /// A clib4 POSIX call set errno to this value.
    IoError(i32),
    /// An AmigaOS DOS error code (from IoErr()).
    DosError(i32),
}

impl fmt::Display for AmigaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AmigaError::NullPointer => write!(f, "null pointer"),
            AmigaError::AllocationFailed => write!(f, "allocation failed"),
            AmigaError::IoError(e) => write!(f, "I/O error (errno={})", e),
            AmigaError::DosError(e) => write!(f, "DOS error ({})", e),
        }
    }
}

/// Convenience alias used throughout this crate.
pub type Result<T> = core::result::Result<T, AmigaError>;

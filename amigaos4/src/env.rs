use alloc::vec::Vec;
use crate::error::{AmigaError, Result};

extern "C" {
    fn getcwd(buf: *mut u8, size: u32) -> *mut u8;
    fn getenv(name: *const u8) -> *const u8;
}

/// Get the current working directory as a byte vector.
pub fn current_dir() -> Result<Vec<u8>> {
    let mut buf = [0u8; 512];
    let ptr = unsafe { getcwd(buf.as_mut_ptr(), buf.len() as u32) };
    if ptr.is_null() {
        Err(AmigaError::IoError(0))
    } else {
        // Find null terminator
        let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        let mut v = Vec::with_capacity(len);
        v.extend_from_slice(&buf[..len]);
        Ok(v)
    }
}

/// Get an environment variable by name. `name` must be null-terminated.
/// Returns `None` if the variable is not set.
pub fn var(name: &[u8]) -> Option<Vec<u8>> {
    let ptr = unsafe { getenv(name.as_ptr()) };
    if ptr.is_null() {
        None
    } else {
        // Read the C string
        let mut len = 0usize;
        unsafe {
            while *ptr.add(len) != 0 {
                len += 1;
            }
        }
        let mut v = Vec::with_capacity(len);
        unsafe {
            v.extend_from_slice(core::slice::from_raw_parts(ptr, len));
        }
        Some(v)
    }
}

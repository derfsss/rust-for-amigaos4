//! Safe DOS wrappers for AmigaOS 4.1 lock management, directory scanning,
//! and path utilities.
//!
//! [`AmigaLock`] provides RAII lock management via `Lock`/`UnLock`.
//! [`DirScanner`] wraps `ObtainDirContext`/`ExamineDir`/`ReleaseDirContext`
//! for iterating directory contents.
//! Free functions [`file_part`] and [`path_part`] wrap the DOS path utilities.
//!
//! All path arguments must be null-terminated byte slices (convention from Phase 1).

use alloc::vec::Vec;
use crate::error::{AmigaError, Result};
use crate::tag::TagListBuilder;
use amigaos4_sys::{APTR, CONST_STRPTR, ExamineData, STRPTR, TAG_USER, TagItem};

// ---- DOS lock mode constants ----

/// Shared (read) lock mode for `Lock()`.
pub const SHARED_LOCK: i32 = -2;

/// Exclusive (write) lock mode for `Lock()`.
pub const EXCLUSIVE_LOCK: i32 = -1;

// ---- SameLock return values ----

/// Locks refer to the same object.
pub const LOCK_SAME: i32 = 0;

/// Locks are on the same volume but different objects.
pub const LOCK_SAME_VOLUME: i32 = 1;

/// Locks are on different volumes.
pub const LOCK_DIFFERENT: i32 = -1;

// ---- ExamineDir tag constants ----

/// Tag: pass a lock (u32 BPTR) as the directory to scan.
const EX_FILE_LOCK_INPUT: u32 = TAG_USER + 5;

/// Tag: bitmask of which ExamineData fields to populate.
const EX_DATA_FIELDS: u32 = TAG_USER + 2;

/// ExamineData field flag: populate all available fields.
const EXF_ALL: u32 = 0x7F;

// ---- AmigaLock ----

/// RAII wrapper around an AmigaOS DOS lock (`Lock`/`UnLock`).
///
/// The lock handle is a BPTR stored as `u32`. The lock is released
/// automatically when this value is dropped.
pub struct AmigaLock {
    raw: u32,
}

impl AmigaLock {
    /// Obtain a shared (read) lock on `path`. `path` must be null-terminated.
    pub fn shared(path: &[u8]) -> Result<Self> {
        let raw = unsafe {
            amigaos4_sys::dos_lock(path.as_ptr() as CONST_STRPTR, SHARED_LOCK)
        };
        if raw == 0 {
            let err = unsafe { amigaos4_sys::dos_io_err() };
            Err(AmigaError::DosError(err))
        } else {
            Ok(Self { raw })
        }
    }

    /// Obtain an exclusive (write) lock on `path`. `path` must be null-terminated.
    pub fn exclusive(path: &[u8]) -> Result<Self> {
        let raw = unsafe {
            amigaos4_sys::dos_lock(path.as_ptr() as CONST_STRPTR, EXCLUSIVE_LOCK)
        };
        if raw == 0 {
            let err = unsafe { amigaos4_sys::dos_io_err() };
            Err(AmigaError::DosError(err))
        } else {
            Ok(Self { raw })
        }
    }

    /// Get the full path name for this lock via `NameFromLock`.
    ///
    /// Returns the path as a `Vec<u8>` (without trailing null).
    pub fn name(&self) -> Result<Vec<u8>> {
        // Start with a reasonable buffer; AmigaOS paths are typically < 256 bytes.
        let mut buf: Vec<u8> = Vec::with_capacity(256);
        let mut cap = buf.capacity() as i32;

        loop {
            buf.resize(cap as usize, 0);
            let ok = unsafe {
                amigaos4_sys::dos_name_from_lock(
                    self.raw,
                    buf.as_mut_ptr() as STRPTR,
                    cap,
                )
            };
            if ok != 0 {
                // Find the null terminator and truncate.
                let len = buf.iter().position(|&b| b == 0).unwrap_or(cap as usize);
                buf.truncate(len);
                return Ok(buf);
            }
            // Check if the error is ERROR_LINE_TOO_LONG (buffer too small).
            let err = unsafe { amigaos4_sys::dos_io_err() };
            if err == 120 {
                // Double the buffer and retry.
                cap *= 2;
                buf.reserve(cap as usize);
            } else {
                return Err(AmigaError::DosError(err));
            }
        }
    }

    /// Get a lock on the parent directory of this lock.
    pub fn parent(&self) -> Result<AmigaLock> {
        let raw = unsafe { amigaos4_sys::dos_parent_dir(self.raw) };
        if raw == 0 {
            let err = unsafe { amigaos4_sys::dos_io_err() };
            // A zero lock with IoErr()==0 means we're at the root (no parent).
            if err == 0 {
                Err(AmigaError::NullPointer)
            } else {
                Err(AmigaError::DosError(err))
            }
        } else {
            Ok(Self { raw })
        }
    }

    /// Test whether two locks refer to the same object.
    ///
    /// Returns `true` if `SameLock` reports `LOCK_SAME`.
    pub fn same_as(&self, other: &AmigaLock) -> bool {
        let rc = unsafe { amigaos4_sys::dos_same_lock(self.raw, other.raw) };
        rc == LOCK_SAME
    }

    /// Return the raw BPTR lock value.
    #[inline]
    pub fn as_raw(&self) -> u32 {
        self.raw
    }
}

impl Drop for AmigaLock {
    fn drop(&mut self) {
        if self.raw != 0 {
            unsafe { amigaos4_sys::dos_un_lock(self.raw) }
        }
    }
}

// ---- Path utility functions ----

/// Extract the filename component from a path.
///
/// Wraps `dos_file_part`. `path` must be null-terminated.
/// Returns a sub-slice of `path` starting at the filename.
pub fn file_part(path: &[u8]) -> &[u8] {
    let path_ptr = path.as_ptr() as CONST_STRPTR;
    let result = unsafe { amigaos4_sys::dos_file_part(path_ptr) };
    if result.is_null() {
        return path;
    }
    // Verify the returned pointer is within the original buffer.
    let result_addr = result as usize;
    let path_start = path_ptr as usize;
    let path_end = path_start + path.len();
    if result_addr < path_start || result_addr > path_end {
        return &[];
    }
    let offset = result_addr - path_start;
    if offset >= path.len() {
        return &[];
    }
    // Return slice without trailing null if present.
    let tail = &path[offset..];
    match tail.iter().position(|&b| b == 0) {
        Some(end) => &tail[..end],
        None => tail,
    }
}

/// Extract the directory component from a path.
///
/// Wraps `dos_path_part`. `path` must be null-terminated.
/// Returns a sub-slice of `path` up to (but not including) the filename.
pub fn path_part(path: &[u8]) -> &[u8] {
    let path_ptr = path.as_ptr() as CONST_STRPTR;
    let result = unsafe { amigaos4_sys::dos_path_part(path_ptr) };
    if result.is_null() {
        return path;
    }
    let result_addr = result as usize;
    let path_start = path_ptr as usize;
    let path_end = path_start + path.len();
    if result_addr < path_start || result_addr > path_end {
        return path;
    }
    let end = result_addr - path_start;
    if end > path.len() {
        return path;
    }
    &path[..end]
}

// ---- DirScanner ----

/// RAII directory scanner wrapping `ObtainDirContext`/`ExamineDir`/`ReleaseDirContext`.
///
/// Call [`next`](DirScanner::next) repeatedly to get raw `*mut ExamineData`
/// pointers for each directory entry. Since `ExamineData` is opaque in our
/// bindings, field access requires `unsafe` code with known struct offsets.
///
/// The scanner is automatically released on drop.
pub struct DirScanner {
    ctx: APTR,
    /// Keep the lock alive for the lifetime of the context.
    _lock: AmigaLock,
    /// Keep the tag list alive for the lifetime of the context.
    _tags: Vec<TagItem>,
}

impl DirScanner {
    /// Open a directory for scanning. `path` must be null-terminated.
    pub fn open(path: &[u8]) -> Result<Self> {
        let lock = AmigaLock::shared(path)?;

        let tags = TagListBuilder::new()
            .tag(EX_FILE_LOCK_INPUT, lock.as_raw())
            .tag(EX_DATA_FIELDS, EXF_ALL)
            .build();

        let ctx = unsafe { amigaos4_sys::dos_obtain_dir_context(tags.as_ptr()) };
        if ctx.is_null() {
            let err = unsafe { amigaos4_sys::dos_io_err() };
            return Err(AmigaError::DosError(err));
        }

        Ok(Self {
            ctx,
            _lock: lock,
            _tags: tags,
        })
    }

    /// Get the next directory entry, or `None` when the listing is exhausted.
    ///
    /// Returns a raw `*mut ExamineData` pointer. The pointer is valid until
    /// the next call to `next()` or until this `DirScanner` is dropped.
    pub fn next(&mut self) -> Option<*mut ExamineData> {
        let ed = unsafe { amigaos4_sys::dos_examine_dir(self.ctx) };
        if ed.is_null() {
            None
        } else {
            Some(ed)
        }
    }
}

impl Drop for DirScanner {
    fn drop(&mut self) {
        if !self.ctx.is_null() {
            unsafe { amigaos4_sys::dos_release_dir_context(self.ctx) }
        }
    }
}

#[cfg(test)]
mod tests {
    // Path utility tests operate on byte slices and don't need the OS,
    // but file_part/path_part call into DOS FFI which is unavailable
    // in a host test environment. We test only pure-Rust logic here.

    use super::*;

    #[test]
    fn lock_same_constants() {
        assert_eq!(SHARED_LOCK, -2);
        assert_eq!(EXCLUSIVE_LOCK, -1);
    }

    #[test]
    fn same_lock_constants() {
        assert_eq!(LOCK_SAME, 0);
        assert_eq!(LOCK_SAME_VOLUME, 1);
        assert_eq!(LOCK_DIFFERENT, -1);
    }

    #[test]
    fn examine_tag_constants() {
        // Verify the tag values follow TAG_USER convention.
        assert_eq!(EX_FILE_LOCK_INPUT, TAG_USER + 5);
        assert_eq!(EX_DATA_FIELDS, TAG_USER + 2);
        assert_eq!(EXF_ALL, 0x7F);
    }

    #[test]
    fn amiga_lock_struct_is_u32() {
        assert_eq!(core::mem::size_of::<AmigaLock>(), core::mem::size_of::<u32>());
    }

    #[test]
    fn lock_shared_is_neg2() {
        assert_eq!(SHARED_LOCK, -2);
    }

    #[test]
    fn lock_exclusive_is_neg1() {
        assert_eq!(EXCLUSIVE_LOCK, -1);
    }

    #[test]
    fn lock_same_constants_values() {
        assert_eq!(LOCK_SAME, 0);
        assert_eq!(LOCK_SAME_VOLUME, 1);
        assert_eq!(LOCK_DIFFERENT, -1);
    }
}

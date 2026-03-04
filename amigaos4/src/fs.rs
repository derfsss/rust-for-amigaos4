use alloc::vec::Vec;
use crate::error::{AmigaError, Result};
use crate::io;

// clib4 POSIX FFI
extern "C" {
    fn open(path: *const u8, flags: i32, ...) -> i32;
    fn close(fd: i32) -> i32;
    fn read(fd: i32, buf: *mut u8, count: u32) -> i32;
    fn write(fd: i32, buf: *const u8, count: u32) -> i32;
    fn lseek(fd: i32, offset: i32, whence: i32) -> i32;
    fn stat(path: *const u8, buf: *mut Stat) -> i32;
    fn unlink(path: *const u8) -> i32;
    fn mkdir(path: *const u8, mode: u32) -> i32;
    fn __errno() -> *mut i32;
}

fn errno() -> i32 {
    unsafe { *__errno() }
}

// clib4 fcntl.h constants
const O_RDONLY: i32 = 0x0;
const O_WRONLY: i32 = 0x1;
const O_RDWR: i32 = 0x2;
const O_CREAT: i32 = 1 << 3;
const O_TRUNC: i32 = 1 << 5;

// stat mode constants
const S_IFMT: u32 = 0o170000;
const S_IFDIR: u32 = 0o040000;

// lseek whence
const SEEK_SET: i32 = 0;
const SEEK_END: i32 = 2;

/// Layout of clib4's `struct stat` on PPC AmigaOS.
#[repr(C)]
struct Stat {
    st_dev: u32,      // dev_t = unsigned long
    st_ino: u32,      // ino_t = unsigned int
    st_mode: u32,     // mode_t = unsigned int
    st_nlink: u32,    // nlink_t = unsigned int
    st_uid: u32,      // uid_t = unsigned int
    st_gid: u32,      // gid_t = unsigned int
    st_rdev: u32,     // dev_t = unsigned long
    st_size: i64,     // off_t = int64_t
    st_atime: i32,    // time_t = long (32-bit on PPC)
    st_spare1: i32,
    st_mtime: i32,    // time_t
    st_spare2: i32,
    st_ctime: i32,    // time_t
    st_spare3: i32,
    st_blksize: i32,
    st_blocks: i32,
    st_spare4: [i32; 2],
}

/// RAII file handle backed by clib4 POSIX open/close.
pub struct File {
    fd: i32,
}

impl File {
    /// Open a file for reading. `path` must be null-terminated.
    pub fn open(path: &[u8]) -> Result<Self> {
        let fd = unsafe { open(path.as_ptr(), O_RDONLY) };
        if fd < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(Self { fd })
        }
    }

    /// Create (or truncate) a file for writing. `path` must be null-terminated.
    pub fn create(path: &[u8]) -> Result<Self> {
        let fd = unsafe { open(path.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, 0o644u32) };
        if fd < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(Self { fd })
        }
    }

    /// Open a file for reading and writing. `path` must be null-terminated.
    pub fn open_rw(path: &[u8]) -> Result<Self> {
        let fd = unsafe { open(path.as_ptr(), O_RDWR) };
        if fd < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(Self { fd })
        }
    }

    /// Return the raw file descriptor.
    #[inline]
    pub fn as_raw_fd(&self) -> i32 {
        self.fd
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = unsafe { read(self.fd, buf.as_mut_ptr(), buf.len() as u32) };
        if n < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(n as usize)
        }
    }
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = unsafe { write(self.fd, buf.as_ptr(), buf.len() as u32) };
        if n < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(n as usize)
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(()) // clib4 doesn't buffer at fd level
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.fd >= 0 {
            unsafe { close(self.fd); }
        }
    }
}

/// Read an entire file into a Vec. `path` must be null-terminated.
pub fn read_to_vec(path: &[u8]) -> Result<Vec<u8>> {
    use crate::io::Read;
    // Get file size via stat
    let size = metadata(path)?.size() as usize;
    let mut file = File::open(path)?;
    let mut buf = Vec::with_capacity(size);
    // Safety: we'll read into the spare capacity
    unsafe { buf.set_len(size); }
    let mut total = 0;
    while total < size {
        let n = file.read(&mut buf[total..])?;
        if n == 0 { break; }
        total += n;
    }
    buf.truncate(total);
    Ok(buf)
}

/// Write `data` to a file, creating or truncating it. `path` must be null-terminated.
pub fn write_file(path: &[u8], data: &[u8]) -> Result<()> {
    use crate::io::Write;
    let mut file = File::create(path)?;
    file.write_all(data)
}

/// Remove a file. `path` must be null-terminated.
pub fn remove_file(path: &[u8]) -> Result<()> {
    let rc = unsafe { unlink(path.as_ptr()) };
    if rc < 0 {
        Err(AmigaError::IoError(errno()))
    } else {
        Ok(())
    }
}

/// Create a directory. `path` must be null-terminated.
pub fn create_dir(path: &[u8]) -> Result<()> {
    let rc = unsafe { mkdir(path.as_ptr(), 0o755) };
    if rc < 0 {
        Err(AmigaError::IoError(errno()))
    } else {
        Ok(())
    }
}

/// File metadata from stat().
pub struct Metadata {
    inner: Stat,
}

impl Metadata {
    /// File size in bytes.
    #[inline]
    pub fn size(&self) -> i64 {
        self.inner.st_size
    }

    /// True if this is a directory.
    #[inline]
    pub fn is_dir(&self) -> bool {
        (self.inner.st_mode & S_IFMT) == S_IFDIR
    }

    /// Last modification time as Unix timestamp.
    #[inline]
    pub fn modified(&self) -> i32 {
        self.inner.st_mtime
    }
}

/// Query file metadata. `path` must be null-terminated.
pub fn metadata(path: &[u8]) -> Result<Metadata> {
    unsafe {
        let mut st: Stat = core::mem::zeroed();
        let rc = stat(path.as_ptr(), &mut st);
        if rc < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(Metadata { inner: st })
        }
    }
}

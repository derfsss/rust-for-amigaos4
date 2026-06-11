//! ASL file requester — the standard AmigaOS file open/save dialog.
//!
//! [`FileRequester`] is a builder over asl.library's `AllocAslRequest` /
//! `AslRequest` / `FreeAslRequest`. The library is opened at runtime, so
//! this works in every build mode and needs no `-lauto` coverage.
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::asl::{FileRequester, FileMode};
//! use amigaos4::amstr;
//!
//! let pick = FileRequester::new(FileMode::Open)
//!     .title(amstr!("Choose a picture"))
//!     .initial_drawer(amstr!("RAM:"))
//!     .pattern(amstr!("#?.iff"))
//!     .show()?;
//!
//! if let Some(sel) = pick {
//!     // sel.path is "drawer/file" joined the Amiga way
//! }
//! ```

use alloc::vec::Vec;
use crate::error::{AmigaError, Result};
use crate::iface::OpenedInterface;
use crate::tag::TagListBuilder;
use amigaos4_sys::{APTR, AslIFace, TagItem};

// ---------------------------------------------------------------------------
// libraries/asl.h constants (AmigaOS 4.1 SDK 54.16)
// ---------------------------------------------------------------------------

const TAG_USER: u32 = amigaos4_sys::TAG_USER;

/// `ASL_Dummy = TAG_USER + 0x80000` — base for all ASL tags.
const ASL_DUMMY: u32 = TAG_USER + 0x0008_0000;

/// Requester type passed to `AllocAslRequest`: file requester.
const ASL_FILE_REQUEST: u32 = 0;

/// Title text (CONST_STRPTR).
const ASLFR_TITLE_TEXT: u32 = ASL_DUMMY + 1;
/// Initial contents of the File gadget (CONST_STRPTR).
const ASLFR_INITIAL_FILE: u32 = ASL_DUMMY + 8;
/// Initial contents of the Drawer gadget (CONST_STRPTR).
const ASLFR_INITIAL_DRAWER: u32 = ASL_DUMMY + 9;
/// Initial contents of the Pattern gadget (CONST_STRPTR).
const ASLFR_INITIAL_PATTERN: u32 = ASL_DUMMY + 10;
/// Save-mode requester (BOOL): different layout, confirms overwrite.
const ASLFR_DO_SAVE_MODE: u32 = ASL_DUMMY + 44;
/// Show the Pattern gadget (BOOL).
const ASLFR_DO_PATTERNS: u32 = ASL_DUMMY + 46;
/// Only show (and pick) drawers (BOOL).
const ASLFR_DRAWERS_ONLY: u32 = ASL_DUMMY + 47;

// struct FileRequester result-field offsets (PPC 32-bit):
//   uint8  fr_Reserved0[4];   //  0
//   STRPTR fr_File;           //  4
//   STRPTR fr_Drawer;         //  8
/// Offset of `fr_File` within `struct FileRequester`.
const FR_FILE_OFFSET: usize = 4;
/// Offset of `fr_Drawer` within `struct FileRequester`.
const FR_DRAWER_OFFSET: usize = 8;

/// Safety cap when reading the returned path strings.
const MAX_PATH_LEN: usize = 4096;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// What kind of file requester to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    /// Pick an existing file ("Open...").
    Open,
    /// Pick a file name to write ("Save As..." — confirms overwrite).
    Save,
    /// Pick a drawer (directory) only.
    Drawer,
}

/// The user's selection from a [`FileRequester`].
pub struct FileSelection {
    /// Contents of the Drawer gadget (no trailing null).
    pub drawer: Vec<u8>,
    /// Contents of the File gadget (no trailing null). Empty in
    /// [`FileMode::Drawer`].
    pub file: Vec<u8>,
    /// `drawer` and `file` joined with Amiga path rules — ready to pass
    /// (null-terminated) to `fs`/`dos` functions.
    pub path: Vec<u8>,
}

/// Builder for an ASL file requester.
pub struct FileRequester {
    mode: FileMode,
    title: Option<Vec<u8>>,
    initial_drawer: Option<Vec<u8>>,
    initial_file: Option<Vec<u8>>,
    pattern: Option<Vec<u8>>,
    /// First builder argument that was missing its `\0`, surfaced in show().
    invalid: bool,
}

impl FileRequester {
    /// Start building a requester of the given mode.
    pub fn new(mode: FileMode) -> Self {
        Self {
            mode,
            title: None,
            initial_drawer: None,
            initial_file: None,
            pattern: None,
            invalid: false,
        }
    }

    /// Window title. Must be null-terminated (use [`amstr!`](crate::amstr)).
    pub fn title(mut self, text: &[u8]) -> Self {
        self.set(text, |s, v| s.title = Some(v));
        self
    }

    /// Initial drawer (directory). Must be null-terminated.
    pub fn initial_drawer(mut self, drawer: &[u8]) -> Self {
        self.set(drawer, |s, v| s.initial_drawer = Some(v));
        self
    }

    /// Initial file name. Must be null-terminated.
    pub fn initial_file(mut self, file: &[u8]) -> Self {
        self.set(file, |s, v| s.initial_file = Some(v));
        self
    }

    /// Show a pattern gadget with this initial pattern (e.g.
    /// `amstr!("#?.txt")`). Must be null-terminated.
    pub fn pattern(mut self, pattern: &[u8]) -> Self {
        self.set(pattern, |s, v| s.pattern = Some(v));
        self
    }

    fn set(&mut self, value: &[u8], store: impl FnOnce(&mut Self, Vec<u8>)) {
        if value.last() == Some(&0) {
            store(self, value.to_vec());
        } else {
            self.invalid = true;
        }
    }

    /// Open asl.library, show the requester, and wait for the user.
    ///
    /// Returns `Ok(Some(selection))` if the user confirmed, `Ok(None)`
    /// if they cancelled.
    ///
    /// # Errors
    ///
    /// `NotNulTerminated` if a builder argument was missing its `\0`;
    /// `NullPointer` if asl.library cannot be opened;
    /// `AllocationFailed` if the requester cannot be allocated.
    pub fn show(self) -> Result<Option<FileSelection>> {
        if self.invalid {
            return Err(AmigaError::NotNulTerminated);
        }

        let asl = OpenedInterface::open(b"asl.library\0", 50)?;
        let iasl = asl.as_ptr() as *mut AslIFace;

        let mut tags = TagListBuilder::new();
        if let Some(ref t) = self.title {
            tags = tags.tag(ASLFR_TITLE_TEXT, t.as_ptr() as u32);
        }
        if let Some(ref d) = self.initial_drawer {
            tags = tags.tag(ASLFR_INITIAL_DRAWER, d.as_ptr() as u32);
        }
        if let Some(ref f) = self.initial_file {
            tags = tags.tag(ASLFR_INITIAL_FILE, f.as_ptr() as u32);
        }
        if let Some(ref p) = self.pattern {
            tags = tags
                .tag(ASLFR_DO_PATTERNS, 1)
                .tag(ASLFR_INITIAL_PATTERN, p.as_ptr() as u32);
        }
        match self.mode {
            FileMode::Open => {}
            FileMode::Save => tags = tags.tag(ASLFR_DO_SAVE_MODE, 1),
            FileMode::Drawer => tags = tags.tag(ASLFR_DRAWERS_ONLY, 1),
        }
        let tags: Vec<TagItem> = tags.build();

        // SAFETY: iasl is a valid AslIFace obtained above; tags is
        // TAG_DONE-terminated; the title/drawer/file/pattern buffers
        // outlive the requester (owned by self until end of scope).
        let req: APTR = unsafe {
            ((*iasl).AllocAslRequest)(iasl, ASL_FILE_REQUEST, tags.as_ptr())
        };
        if req.is_null() {
            return Err(AmigaError::AllocationFailed);
        }

        // SAFETY: req was allocated above; null tag list = no overrides.
        // AslRequest blocks until the user confirms or cancels.
        let ok = unsafe { ((*iasl).AslRequest)(iasl, req, core::ptr::null()) };

        let result = if ok != 0 {
            // SAFETY: on success fr_File/fr_Drawer are valid C strings
            // (possibly empty) owned by the requester; read them before
            // FreeAslRequest below.
            let (file, drawer) = unsafe {
                let base = req as *const u8;
                let file_ptr = *(base.add(FR_FILE_OFFSET) as *const *const u8);
                let drawer_ptr = *(base.add(FR_DRAWER_OFFSET) as *const *const u8);
                (read_c_string(file_ptr), read_c_string(drawer_ptr))
            };
            let path = join_path(&drawer, &file);
            Some(FileSelection { drawer, file, path })
        } else {
            None
        };

        // SAFETY: req came from AllocAslRequest and is freed exactly once.
        unsafe { ((*iasl).FreeAslRequest)(iasl, req) };
        // asl.library closes when `asl` drops.

        Ok(result)
    }
}

/// Copy a C string (null pointer → empty), capped at [`MAX_PATH_LEN`].
unsafe fn read_c_string(ptr: *const u8) -> Vec<u8> {
    let mut v = Vec::new();
    if ptr.is_null() {
        return v;
    }
    let mut len = 0usize;
    // SAFETY (caller): ptr is a valid null-terminated C string; the cap
    // bounds the scan if the terminator is missing.
    unsafe {
        while len < MAX_PATH_LEN && *ptr.add(len) != 0 {
            len += 1;
        }
        v.extend_from_slice(core::slice::from_raw_parts(ptr, len));
    }
    v
}

/// Join an Amiga drawer and file name (like dos.library `AddPart`):
/// no separator after `:` or `/` or an empty drawer; `/` otherwise.
/// The result is null-terminated, ready for `fs`/`dos` calls.
pub fn join_path(drawer: &[u8], file: &[u8]) -> Vec<u8> {
    let mut path = Vec::with_capacity(drawer.len() + file.len() + 2);
    path.extend_from_slice(drawer);
    match path.last() {
        None | Some(&b':') | Some(&b'/') => {}
        Some(_) => path.push(b'/'),
    }
    path.extend_from_slice(file);
    path.push(0);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_plain_drawer() {
        assert_eq!(join_path(b"Work:Pics", b"a.iff"), b"Work:Pics/a.iff\0");
    }

    #[test]
    fn join_volume_root() {
        assert_eq!(join_path(b"RAM:", b"x"), b"RAM:x\0");
    }

    #[test]
    fn join_trailing_slash() {
        assert_eq!(join_path(b"Work:a/", b"x"), b"Work:a/x\0");
    }

    #[test]
    fn join_empty_drawer() {
        assert_eq!(join_path(b"", b"file"), b"file\0");
    }

    #[test]
    fn join_empty_file() {
        assert_eq!(join_path(b"RAM:T", b""), b"RAM:T/\0");
    }

    #[test]
    fn builder_rejects_unterminated() {
        let r = FileRequester::new(FileMode::Open).title(b"no nul");
        assert!(r.invalid);
    }

    #[test]
    fn asl_tag_values() {
        // libraries/asl.h: ASL_Dummy = TAG_USER + 0x80000.
        assert_eq!(ASL_DUMMY, TAG_USER + 0x80000);
        assert_eq!(ASLFR_TITLE_TEXT, ASL_DUMMY + 1);
        assert_eq!(ASLFR_INITIAL_FILE, ASL_DUMMY + 8);
        assert_eq!(ASLFR_INITIAL_DRAWER, ASL_DUMMY + 9);
        assert_eq!(ASLFR_DO_SAVE_MODE, ASL_DUMMY + 44);
        assert_eq!(ASLFR_DRAWERS_ONLY, ASL_DUMMY + 47);
    }
}

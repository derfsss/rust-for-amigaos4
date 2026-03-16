//! Clipboard access via IFFParse for reading and writing text.
//!
//! [`read_text`] and [`write_text`] provide clipboard access using the
//! IFFParse library's clipboard support. Text is stored in the standard
//! FTXT/CHRS IFF format used by all AmigaOS applications.
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::clipboard;
//!
//! // Write text to clipboard
//! clipboard::write_text(b"Hello from Rust!")?;
//!
//! // Read text from clipboard
//! let text = clipboard::read_text()?;
//! ```

use alloc::vec::Vec;
use crate::error::{AmigaError, Result};
use amigaos4_sys::{APTR, CONST_APTR, IFFHandle};

// ---------------------------------------------------------------------------
// IFF constants for FTXT clipboard format
// ---------------------------------------------------------------------------

/// IFF type identifier: `FTXT` (formatted text).
const ID_FTXT: i32 = make_id(b'F', b'T', b'X', b'T');

/// IFF chunk identifier: `CHRS` (character string).
const ID_CHRS: i32 = make_id(b'C', b'H', b'R', b'S');

/// IFF top-level container: `FORM`.
const ID_FORM: i32 = make_id(b'F', b'O', b'R', b'M');

/// IFF size unknown — let IFFParse compute it.
const IFFSIZE_UNKNOWN: i32 = -1;

/// IFF open mode: read.
const IFFF_READ: i32 = 0;

/// IFF open mode: write.
const IFFF_WRITE: i32 = 1;

/// IFFParse control mode: scan for matching chunks.
const IFFPARSE_SCAN: i32 = 0;

/// IFFParse error: reached end of file.
const IFFERR_EOF: i32 = -1;

/// Primary clipboard unit (unit 0).
const PRIMARY_CLIP: i32 = 0;

/// Offset of `iff_Stream` field in IFFHandle (first field, offset 0).
const IFF_STREAM_OFFSET: usize = 0;

/// Maximum text size to read from clipboard (safety cap).
const MAX_CLIPBOARD_TEXT: usize = 1024 * 1024; // 1 MB

/// Compile-time IFF ID construction.
const fn make_id(a: u8, b: u8, c: u8, d: u8) -> i32 {
    ((a as i32) << 24) | ((b as i32) << 16) | ((c as i32) << 8) | (d as i32)
}

// ---------------------------------------------------------------------------
// Internal cleanup helper
// ---------------------------------------------------------------------------

/// Release IFFParse resources in the correct order.
///
/// `iff_opened` must be true only if `iffparse_open_iff` succeeded.
/// This prevents calling `close_iff` on an unopened handle.
unsafe fn cleanup(
    iff: *mut IFFHandle,
    clip: *mut amigaos4_sys::ClipboardHandle,
    iff_opened: bool,
) {
    if iff_opened {
        amigaos4_sys::iffparse_close_iff(iff);
    }
    amigaos4_sys::iffparse_free_iff(iff);
    amigaos4_sys::iffparse_close_clipboard(clip);
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Read text from the system clipboard.
///
/// Opens the primary clipboard unit, parses the IFF FTXT/CHRS structure,
/// and returns the text content as a byte vector. Returns an empty vector
/// if the clipboard contains no FTXT data.
///
/// # Errors
///
/// Returns `NullPointer` if the clipboard cannot be opened.
/// Returns `AllocationFailed` if the IFF handle cannot be allocated.
/// Returns `IoError` if the IFF stream cannot be opened or parsed.
pub fn read_text() -> Result<Vec<u8>> {
    // 1. Open clipboard unit
    // SAFETY: OpenClipboard returns a ClipboardHandle or null on failure.
    let clip = unsafe { amigaos4_sys::iffparse_open_clipboard(PRIMARY_CLIP) };
    if clip.is_null() {
        return Err(AmigaError::NullPointer);
    }

    // 2. Allocate IFF handle
    // SAFETY: AllocIFF returns a new IFFHandle or null on failure.
    let iff = unsafe { amigaos4_sys::iffparse_alloc_iff() };
    if iff.is_null() {
        unsafe { amigaos4_sys::iffparse_close_clipboard(clip) };
        return Err(AmigaError::AllocationFailed);
    }

    // 3. Set iff_Stream to the clipboard handle and initialize
    // SAFETY: iff_Stream is the first field (offset 0) of IFFHandle.
    // We write the ClipboardHandle pointer as a u32 (PPC 32-bit convention).
    // InitIFFasClip is a void function that sets up clipboard stream hooks.
    unsafe {
        let stream_ptr = (iff as *mut u8).add(IFF_STREAM_OFFSET) as *mut u32;
        *stream_ptr = clip as u32;
        amigaos4_sys::iffparse_init_iffas_clip(iff);
    }

    // 4. Open for reading
    let rc = unsafe { amigaos4_sys::iffparse_open_iff(iff, IFFF_READ) };
    if rc != 0 {
        // SAFETY: open_iff failed, so we must NOT call close_iff.
        unsafe { cleanup(iff, clip, false) };
        return Err(AmigaError::IoError(rc));
    }

    // 5. Register interest in FTXT/CHRS chunks
    // SAFETY: iff is a valid, opened IFF handle.
    let rc = unsafe { amigaos4_sys::iffparse_stop_chunk(iff, ID_FTXT, ID_CHRS) };
    if rc != 0 {
        unsafe { cleanup(iff, clip, true) };
        return Err(AmigaError::IoError(rc));
    }

    // 6. Parse until we find the CHRS chunk (or EOF)
    let mut result = Vec::new();
    loop {
        let rc = unsafe { amigaos4_sys::iffparse_parse_iff(iff, IFFPARSE_SCAN) };
        if rc == IFFERR_EOF {
            break; // No more data
        }
        if rc != 0 {
            break; // End of context or error — stop parsing
        }

        // Read the CHRS chunk data
        let mut buf = [0u8; 4096];
        loop {
            // SAFETY: buf is a valid stack buffer; ReadChunkBytes reads
            // at most buf.len() bytes into it.
            let n = unsafe {
                amigaos4_sys::iffparse_read_chunk_bytes(
                    iff,
                    buf.as_mut_ptr() as APTR,
                    buf.len() as i32,
                )
            };
            if n <= 0 {
                break; // End of chunk or error
            }
            let n = n as usize;
            if result.len() + n > MAX_CLIPBOARD_TEXT {
                let remaining = MAX_CLIPBOARD_TEXT - result.len();
                result.extend_from_slice(&buf[..remaining]);
                break;
            }
            result.extend_from_slice(&buf[..n]);
        }
    }

    // 7. Cleanup (iff was successfully opened)
    unsafe { cleanup(iff, clip, true) };

    Ok(result)
}

/// Write text to the system clipboard.
///
/// Opens the primary clipboard unit and writes the data as an IFF
/// FTXT/CHRS structure, replacing any existing clipboard content.
///
/// # Errors
///
/// Returns `NullPointer` if the clipboard cannot be opened.
/// Returns `AllocationFailed` if the IFF handle cannot be allocated.
/// Returns `IoError` if the IFF stream cannot be opened or written.
pub fn write_text(data: &[u8]) -> Result<()> {
    // 1. Open clipboard unit
    // SAFETY: OpenClipboard returns a ClipboardHandle or null on failure.
    let clip = unsafe { amigaos4_sys::iffparse_open_clipboard(PRIMARY_CLIP) };
    if clip.is_null() {
        return Err(AmigaError::NullPointer);
    }

    // 2. Allocate IFF handle
    // SAFETY: AllocIFF returns a new IFFHandle or null on failure.
    let iff = unsafe { amigaos4_sys::iffparse_alloc_iff() };
    if iff.is_null() {
        unsafe { amigaos4_sys::iffparse_close_clipboard(clip) };
        return Err(AmigaError::AllocationFailed);
    }

    // 3. Set iff_Stream and initialize clipboard hooks
    // SAFETY: iff_Stream is the first field (offset 0) of IFFHandle.
    // InitIFFasClip is void — sets up clipboard stream hooks.
    unsafe {
        let stream_ptr = (iff as *mut u8).add(IFF_STREAM_OFFSET) as *mut u32;
        *stream_ptr = clip as u32;
        amigaos4_sys::iffparse_init_iffas_clip(iff);
    }

    // 4. Open for writing
    let rc = unsafe { amigaos4_sys::iffparse_open_iff(iff, IFFF_WRITE) };
    if rc != 0 {
        // SAFETY: open_iff failed, so we must NOT call close_iff.
        unsafe { cleanup(iff, clip, false) };
        return Err(AmigaError::IoError(rc));
    }

    // 5. Push FORM FTXT container
    // SAFETY: iff is a valid, opened IFF handle.
    let err = unsafe {
        amigaos4_sys::iffparse_push_chunk(iff, ID_FTXT, ID_FORM, IFFSIZE_UNKNOWN)
    };
    if err != 0 {
        unsafe { cleanup(iff, clip, true) };
        return Err(AmigaError::IoError(err));
    }

    // 6. Push CHRS chunk
    let err = unsafe {
        amigaos4_sys::iffparse_push_chunk(iff, 0, ID_CHRS, IFFSIZE_UNKNOWN)
    };
    if err != 0 {
        unsafe { cleanup(iff, clip, true) };
        return Err(AmigaError::IoError(err));
    }

    // 7. Write the text data
    // SAFETY: data.as_ptr() and data.len() describe a valid byte slice.
    let written = unsafe {
        amigaos4_sys::iffparse_write_chunk_bytes(
            iff,
            data.as_ptr() as CONST_APTR,
            data.len() as i32,
        )
    };
    if written != data.len() as i32 {
        unsafe { cleanup(iff, clip, true) };
        return Err(AmigaError::IoError(-6)); // IFFERR_WRITE
    }

    // 8. Pop CHRS chunk
    let err = unsafe { amigaos4_sys::iffparse_pop_chunk(iff) };
    if err != 0 {
        unsafe { cleanup(iff, clip, true) };
        return Err(AmigaError::IoError(err));
    }

    // 9. Pop FORM FTXT
    let err = unsafe { amigaos4_sys::iffparse_pop_chunk(iff) };
    if err != 0 {
        unsafe { cleanup(iff, clip, true) };
        return Err(AmigaError::IoError(err));
    }

    // 10. Cleanup (iff was successfully opened)
    unsafe { cleanup(iff, clip, true) };

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_ftxt_value() {
        assert_eq!(ID_FTXT, make_id(b'F', b'T', b'X', b'T'));
        // FTXT = 0x46545854
        assert_eq!(ID_FTXT, 0x46545854_u32 as i32);
    }

    #[test]
    fn id_chrs_value() {
        assert_eq!(ID_CHRS, make_id(b'C', b'H', b'R', b'S'));
    }

    #[test]
    fn id_form_value() {
        assert_eq!(ID_FORM, make_id(b'F', b'O', b'R', b'M'));
    }

    #[test]
    fn make_id_roundtrip() {
        let id = make_id(b'T', b'E', b'S', b'T');
        let bytes = id.to_be_bytes();
        assert_eq!(&bytes, b"TEST");
    }

    #[test]
    fn iff_constants() {
        assert_eq!(IFFF_READ, 0);
        assert_eq!(IFFF_WRITE, 1);
        assert_eq!(IFFPARSE_SCAN, 0);
        assert_eq!(IFFERR_EOF, -1);
    }

    #[test]
    fn primary_clip_is_zero() {
        assert_eq!(PRIMARY_CLIP, 0);
    }

    #[test]
    fn max_clipboard_text_reasonable() {
        assert_eq!(MAX_CLIPBOARD_TEXT, 1024 * 1024);
    }
}

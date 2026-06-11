//! Simple requester dialogs via `EasyRequestArgs`.
//!
//! This module provides a safe wrapper around Intuition's `EasyRequestArgs`,
//! which displays a modal dialog with a title, body text, and one or more
//! gadget buttons.
//!
//! # Example
//! ```ignore
//! use amigaos4::requester::easy_request;
//!
//! // Show a two-button requester. Returns 1 for "OK", 0 for "Cancel".
//! let choice = easy_request(
//!     b"Warning\0",
//!     b"Are you sure you want to continue?\0",
//!     b"OK|Cancel\0",
//! );
//! if choice == 1 {
//!     // user clicked OK
//! }
//! ```

use amigaos4_sys::{APTR, CONST_STRPTR, Window};

/// Layout of the AmigaOS `EasyStruct` used by `EasyRequestArgs`.
///
/// ```c
/// struct EasyStruct {
///     ULONG       es_StructSize;
///     ULONG       es_Flags;
///     CONST_STRPTR es_Title;
///     CONST_STRPTR es_TextFormat;
///     CONST_STRPTR es_GadgetFormat;
/// };
/// ```
#[repr(C)]
struct EasyStructData {
    es_struct_size: u32,
    es_flags: u32,
    es_title: CONST_STRPTR,
    es_text_format: CONST_STRPTR,
    es_gadget_format: CONST_STRPTR,
}

/// Show a simple requester dialog on the default public screen.
///
/// Returns the gadget number the user clicked:
/// - `1` for the leftmost gadget
/// - `0` for the rightmost gadget (typically "Cancel")
/// - Higher numbers for gadgets in between, counting left to right
///
/// All three string arguments must be null-terminated byte strings.
///
/// # Panics
/// Does not panic, but returns `-1` if the requester could not be created
/// (e.g. out of memory).
pub fn easy_request(title: &[u8], body: &[u8], gadgets: &[u8]) -> i32 {
    easy_request_on(core::ptr::null_mut(), title, body, gadgets)
}

/// Show a simple requester dialog relative to a specific window.
///
/// If `window` is null, the requester appears on the default public screen.
/// See [`easy_request`] for return values and string requirements.
/// Returns `-1` (the could-not-create code) if any string argument is
/// missing its `\0` terminator.
pub fn easy_request_on(window: *mut Window, title: &[u8], body: &[u8], gadgets: &[u8]) -> i32 {
    if title.last() != Some(&0) || body.last() != Some(&0) || gadgets.last() != Some(&0) {
        return -1;
    }
    let es = EasyStructData {
        es_struct_size: core::mem::size_of::<EasyStructData>() as u32,
        es_flags: 0,
        es_title: title.as_ptr() as CONST_STRPTR,
        es_text_format: body.as_ptr() as CONST_STRPTR,
        es_gadget_format: gadgets.as_ptr() as CONST_STRPTR,
    };
    unsafe {
        amigaos4_sys::intuition_easy_request_args(
            window,
            &es as *const EasyStructData as *const amigaos4_sys::EasyStruct,
            core::ptr::null_mut(),
            core::ptr::null_mut() as APTR,
        )
    }
}

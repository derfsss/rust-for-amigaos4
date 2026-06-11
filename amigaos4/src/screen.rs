//! RAII public screen locking and DrawInfo access.
//!
//! [`PubScreen`] wraps `LockPubScreen`/`UnlockPubScreen` with RAII.
//! [`AmigaDrawInfo`] borrows the parent screen via a lifetime parameter,
//! ensuring the DrawInfo is freed before the screen is unlocked.

use crate::error::{AmigaError, Result};
use amigaos4_sys::{DrawInfo, Screen, CONST_STRPTR};

/// RAII wrapper around LockPubScreen / UnlockPubScreen.
pub struct PubScreen {
    ptr: *mut Screen,
}

impl PubScreen {
    /// Lock a public screen by name. Pass `None` for the default (Workbench)
    /// screen. A `Some` name must be null-terminated.
    pub fn lock(name: Option<&[u8]>) -> Result<Self> {
        let name_ptr = match name {
            Some(s) => crate::cstr::require_nul(s)? as CONST_STRPTR,
            None => core::ptr::null(),
        };
        let ptr = unsafe { amigaos4_sys::intuition_lock_pub_screen(name_ptr) };
        if ptr.is_null() {
            Err(AmigaError::NullPointer)
        } else {
            Ok(Self { ptr })
        }
    }

    /// Get the DrawInfo for this screen. The returned handle borrows this screen.
    pub fn draw_info(&self) -> Result<AmigaDrawInfo<'_>> {
        let di = unsafe { amigaos4_sys::intuition_get_screen_draw_info(self.ptr) };
        if di.is_null() {
            Err(AmigaError::NullPointer)
        } else {
            Ok(AmigaDrawInfo { ptr: di, screen: self })
        }
    }

    /// Get the raw `Screen` pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut Screen {
        self.ptr
    }
}

impl Drop for PubScreen {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { amigaos4_sys::intuition_unlock_pub_screen(core::ptr::null(), self.ptr) }
        }
    }
}

/// RAII handle for screen DrawInfo. Borrows the parent PubScreen.
pub struct AmigaDrawInfo<'a> {
    ptr: *mut DrawInfo,
    screen: &'a PubScreen,
}

impl<'a> AmigaDrawInfo<'a> {
    /// Get the raw `DrawInfo` pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut DrawInfo {
        self.ptr
    }
}

impl Drop for AmigaDrawInfo<'_> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                amigaos4_sys::intuition_free_screen_draw_info(self.screen.ptr, self.ptr)
            }
        }
    }
}

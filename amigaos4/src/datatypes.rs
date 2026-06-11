//! Picture loading via datatypes.library.
//!
//! [`DtPicture`] wraps `NewDTObjectA`/`GetDTAttrsA`/`DisposeDTObject`
//! for the most common datatypes use: load any picture format the
//! system has a datatype for (IFF, PNG, JPEG, ...) and query its
//! properties. The library is opened at runtime, so this works in
//! every build mode.
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::datatypes::DtPicture;
//! use amigaos4::amstr;
//!
//! let pic = DtPicture::load(amstr!("SYS:Prefs/Presets/Patterns/Bricks/BricksBlue.brush"))?;
//! let (w, h) = pic.dimensions()?;
//! ```
//!
//! Rendering into a window (`DTM_PROCLAYOUT` + `DTM_DRAW`) is not
//! wrapped yet — use [`DtPicture::as_ptr`] with the BOOPSI method
//! helpers for that.

use crate::cstr::require_nul;
use crate::error::{AmigaError, Result};
use crate::iface::OpenedInterface;
use crate::tag::TagListBuilder;
use amigaos4_sys::{APTR, CONST_STRPTR, DataTypesIFace, TagItem, TAG_USER, TAG_DONE};

// datatypes/datatypesclass.h + pictureclass.h (SDK 54.16).

/// `DTA_Dummy = TAG_USER + 0x1000` — base for all DTA_/PDTA_ tags.
const DTA_DUMMY: u32 = TAG_USER + 0x1000;

/// Nominal height in pixels (u32 attr).
pub const DTA_NOMINAL_VERT: u32 = DTA_DUMMY + 124;
/// Nominal width in pixels (u32 attr).
pub const DTA_NOMINAL_HORIZ: u32 = DTA_DUMMY + 125;
/// (struct BitMap *) the picture's bitmap after layout.
pub const PDTA_BIT_MAP: u32 = DTA_DUMMY + 202;
/// (BOOL) remap to a screen's palette; off for raw truecolour data.
pub const PDTA_REMAP: u32 = DTA_DUMMY + 211;

/// A loaded picture datatype object (RAII — disposed on drop).
pub struct DtPicture {
    obj: *mut APTR,
    lib: OpenedInterface,
}

impl DtPicture {
    /// Load a picture file (any installed picture datatype). `path`
    /// must be null-terminated (use [`amstr!`](crate::amstr)).
    ///
    /// Loads with `PDTA_Remap = FALSE` (no target screen).
    ///
    /// # Errors
    ///
    /// `NotNulTerminated` for a missing `\0`; `NullPointer` if
    /// datatypes.library cannot be opened or the file cannot be
    /// loaded as a picture.
    pub fn load(path: &[u8]) -> Result<Self> {
        let path_ptr = require_nul(path)?;
        let lib = OpenedInterface::open(b"datatypes.library\0", 52)?;
        let idt = lib.as_ptr() as *mut DataTypesIFace;

        let tags: alloc::vec::Vec<TagItem> = TagListBuilder::new()
            .tag(PDTA_REMAP, 0)
            .build();

        // SAFETY: idt is the open datatypes interface; path is
        // null-terminated; tags is TAG_DONE-terminated.
        let obj = unsafe {
            ((*idt).NewDTObjectA)(
                idt,
                path_ptr as CONST_STRPTR,
                tags.as_ptr() as *mut APTR,
            )
        };
        if obj.is_null() {
            return Err(AmigaError::NullPointer);
        }
        Ok(Self { obj, lib })
    }

    /// Query a single u32 attribute (e.g. [`DTA_NOMINAL_HORIZ`]).
    /// Returns `None` if the object does not provide it.
    pub fn get_attr(&self, tag: u32) -> Option<u32> {
        let idt = self.lib.as_ptr() as *mut DataTypesIFace;
        let mut value: u32 = 0;
        let query = [
            TagItem { ti_Tag: tag, ti_Data: &mut value as *mut u32 as u32 },
            TagItem { ti_Tag: TAG_DONE, ti_Data: 0 },
        ];
        // SAFETY: obj is a live DT object; the tag data pointer stays
        // valid for the duration of the call.
        let hits = unsafe {
            ((*idt).GetDTAttrsA)(idt, self.obj, query.as_ptr() as *mut APTR)
        };
        if hits > 0 {
            Some(value)
        } else {
            None
        }
    }

    /// The picture's nominal (width, height) in pixels.
    ///
    /// # Errors
    ///
    /// `NullPointer` if the object does not report dimensions.
    pub fn dimensions(&self) -> Result<(u32, u32)> {
        let w = self.get_attr(DTA_NOMINAL_HORIZ).ok_or(AmigaError::NullPointer)?;
        let h = self.get_attr(DTA_NOMINAL_VERT).ok_or(AmigaError::NullPointer)?;
        Ok((w, h))
    }

    /// The raw BOOPSI object pointer, for datatype methods this
    /// wrapper does not cover (e.g. `DTM_PROCLAYOUT`/`DTM_DRAW` via
    /// the `amiga_do_method_*` helpers).
    #[inline]
    pub fn as_ptr(&self) -> *mut APTR {
        self.obj
    }
}

impl Drop for DtPicture {
    fn drop(&mut self) {
        let idt = self.lib.as_ptr() as *mut DataTypesIFace;
        // SAFETY: obj came from NewDTObjectA and is disposed exactly
        // once; the library closes afterwards when self.lib drops.
        unsafe { ((*idt).DisposeDTObject)(idt, self.obj) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dta_tags_match_sdk() {
        // datatypes/datatypesclass.h + pictureclass.h
        assert_eq!(DTA_DUMMY, TAG_USER + 0x1000);
        assert_eq!(DTA_NOMINAL_VERT, DTA_DUMMY + 124);
        assert_eq!(DTA_NOMINAL_HORIZ, DTA_DUMMY + 125);
        assert_eq!(PDTA_BIT_MAP, DTA_DUMMY + 202);
        assert_eq!(PDTA_REMAP, DTA_DUMMY + 211);
    }
}

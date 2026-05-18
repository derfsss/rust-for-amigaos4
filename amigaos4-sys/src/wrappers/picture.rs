//! IPicture global(s) and convenience wrappers.
//!
//! Hand-written binding for picture.datatype's BOOPSI class. Like
//! other ReAction-adjacent classes, exposes a `_GetClass` accessor.
//! `PICTURE_Private` is included because the SDK declares it as a
//! regular vtable method (not marked APTR/UNIMPLEMENTED); it's an
//! SDK-internal slot whose body is unspecified, so calling it is
//! probably a bad idea but the binding mirrors what the SDK exposes.

use crate::types::*;
use crate::interfaces::picture::*;

// ---- IPicture (PictureIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IPicture: *mut PictureIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IPicture: *mut PictureIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn picture_picture_get_class() -> *mut APTR {
    ((*IPicture).PICTURE_GetClass)(IPicture)
}

#[inline]
pub unsafe fn picture_picture_private() {
    ((*IPicture).PICTURE_Private)(IPicture)
}

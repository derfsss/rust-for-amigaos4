//! IVersion global(s) and convenience wrappers.
//!
//! Hand-written to match the amigaos4-bindgen convention used by the
//! other wrapper modules.

use crate::types::*;
use crate::interfaces::version::*;

// ---- IVersion (VersionIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IVersion: *mut VersionIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IVersion: *mut VersionIFace = core::ptr::null_mut();

// ── Default-locale strings ───────────────────────────────────

#[inline]
pub unsafe fn version_get_release_string() -> CONST_STRPTR {
    ((*IVersion).GetReleaseString)(IVersion)
}

#[inline]
pub unsafe fn version_get_osstring() -> CONST_STRPTR {
    ((*IVersion).GetOSString)(IVersion)
}

#[inline]
pub unsafe fn version_get_copyright_string() -> CONST_STRPTR {
    ((*IVersion).GetCopyrightString)(IVersion)
}

#[inline]
pub unsafe fn version_get_company_string() -> CONST_STRPTR {
    ((*IVersion).GetCompanyString)(IVersion)
}

// ── Localized variants (formatted per current locale settings) ──

#[inline]
pub unsafe fn version_get_localized_release_string() -> CONST_STRPTR {
    ((*IVersion).GetLocalizedReleaseString)(IVersion)
}

#[inline]
pub unsafe fn version_get_localized_osstring() -> CONST_STRPTR {
    ((*IVersion).GetLocalizedOSString)(IVersion)
}

#[inline]
pub unsafe fn version_get_localized_copyright_string() -> CONST_STRPTR {
    ((*IVersion).GetLocalizedCopyrightString)(IVersion)
}

#[inline]
pub unsafe fn version_get_localized_company_string() -> CONST_STRPTR {
    ((*IVersion).GetLocalizedCompanyString)(IVersion)
}

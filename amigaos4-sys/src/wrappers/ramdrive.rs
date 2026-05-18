//! IRamdrive global(s) and convenience wrappers.
//!
//! Hand-written binding for ramdrive.device — the AmigaOS recoverable
//! RAM disk (RAD:). The two methods kill RAD0: or a specified RAD
//! unit respectively, freeing its memory.

use crate::types::*;
use crate::interfaces::ramdrive::*;

// ---- IRamdrive (RamdriveIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IRamdrive: *mut RamdriveIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IRamdrive: *mut RamdriveIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn ramdrive_kill_rad0() -> STRPTR {
    ((*IRamdrive).KillRAD0)(IRamdrive)
}

#[inline]
pub unsafe fn ramdrive_kill_rad(unit: u32) -> STRPTR {
    ((*IRamdrive).KillRAD)(IRamdrive, unit)
}

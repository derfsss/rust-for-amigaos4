//! ITimezone global(s) and convenience wrappers.
//!
//! Hand-written binding for timezone.library — local/UTC time
//! conversion and timezone preferences.

use crate::types::*;
use crate::interfaces::timezone::*;

// ---- ITimezone (TimezoneIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ITimezone: *mut TimezoneIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ITimezone: *mut TimezoneIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn timezone_get_timezone_attrs_a(reserved: APTR, tag_list: *const TagItem) -> u32 {
    ((*ITimezone).GetTimezoneAttrsA)(ITimezone, reserved, tag_list)
}

#[inline]
pub unsafe fn timezone_set_timezone_attrs_a(reserved: APTR, tag_list: *const TagItem) -> u32 {
    ((*ITimezone).SetTimezoneAttrsA)(ITimezone, reserved, tag_list)
}

#[inline]
pub unsafe fn timezone_parse_zone(zone_string: CONST_STRPTR) -> u32 {
    ((*ITimezone).ParseZone)(ITimezone, zone_string)
}

#[inline]
pub unsafe fn timezone_get_utctime(time_val: *mut TimeVal) -> u32 {
    ((*ITimezone).GetUTCTime)(ITimezone, time_val)
}

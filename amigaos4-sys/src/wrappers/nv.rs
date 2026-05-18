//! INV global(s) and convenience wrappers.
//!
//! Hand-written binding for nonvolatile.library — battery-backed
//! NVRAM storage on AmigaOne hardware. Stores small key/value
//! settings that survive reboot (preferences, BIOS variables).
//! Bound from `interfaces/nv.rs` (Rust basename `nv`, SDK header
//! `nonvolatile.h`).

use crate::types::*;
use crate::interfaces::nv::*;

// ---- INV (NVIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static INV: *mut NVIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut INV: *mut NVIFace = core::ptr::null_mut();

// ── NV variable storage ──────────────────────────────────────

#[inline]
pub unsafe fn nv_get_copy_nv(app_name: CONST_STRPTR, item_name: CONST_STRPTR, kill: u32) -> APTR {
    ((*INV).GetCopyNV)(INV, app_name, item_name, kill)
}

#[inline]
pub unsafe fn nv_free_nvdata(data: APTR) {
    ((*INV).FreeNVData)(INV, data)
}

#[inline]
pub unsafe fn nv_store_nv(
    app_name: CONST_STRPTR, item_name: CONST_STRPTR, data: APTR, size: u32, kill_requesters: u32,
) -> u16 {
    ((*INV).StoreNV)(INV, app_name, item_name, data, size, kill_requesters)
}

#[inline]
pub unsafe fn nv_delete_nv(app_name: CONST_STRPTR, item_name: CONST_STRPTR, kill_requesters: u32) -> u32 {
    ((*INV).DeleteNV)(INV, app_name, item_name, kill_requesters)
}

// ── Enumeration ──────────────────────────────────────────────

#[inline]
pub unsafe fn nv_get_nvinfo(kill_requesters: u32) -> *mut NVInfo {
    ((*INV).GetNVInfo)(INV, kill_requesters)
}

#[inline]
pub unsafe fn nv_get_nvlist(app_name: CONST_STRPTR, kill_requesters: u32) -> *mut MinList {
    ((*INV).GetNVList)(INV, app_name, kill_requesters)
}

#[inline]
pub unsafe fn nv_set_nvprotection(
    app_name: CONST_STRPTR, item_name: CONST_STRPTR, mask: i32, kill_requesters: u32,
) -> u32 {
    ((*INV).SetNVProtection)(INV, app_name, item_name, mask, kill_requesters)
}

// ── U-Boot environment variables ─────────────────────────────

#[inline]
pub unsafe fn nv_set_uboot_var(name: CONST_STRPTR, value: CONST_STRPTR) -> u32 {
    ((*INV).SetUBootVar)(INV, name, value)
}

#[inline]
pub unsafe fn nv_get_uboot_var(name: CONST_STRPTR) -> CONST_STRPTR {
    ((*INV).GetUBootVar)(INV, name)
}

#[inline]
pub unsafe fn nv_free_uboot_var(value: CONST_STRPTR) {
    ((*INV).FreeUBootVar)(INV, value)
}

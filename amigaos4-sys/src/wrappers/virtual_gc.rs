//! IVirtual global(s) and convenience wrappers.
//!
//! Hand-written binding for virtual.gadget — the ReAction virtual-
//! scroll container class. Used to wrap a child gadget in a
//! scrolling viewport.

use crate::types::*;
use crate::interfaces::virtual_gc::*;

// ---- IVirtual (VirtualIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IVirtual: *mut VirtualIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IVirtual: *mut VirtualIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn virtual_gc_virtual_get_class() -> *mut APTR {
    ((*IVirtual).VIRTUAL_GetClass)(IVirtual)
}

#[inline]
pub unsafe fn virtual_gc_refresh_virtual_gadget(
    gadget: *mut Gadget, virtual_class: *mut APTR, win: *mut Window, req: *mut Requester,
) {
    ((*IVirtual).RefreshVirtualGadget)(IVirtual, gadget, virtual_class, win, req)
}

#[inline]
pub unsafe fn virtual_gc_rethink_virtual_size(
    gadget: *mut APTR, virtual_class: *mut APTR, font: *mut TextFont, screen: *mut Screen,
    limits: *mut LayoutLimits,
) -> u32 {
    ((*IVirtual).RethinkVirtualSize)(IVirtual, gadget, virtual_class, font, screen, limits)
}

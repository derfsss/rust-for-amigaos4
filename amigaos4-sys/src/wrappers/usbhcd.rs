//! IUSBHCD global(s) and convenience wrappers.
//!
//! Hand-written binding for USB host-controller-driver interface —
//! the lower-level USB stack (xhci/ehci/ohci drivers register here
//! and the upper stack consumes the interface).

use crate::types::*;
use crate::interfaces::usbhcd::*;

// ---- IUSBHCD (USBHCDIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IUSBHCD: *mut USBHCDIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IUSBHCD: *mut USBHCDIFace = core::ptr::null_mut();

// ── Function (= USB device) management ───────────────────────

#[inline]
pub unsafe fn usbhcd_usbhcadd_function_a(
    bus: APTR, function: APTR, parent: APTR, tag_list: *mut TagItem,
) -> APTR {
    ((*IUSBHCD).USBHCAddFunctionA)(IUSBHCD, bus, function, parent, tag_list)
}

#[inline]
pub unsafe fn usbhcd_usbhcrem_function(bus: APTR, function: APTR) {
    ((*IUSBHCD).USBHCRemFunction)(IUSBHCD, bus, function)
}

// ── Endpoint management ──────────────────────────────────────

#[inline]
pub unsafe fn usbhcd_usbhcadd_end_point_a(
    bus: APTR, function: APTR, endpoint: APTR, ep_type: i32, tag_list: *mut TagItem,
) -> APTR {
    ((*IUSBHCD).USBHCAddEndPointA)(IUSBHCD, bus, function, endpoint, ep_type, tag_list)
}

#[inline]
pub unsafe fn usbhcd_usbhcrem_end_point(bus: APTR, endpoint: APTR) {
    ((*IUSBHCD).USBHCRemEndPoint)(IUSBHCD, bus, endpoint)
}

// ── Bus / unit / hub queries ─────────────────────────────────

#[inline]
pub unsafe fn usbhcd_usbhcget_attrs_a(tag_list: *mut TagItem) {
    ((*IUSBHCD).USBHCGetAttrsA)(IUSBHCD, tag_list)
}

#[inline]
pub unsafe fn usbhcd_usbhcinit_root_hub_a(
    bus: APTR, raw_iface: *mut UsbRawInterface, hub: *mut APTR, tag_list: *mut TagItem,
) -> i32 {
    ((*IUSBHCD).USBHCInitRootHubA)(IUSBHCD, bus, raw_iface, hub, tag_list)
}

#[inline]
pub unsafe fn usbhcd_usbhcuninit_root_hub(bus: APTR, hub: APTR) {
    ((*IUSBHCD).USBHCUninitRootHub)(IUSBHCD, bus, hub)
}

#[inline]
pub unsafe fn usbhcd_usbhcget_frame_number(bus: APTR, time: *mut TimeVal) -> u32 {
    ((*IUSBHCD).USBHCGetFrameNumber)(IUSBHCD, bus, time)
}

#[inline]
pub unsafe fn usbhcd_usbhcget_unit_attrs_a(unit: APTR, tag_list: *mut TagItem) {
    ((*IUSBHCD).USBHCGetUnitAttrsA)(IUSBHCD, unit, tag_list)
}

//! IPerfMon global(s) and convenience wrappers.
//!
//! Hand-written binding for performancemonitor.library — PowerPC
//! hardware-performance-counter access (cache misses, branch
//! mispredicts, cycle counts, etc.). Rust basename `perfmon`
//! overrides to SDK `performancemonitor`.

use crate::types::*;
use crate::interfaces::perfmon::*;

// ---- IPerfMon (PerformanceMonitorIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IPerfMon: *mut PerformanceMonitorIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IPerfMon: *mut PerformanceMonitorIFace = core::ptr::null_mut();

// ── Per-counter operations ───────────────────────────────────

#[inline]
pub unsafe fn perfmon_counter_get(counter: u32) -> u32 {
    ((*IPerfMon).CounterGet)(IPerfMon, counter)
}

#[inline]
pub unsafe fn perfmon_counter_match(counter: u32) -> u32 {
    ((*IPerfMon).CounterMatch)(IPerfMon, counter)
}

#[inline]
pub unsafe fn perfmon_counter_control(counter: u32, control: i32, threshold: u32) -> u32 {
    ((*IPerfMon).CounterControl)(IPerfMon, counter, control, threshold)
}

// ── Event selection ──────────────────────────────────────────

#[inline]
pub unsafe fn perfmon_event_control(tag_list: *mut TagItem) -> u32 {
    ((*IPerfMon).EventControl)(IPerfMon, tag_list)
}

// ── Interrupt-driven sampling ────────────────────────────────

#[inline]
pub unsafe fn perfmon_set_interrupt_vector(counter: u32, interrupt: *mut Interrupt) -> *mut Interrupt {
    ((*IPerfMon).SetInterruptVector)(IPerfMon, counter, interrupt)
}

// ── Task-scope marking ───────────────────────────────────────

#[inline]
pub unsafe fn perfmon_mark(task: *mut Task) {
    ((*IPerfMon).Mark)(IPerfMon, task)
}

#[inline]
pub unsafe fn perfmon_unmark(task: *mut Task) {
    ((*IPerfMon).Unmark)(IPerfMon, task)
}

// ── Monitor lifecycle / state queries ────────────────────────

#[inline]
pub unsafe fn perfmon_monitor_control(tag_list: *mut TagItem) -> u32 {
    ((*IPerfMon).MonitorControl)(IPerfMon, tag_list)
}

#[inline]
pub unsafe fn perfmon_query(attr: u32) -> u32 {
    ((*IPerfMon).Query)(IPerfMon, attr)
}

// ── Sampled-address-register access ──────────────────────────

#[inline]
pub unsafe fn perfmon_set_sampled_address(address: APTR) {
    ((*IPerfMon).SetSampledAddress)(IPerfMon, address)
}

#[inline]
pub unsafe fn perfmon_get_sampled_address() -> APTR {
    ((*IPerfMon).GetSampledAddress)(IPerfMon)
}

// ── Breakpoint registers ─────────────────────────────────────

#[inline]
pub unsafe fn perfmon_set_breakpoint(slot: u32, address: APTR, mask: u32, control: u32) {
    ((*IPerfMon).SetBreakpoint)(IPerfMon, slot, address, mask, control)
}

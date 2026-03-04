//! PPC inline assembly helpers for cache management and MMIO access.
//!
//! Direct translations of the C inline asm patterns used in AmigaOS device
//! drivers (VirtIO9P, VirtualSCSIDevice) for hardware interaction on PowerPC.
//!
//! # Cache Management
//! - `cache_flush` — dcbst (write dirty cache to RAM, keep cached)
//! - `cache_invalidate` — dcbf (write dirty cache to RAM + invalidate)
//! - Both user-mode safe. Do NOT use dcbi (supervisor-only).
//!
//! # MMIO Access
//! PCI uses little-endian registers on big-endian PPC. The `lwbrx`/`stwbrx`
//! instructions perform byte-reversed load/store, giving LE↔BE conversion.
//! Works on Pegasos2 (MV64361 transparent bridge).
//! Does NOT work on AmigaOne (Articia S — use I/O port access instead).
//!
//! # Memory Barriers
//! - `sync` — full memory barrier (heavyweight)
//! - `lwsync` — lightweight sync (load-store ordering)
//! - `eieio` — enforce in-order execution of I/O
//! - `mbar` — memory barrier (used after MMIO writes)

use core::arch::asm;

// ---- Cache Management ----

/// Cache line size on G3/G4 PPC processors (32 bytes).
pub const CACHE_LINE: usize = 32;

/// Flush a memory range from CPU data cache to RAM using `dcbst`.
///
/// After this call, the device can read the latest CPU-written data.
/// The cache lines remain valid (not invalidated).
///
/// # Safety
/// `addr` must be a valid pointer to at least `len` bytes.
#[inline]
pub unsafe fn cache_flush(addr: *const u8, len: usize) {
    let mut p = (addr as usize & !(CACHE_LINE - 1)) as *const u8;
    let end = unsafe { addr.add(len) };
    while p < end {
        unsafe {
            asm!("dcbst 0, {0}", in(reg) p, options(nostack, preserves_flags));
        }
        p = unsafe { p.add(CACHE_LINE) };
    }
    unsafe {
        asm!("sync", options(nostack, preserves_flags));
    }
}

/// Flush + invalidate a memory range using `dcbf`.
///
/// After this call, the next CPU read fetches fresh data from RAM
/// (device-written data becomes visible to the CPU).
///
/// # Safety
/// `addr` must be a valid pointer to at least `len` bytes.
#[inline]
pub unsafe fn cache_invalidate(addr: *const u8, len: usize) {
    let mut p = (addr as usize & !(CACHE_LINE - 1)) as *const u8;
    let end = unsafe { addr.add(len) };
    while p < end {
        unsafe {
            asm!("dcbf 0, {0}", in(reg) p, options(nostack));
        }
        p = unsafe { p.add(CACHE_LINE) };
    }
    unsafe {
        asm!("sync", options(nostack, preserves_flags));
    }
}

// ---- MMIO Access (Byte-Reversed for LE PCI on BE PPC) ----

/// Read an 8-bit MMIO register.
///
/// # Safety
/// `addr` must be a valid MMIO address.
#[inline]
pub unsafe fn mmio_r8(addr: u32) -> u8 {
    let r: u32;
    unsafe {
        asm!("lbz {0}, 0({1})", out(reg) r, in(reg) addr, options(nostack, preserves_flags));
    }
    r as u8
}

/// Write an 8-bit MMIO register, followed by `mbar` barrier.
///
/// # Safety
/// `addr` must be a valid MMIO address.
#[inline]
pub unsafe fn mmio_w8(addr: u32, val: u8) {
    unsafe {
        asm!(
            "stb {0}, 0({1})",
            "mbar",
            in(reg) val as u32,
            in(reg) addr,
            options(nostack, preserves_flags)
        );
    }
}

/// Read a 16-bit MMIO register with byte-reversal (LE→BE).
///
/// # Safety
/// `addr` must be a valid, 2-byte aligned MMIO address.
#[inline]
pub unsafe fn mmio_r16(addr: u32) -> u16 {
    let r: u32;
    unsafe {
        asm!("lhbrx {0}, 0, {1}", out(reg) r, in(reg) addr, options(nostack, preserves_flags));
    }
    r as u16
}

/// Write a 16-bit MMIO register with byte-reversal (BE→LE), followed by `mbar`.
///
/// # Safety
/// `addr` must be a valid, 2-byte aligned MMIO address.
#[inline]
pub unsafe fn mmio_w16(addr: u32, val: u16) {
    unsafe {
        asm!(
            "sthbrx {0}, 0, {1}",
            "mbar",
            in(reg) val as u32,
            in(reg) addr,
            options(nostack, preserves_flags)
        );
    }
}

/// Read a 32-bit MMIO register with byte-reversal (LE→BE).
///
/// # Safety
/// `addr` must be a valid, 4-byte aligned MMIO address.
#[inline]
pub unsafe fn mmio_r32(addr: u32) -> u32 {
    let r: u32;
    unsafe {
        asm!("lwbrx {0}, 0, {1}", out(reg) r, in(reg) addr, options(nostack, preserves_flags));
    }
    r
}

/// Write a 32-bit MMIO register with byte-reversal (BE→LE), followed by `mbar`.
///
/// # Safety
/// `addr` must be a valid, 4-byte aligned MMIO address.
#[inline]
pub unsafe fn mmio_w32(addr: u32, val: u32) {
    unsafe {
        asm!(
            "stwbrx {0}, 0, {1}",
            "mbar",
            in(reg) val,
            in(reg) addr,
            options(nostack, preserves_flags)
        );
    }
}

// ---- Memory Barriers ----

/// Full memory barrier (`sync`). Heavyweight — orders all loads and stores.
#[inline]
pub unsafe fn sync_barrier() {
    unsafe {
        asm!("sync", options(nostack, preserves_flags));
    }
}

/// Lightweight sync (`lwsync`). Orders load-store and store-store, but not load-load.
#[inline]
pub unsafe fn lwsync_barrier() {
    unsafe {
        asm!("lwsync", options(nostack, preserves_flags));
    }
}

/// Enforce in-order execution of I/O (`eieio`).
#[inline]
pub unsafe fn eieio_barrier() {
    unsafe {
        asm!("eieio", options(nostack, preserves_flags));
    }
}

/// Memory barrier for MMIO writes (`mbar`).
#[inline]
pub unsafe fn mbar_barrier() {
    unsafe {
        asm!("mbar", options(nostack, preserves_flags));
    }
}

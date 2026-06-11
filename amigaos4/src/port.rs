//! RAII message port management via `CreateMsgPort`/`DeleteMsgPort`.
//!
//! Message ports are the fundamental inter-process communication mechanism
//! in AmigaOS. [`AmigaMsgPort`] ensures the port is cleaned up on drop.

use crate::error::{AmigaError, Result};
use amigaos4_sys::MsgPort;

/// RAII wrapper around CreateMsgPort / DeleteMsgPort.
pub struct AmigaMsgPort {
    ptr: *mut MsgPort,
}

impl AmigaMsgPort {
    /// Create a new message port.
    pub fn new() -> Result<Self> {
        let ptr = unsafe { amigaos4_sys::exec_create_msg_port() };
        if ptr.is_null() {
            Err(AmigaError::AllocationFailed)
        } else {
            Ok(Self { ptr })
        }
    }

    /// Get the raw `MsgPort` pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut MsgPort {
        self.ptr
    }

    /// The Exec signal mask of this port (`1 << mp_SigBit`).
    ///
    /// Useful for combining a port wait with other signals via
    /// `exec_wait`, and used by the async runtime to wake the executor
    /// when a message arrives.
    #[inline]
    pub fn signal_mask(&self) -> u32 {
        // struct MsgPort (2-byte packed): Node 0..14, mp_Flags @14,
        // mp_SigBit @15, mp_SigTask @16, mp_MsgList @20.
        // SAFETY: self.ptr is a valid open MsgPort (created in new()).
        unsafe { 1u32 << *((self.ptr as *const u8).add(15)) }
    }
}

impl Drop for AmigaMsgPort {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { amigaos4_sys::exec_delete_msg_port(self.ptr) }
        }
    }
}

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

    #[inline]
    pub fn as_ptr(&self) -> *mut MsgPort {
        self.ptr
    }
}

impl Drop for AmigaMsgPort {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { amigaos4_sys::exec_delete_msg_port(self.ptr) }
        }
    }
}

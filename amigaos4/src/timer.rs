//! Safe timer device wrapper for delays and system time queries.
//!
//! [`AmigaTimer`] provides RAII access to `timer.device` for synchronous
//! delays and time queries. Free functions [`get_up_time`] and
//! [`micro_delay`] use the ITimer interface directly without opening the
//! device.
//!
//! Time values use [`TimerVal`], a seconds + microseconds pair that works
//! in both `app` and `driver` modes (no dependency on the feature-gated
//! `time` module).
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::timer::{AmigaTimer, TimerUnit, TimerVal};
//!
//! let mut timer = AmigaTimer::open(TimerUnit::MicroHz)?;
//! timer.delay(TimerVal::from_millis(500))?;
//! let now = timer.get_sys_time();
//! ```

use crate::error::{AmigaError, Result};
use crate::port::AmigaMsgPort;
use amigaos4_sys::{IORequest, MsgPort, CONST_STRPTR};

// ---------------------------------------------------------------------------
// TimerVal — lightweight time representation (no feature dependencies)
// ---------------------------------------------------------------------------

/// A time value in seconds + microseconds.
///
/// This is a lightweight representation that works in both `app` and
/// `driver` modes, independent of the feature-gated `time::Duration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerVal {
    /// Whole seconds.
    pub secs: u32,
    /// Microsecond fractional part (0..999_999).
    pub micros: u32,
}

impl TimerVal {
    /// Zero time value.
    pub const ZERO: TimerVal = TimerVal { secs: 0, micros: 0 };

    /// Create from seconds and microseconds.
    pub fn new(secs: u32, micros: u32) -> Self {
        Self { secs, micros }
    }

    /// Create from milliseconds.
    pub fn from_millis(ms: u32) -> Self {
        Self {
            secs: ms / 1_000,
            micros: (ms % 1_000) * 1_000,
        }
    }

    /// Convert to total milliseconds.
    pub fn as_millis(&self) -> u64 {
        self.secs as u64 * 1_000 + self.micros as u64 / 1_000
    }
}

// ---------------------------------------------------------------------------
// Timer unit selection
// ---------------------------------------------------------------------------

/// Timer device unit for [`AmigaTimer::open`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerUnit {
    /// Microsecond precision (1/1,000,000 s). May drift under load.
    MicroHz = 0,
    /// VBlank precision (~1/60 s). Stable, low overhead.
    VBlank = 1,
    /// E-Clock precision. System-dependent tick rate.
    EClock = 2,
    /// Wait until an absolute system time.
    WaitUntil = 3,
}

// ---------------------------------------------------------------------------
// Timer device constants (from devices/timer.h)
// ---------------------------------------------------------------------------

/// Timer device name.
const TIMER_NAME: &[u8] = b"timer.device\0";

/// Command: add a delay request.
const TR_ADDREQUEST: u16 = 9; // CMD_NONSTD

/// Command: get the system time.
const TR_GETSYSTIME: u16 = 10; // CMD_NONSTD + 1

// ---------------------------------------------------------------------------
// TimeRequest layout (packed(2) on PPC)
//
// struct TimeRequest {
//     struct IORequest Request;  // variable size
//     struct TimeVal   Time;     // { u32 Seconds, u32 Microseconds }
// };
//
// On PPC (32-bit, packed(2)):
//   IORequest = Message(20) + *Device(4) + *Unit(4) + u16(2) + u8(1) + i8(1) = 32 bytes
//   TimeVal = u32(4) + u32(4) = 8 bytes
//   TimeRequest = 40 bytes
//
// We access Time fields via known offsets from the IORequest pointer.
// ---------------------------------------------------------------------------

/// Size of TimeRequest for CreateIORequest.
const TIME_REQUEST_SIZE: u32 = 40;

/// Offset of `io_Command` (u16) within IORequest on PPC.
const IO_COMMAND_OFFSET: usize = 28;

/// Offset of `io_Error` (i8) within IORequest on PPC.
const IO_ERROR_OFFSET: usize = 31;

/// Offset of `Time.Seconds` (u32) within TimeRequest on PPC.
const TIME_SECONDS_OFFSET: usize = 32;

/// Offset of `Time.Microseconds` (u32) within TimeRequest on PPC.
const TIME_MICROS_OFFSET: usize = 36;

// ---------------------------------------------------------------------------
// AmigaTimer
// ---------------------------------------------------------------------------

/// RAII wrapper around the AmigaOS timer device.
///
/// Opens `timer.device` with a specified unit and provides synchronous
/// delay and time-query operations. The device, I/O request, and message
/// port are automatically cleaned up on drop.
pub struct AmigaTimer {
    port: AmigaMsgPort,
    request: *mut IORequest,
    device_open: bool,
}

impl AmigaTimer {
    /// Open the timer device with the given unit.
    ///
    /// # Errors
    ///
    /// Returns `AllocationFailed` if the message port or I/O request
    /// cannot be created, or if `OpenDevice` fails.
    pub fn open(unit: TimerUnit) -> Result<Self> {
        let port = AmigaMsgPort::new()?;

        // SAFETY: CreateIORequest returns a properly sized IORequest
        // (actually TimeRequest) or null on failure.
        let request = unsafe {
            amigaos4_sys::exec_create_iorequest(
                port.as_ptr() as *const MsgPort,
                TIME_REQUEST_SIZE,
            )
        };
        if request.is_null() {
            return Err(AmigaError::AllocationFailed);
        }

        // SAFETY: OpenDevice initializes the IORequest and opens the device.
        let rc = unsafe {
            amigaos4_sys::exec_open_device(
                TIMER_NAME.as_ptr() as CONST_STRPTR,
                unit as u32,
                request,
                0,
            )
        };
        if rc != 0 {
            // SAFETY: request was successfully allocated above.
            unsafe { amigaos4_sys::exec_delete_iorequest(request) };
            return Err(AmigaError::IoError(rc));
        }

        Ok(Self {
            port,
            request,
            device_open: true,
        })
    }

    /// Perform a synchronous delay.
    ///
    /// Blocks the calling task for the specified duration using
    /// `TR_ADDREQUEST` + `DoIO`.
    ///
    /// # Errors
    ///
    /// Returns `IoError` if the device I/O fails.
    pub fn delay(&mut self, duration: TimerVal) -> Result<()> {
        let secs = duration.secs;
        let micros = duration.micros;

        // SAFETY: We write to known offsets within the TimeRequest that
        // was allocated with the correct size in open().
        unsafe {
            let base = self.request as *mut u8;
            *(base.add(IO_COMMAND_OFFSET) as *mut u16) = TR_ADDREQUEST;
            *(base.add(TIME_SECONDS_OFFSET) as *mut u32) = secs;
            *(base.add(TIME_MICROS_OFFSET) as *mut u32) = micros;
        }

        // SAFETY: request is a valid, open timer device IORequest.
        let err = unsafe { amigaos4_sys::exec_do_io(self.request) };
        if err != 0 {
            Err(AmigaError::IoError(err as i32))
        } else {
            Ok(())
        }
    }

    /// Query the current system time via the timer device.
    ///
    /// Returns the system time as a [`TimerVal`]. The system time is
    /// monotonically increasing (seconds since midnight, Jan 1, 1978).
    pub fn get_sys_time(&mut self) -> TimerVal {
        // SAFETY: We write to known offsets within the TimeRequest.
        unsafe {
            let base = self.request as *mut u8;
            *(base.add(IO_COMMAND_OFFSET) as *mut u16) = TR_GETSYSTIME;
            *(base.add(TIME_SECONDS_OFFSET) as *mut u32) = 0;
            *(base.add(TIME_MICROS_OFFSET) as *mut u32) = 0;

            amigaos4_sys::exec_do_io(self.request);

            let secs = *(base.add(TIME_SECONDS_OFFSET) as *const u32);
            let micros = *(base.add(TIME_MICROS_OFFSET) as *const u32);
            TimerVal { secs, micros }
        }
    }
}

impl Drop for AmigaTimer {
    fn drop(&mut self) {
        if self.device_open {
            // SAFETY: request is a valid IORequest from an open device.
            unsafe { amigaos4_sys::exec_close_device(self.request) };
        }
        if !self.request.is_null() {
            // SAFETY: request was allocated by CreateIORequest.
            unsafe { amigaos4_sys::exec_delete_iorequest(self.request) };
        }
        // port is dropped automatically via AmigaMsgPort::Drop
    }
}

// ---------------------------------------------------------------------------
// Free functions (use ITimer directly, no device open needed)
// ---------------------------------------------------------------------------

/// Get the time since system startup.
///
/// Uses `ITimer->GetUpTime` directly. The returned value is monotonically
/// increasing and cannot be modified by `TR_SETSYSTIME`.
///
/// This function requires the `timer` feature on `amigaos4-sys` and that
/// `ITimer` has been initialized (true for any clib4 application using
/// `-lauto`).
pub fn get_up_time() -> TimerVal {
    // SAFETY: We pass a valid stack-allocated TimeVal to GetUpTime.
    // The function writes Seconds and Microseconds as two u32 fields.
    let mut tv = [0u32; 2]; // [Seconds, Microseconds]
    unsafe {
        amigaos4_sys::timer_get_up_time(tv.as_mut_ptr() as *mut amigaos4_sys::TimeVal);
    }
    TimerVal { secs: tv[0], micros: tv[1] }
}

/// Busy-wait for a number of microseconds.
///
/// Uses `ITimer->MicroDelay` directly. Only suitable for very short
/// delays (< 1 second). The calling task will spin, consuming CPU.
///
/// For longer delays, use [`AmigaTimer::delay`] instead.
pub fn micro_delay(microseconds: u32) {
    // SAFETY: MicroDelay is a simple function that busy-waits.
    unsafe { amigaos4_sys::timer_micro_delay(microseconds) };
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_unit_values() {
        assert_eq!(TimerUnit::MicroHz as u32, 0);
        assert_eq!(TimerUnit::VBlank as u32, 1);
        assert_eq!(TimerUnit::EClock as u32, 2);
        assert_eq!(TimerUnit::WaitUntil as u32, 3);
    }

    #[test]
    fn timer_name_null_terminated() {
        assert_eq!(TIMER_NAME.last(), Some(&0u8));
    }

    #[test]
    fn tr_addrequest_is_cmd_nonstd() {
        assert_eq!(TR_ADDREQUEST, 9);
    }

    #[test]
    fn tr_getsystime_is_cmd_nonstd_plus_1() {
        assert_eq!(TR_GETSYSTIME, 10);
    }

    #[test]
    fn time_request_offsets_consistent() {
        // TimeRequest layout: IORequest(32) + TimeVal(8) = 40 bytes
        assert_eq!(TIME_REQUEST_SIZE, 40);
        assert_eq!(TIME_SECONDS_OFFSET, 32);
        assert_eq!(TIME_MICROS_OFFSET, 36);
        assert_eq!(TIME_SECONDS_OFFSET + 4, TIME_MICROS_OFFSET);
    }

    #[test]
    fn timer_unit_debug() {
        extern crate alloc;
        let dbg = alloc::format!("{:?}", TimerUnit::MicroHz);
        assert_eq!(dbg, "MicroHz");
    }

    #[test]
    fn timer_val_from_millis() {
        let tv = TimerVal::from_millis(1500);
        assert_eq!(tv.secs, 1);
        assert_eq!(tv.micros, 500_000);
    }

    #[test]
    fn timer_val_as_millis() {
        let tv = TimerVal::new(2, 500_000);
        assert_eq!(tv.as_millis(), 2500);
    }

    #[test]
    fn timer_val_zero() {
        assert_eq!(TimerVal::ZERO.secs, 0);
        assert_eq!(TimerVal::ZERO.micros, 0);
    }
}

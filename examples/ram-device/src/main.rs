// ram.device — a complete exec device written in Rust.
//
// The C glue (device_glue.c) provides the Resident struct and the
// "__device" manager interface; every I/O decision is made here:
// command dispatch, bounds checking, quick-I/O vs ReplyMsg, io_Error
// and io_Actual reporting.
//
// The device exposes a single unit backed by a 64 KB RAM buffer:
//   CMD_READ   copy from buffer[io_Offset..] into io_Data (io_Length)
//   CMD_WRITE  copy io_Data into buffer[io_Offset..]
//   CMD_CLEAR  zero the whole buffer
//   CMD_RESET  zero the buffer
//
// Built with -nostartfiles -nodefaultlibs -lgcc (no CRT).

#![no_std]
#![no_main]
#![allow(non_snake_case)]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::ExecAllocator;
use amigaos4_sys::{IORequest, Message};

#[global_allocator]
static ALLOCATOR: ExecAllocator = ExecAllocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// exec/io.h constants
// ---------------------------------------------------------------------------

const CMD_RESET: u16 = 1;
const CMD_READ: u16 = 2;
const CMD_WRITE: u16 = 3;
const CMD_UPDATE: u16 = 4;
const CMD_CLEAR: u16 = 5;

/// exec/errors.h: IOERR_NOCMD — command not supported.
const IOERR_NOCMD: i8 = -3;
/// exec/errors.h: IOERR_BADLENGTH — bad length/offset.
const IOERR_BADLENGTH: i8 = -4;

/// exec/io.h: IOF_QUICK — if still set after BeginIO, the request
/// completed synchronously and must NOT be replied.
const IOF_QUICK: u8 = 1 << 0;

// ---------------------------------------------------------------------------
// IOStdReq field offsets (PPC, 2-byte packed — same layout convention
// as timer.rs documents for IORequest):
//   Message: 0..20, io_Device @20, io_Unit @24, io_Command @28 (u16),
//   io_Flags @30 (u8), io_Error @31 (i8), io_Actual @32 (u32),
//   io_Length @36 (u32), io_Data @40 (APTR), io_Offset @44 (u32)
// ---------------------------------------------------------------------------

const IO_COMMAND_OFFSET: usize = 28;
const IO_FLAGS_OFFSET: usize = 30;
const IO_ERROR_OFFSET: usize = 31;
const IO_ACTUAL_OFFSET: usize = 32;
const IO_LENGTH_OFFSET: usize = 36;
const IO_DATA_OFFSET: usize = 40;
const IO_OFFSET_OFFSET: usize = 44;

// ---------------------------------------------------------------------------
// The RAM backing store — one unit, 64 KB, lives in BSS.
// ---------------------------------------------------------------------------

const BUF_SIZE: usize = 64 * 1024;
static mut BUFFER: [u8; BUF_SIZE] = [0; BUF_SIZE];

// ---------------------------------------------------------------------------
// Device entry points called from device_glue.c
// ---------------------------------------------------------------------------

/// Called once from devInit() — IExec is already set.
#[no_mangle]
pub extern "C" fn rust_dev_init() -> i32 {
    amigaos4::serial_println!("ram.device: init ({} KB unit 0)", BUF_SIZE / 1024);
    0
}

/// Called from devOpen(). Only unit 0 exists.
#[no_mangle]
pub extern "C" fn rust_dev_open(_ior: *mut IORequest, unit: u32, _flags: u32) -> i8 {
    if unit != 0 {
        return IOERR_NOCMD;
    }
    amigaos4::serial_println!("ram.device: open unit {}", unit);
    0
}

/// Called from devClose().
#[no_mangle]
pub extern "C" fn rust_dev_close(_ior: *mut IORequest) {
    amigaos4::serial_println!("ram.device: close");
}

/// The BeginIO dispatcher — every I/O request lands here.
#[no_mangle]
pub extern "C" fn rust_dev_begin_io(ior: *mut IORequest) {
    if ior.is_null() {
        return;
    }

    // SAFETY: ior is a valid IOStdReq-sized request from the caller;
    // all offsets are within IOStdReq per the layout comment above.
    unsafe {
        let base = ior as *mut u8;
        let command = *(base.add(IO_COMMAND_OFFSET) as *const u16);
        let length = *(base.add(IO_LENGTH_OFFSET) as *const u32) as usize;
        let data = *(base.add(IO_DATA_OFFSET) as *const *mut u8);
        let offset = *(base.add(IO_OFFSET_OFFSET) as *const u32) as usize;

        let buf = core::ptr::addr_of_mut!(BUFFER) as *mut u8;
        let mut error: i8 = 0;
        let mut actual: usize = 0;

        match command {
            CMD_READ => {
                if offset > BUF_SIZE || data.is_null() {
                    error = IOERR_BADLENGTH;
                } else {
                    actual = length.min(BUF_SIZE - offset);
                    core::ptr::copy_nonoverlapping(buf.add(offset) as *const u8, data, actual);
                }
            }
            CMD_WRITE => {
                if offset > BUF_SIZE || data.is_null() {
                    error = IOERR_BADLENGTH;
                } else {
                    actual = length.min(BUF_SIZE - offset);
                    core::ptr::copy_nonoverlapping(data, buf.add(offset), actual);
                }
            }
            CMD_CLEAR | CMD_RESET => {
                core::ptr::write_bytes(buf, 0, BUF_SIZE);
            }
            CMD_UPDATE => {
                // RAM is always coherent — nothing to do.
            }
            _ => {
                error = IOERR_NOCMD;
            }
        }

        *(base.add(IO_ERROR_OFFSET) as *mut i8) = error;
        *(base.add(IO_ACTUAL_OFFSET) as *mut u32) = actual as u32;

        // Quick-I/O protocol: everything here completes synchronously.
        // If the caller allows quick I/O (IOF_QUICK set), leave the flag
        // set and do not reply; otherwise reply the message.
        let flags = *(base.add(IO_FLAGS_OFFSET) as *const u8);
        if flags & IOF_QUICK == 0 {
            amigaos4_sys::exec_reply_msg(ior as *mut Message);
        }
    }
}

/// AbortIO — all requests complete synchronously inside BeginIO, so
/// there is never an in-flight request to abort.
#[no_mangle]
pub extern "C" fn rust_dev_abort_io(_ior: *mut IORequest) {}

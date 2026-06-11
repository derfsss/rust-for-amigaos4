//
// AmigaOS 4 audio-tone - opens ahi.device (CreateMsgPort +
// AllocSysObject(IOREQUEST) + OpenDevice), then plays an actual tone:
// half a second of 440 Hz square wave, synthesised into a Vec and
// submitted with CMD_WRITE through a V4 AHIRequest.
//
// Demonstrates: exec device I/O from Rust end to end - request
// allocation, AHIRequest field layout, DoIO, io_Error/io_Actual
// reporting - audible on real hardware, verifiable via serial + exit
// code on a headless QEMU.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])           { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

// exec/exectags.h
const ASOT_IOREQUEST: u32 = 0;
const ASOIOR_SIZE:    u32 = TAG_USER + 10;
const ASOIOR_REPLYPORT: u32 = TAG_USER + 11;

// struct AHIRequest (devices/ahi.h), 2-byte packed on PPC:
//   IOStdReq        @ 0  (48 bytes; io_Command @28, io_Actual @32,
//                         io_Length @36, io_Data @40, io_Offset @44)
//   ahir_Version    @48  (u16)
//   ahir_Pad1       @50  (u16)
//   ahir_Private[2] @52  (8 bytes)
//   ahir_Type       @60  (u32)  AHIST_M8S = 0
//   ahir_Frequency  @64  (u32)
//   ahir_Volume     @68  (Fixed: 0x10000 = 1.0)
//   ahir_Position   @72  (Fixed: 0x8000 = centre)
//   ahir_Link       @76  (ptr)  = 80 bytes total
const AHIREQUEST_SIZE: u32 = 80;

const IO_COMMAND_OFFSET: usize = 28;
const IO_LENGTH_OFFSET: usize = 36;
const IO_DATA_OFFSET: usize = 40;
const IO_OFFSET_OFFSET: usize = 44;
const AHIR_VERSION_OFFSET: usize = 48;
const AHIR_TYPE_OFFSET: usize = 60;
const AHIR_FREQUENCY_OFFSET: usize = 64;
const AHIR_VOLUME_OFFSET: usize = 68;
const AHIR_POSITION_OFFSET: usize = 72;
const AHIR_LINK_OFFSET: usize = 76;

const CMD_WRITE: u16 = 3;
/// devices/ahi.h: mono, 8-bit signed.
const AHIST_M8S: u32 = 0;

/// Synthesise `secs/1000` ms of square wave at `tone_hz`, sampled at
/// `rate` Hz, 8-bit signed mono.
fn square_wave(rate: u32, tone_hz: u32, millis: u32) -> alloc::vec::Vec<i8> {
    let total = (rate * millis / 1000) as usize;
    let half_period = (rate / (tone_hz * 2)).max(1) as usize;
    let mut samples = alloc::vec::Vec::with_capacity(total);
    let mut level: i8 = 60;
    let mut next_flip = half_period;
    for i in 0..total {
        if i == next_flip {
            level = -level;
            next_flip += half_period;
        }
        samples.push(level);
    }
    samples
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - audio-tone\n\0");
    say(b"========================================\n\0");

    // 1. Reply port for the AHI device's responses.
    let port = unsafe { exec_create_msg_port() };
    if port.is_null() {
        say(b"FAIL: CreateMsgPort returned NULL\n\0");
        return 1;
    }

    // 2. AHI IORequest object, sized for AHIRequest and bound to our port.
    let tags = [
        TagItem { ti_Tag: ASOIOR_SIZE,      ti_Data: AHIREQUEST_SIZE },
        TagItem { ti_Tag: ASOIOR_REPLYPORT, ti_Data: port as u32 },
        TagItem { ti_Tag: TAG_DONE,         ti_Data: 0 },
    ];
    let req = unsafe { exec_alloc_sys_object(ASOT_IOREQUEST, tags.as_ptr()) };
    if req.is_null() {
        unsafe { exec_delete_msg_port(port); }
        say(b"FAIL: AllocSysObject(IOREQUEST) returned NULL\n\0");
        return 2;
    }

    // 3. OpenDevice("ahi.device", AHI_DEFAULT_UNIT=0). 0 == success.
    let rc = unsafe {
        exec_open_device(
            b"ahi.device\0".as_ptr() as CONST_STRPTR,
            0,
            req as *mut IORequest,
            0,
        )
    };
    say_d(b"  OpenDevice(ahi.device, 0) rc       = \0", rc as u32);

    if rc != 0 {
        say(b"  ahi.device not available on this image\n\0");
        unsafe {
            exec_free_sys_object(ASOT_IOREQUEST, req);
            exec_delete_msg_port(port);
        }
        say(b"FAIL: OpenDevice returned non-zero\n\0");
        return 3;
    }
    say(b"  ahi.device opened cleanly\n\0");

    // 4. Play 500 ms of 440 Hz square wave: fill the V4 AHIRequest
    //    fields and submit with CMD_WRITE via DoIO (blocks until the
    //    sample finished playing).
    let rate: u32 = 8000;
    let samples = square_wave(rate, 440, 500);
    say_d(b"  synthesised samples                = \0", samples.len() as u32);

    // SAFETY: req was allocated AHIREQUEST_SIZE bytes; all offsets are
    // within struct AHIRequest per the layout comment above.
    let err = unsafe {
        let base = req as *mut u8;
        *(base.add(IO_COMMAND_OFFSET) as *mut u16) = CMD_WRITE;
        *(base.add(IO_LENGTH_OFFSET) as *mut u32) = samples.len() as u32;
        *(base.add(IO_DATA_OFFSET) as *mut *const i8) = samples.as_ptr();
        *(base.add(IO_OFFSET_OFFSET) as *mut u32) = 0;
        *(base.add(AHIR_VERSION_OFFSET) as *mut u16) = 4;
        *(base.add(AHIR_TYPE_OFFSET) as *mut u32) = AHIST_M8S;
        *(base.add(AHIR_FREQUENCY_OFFSET) as *mut u32) = rate;
        *(base.add(AHIR_VOLUME_OFFSET) as *mut u32) = 0x1_0000; // 1.0
        *(base.add(AHIR_POSITION_OFFSET) as *mut u32) = 0x8000; // centre
        *(base.add(AHIR_LINK_OFFSET) as *mut u32) = 0;
        exec_do_io(req as *mut IORequest)
    };
    say_d(b"  CMD_WRITE (440 Hz, 500 ms) io_Error = \0", err as u32);

    unsafe {
        exec_close_device(req as *mut IORequest);
        exec_free_sys_object(ASOT_IOREQUEST, req);
        exec_delete_msg_port(port);
    }

    if err != 0 {
        say(b"FAIL: CMD_WRITE returned an error\n\0");
        return 4;
    }

    say(b"\n  => PASS: tone played through ahi.device from Rust\n\0");
    say(b"========================================\n\0");
    0
}

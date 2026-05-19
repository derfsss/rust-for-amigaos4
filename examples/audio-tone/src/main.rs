//
// AmigaOS 4 audio-tone (smoke test) - opens ahi.device through
// exec.library's standard CreateMsgPort + AllocSysObject(IOREQUEST)
// + OpenDevice sequence, prints OpenDevice's return code, and closes
// everything cleanly.
//
// Demonstrates: the exec-library wrappers for device I/O setup, and
// confirms ahi.device can be opened from Rust. Actual audio playback
// (AHI_AllocAudio, SetSound, sample synthesis) needs more glue and is
// left as a follow-up.
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

// AHIRequest base size (devices/ahi.h: struct AHIRequest). 56 bytes is
// the standard size on PPC OS4; ahi.device validates this internally.
const AHIREQUEST_SIZE: u32 = 56;

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

    if rc == 0 {
        say(b"  ahi.device opened cleanly\n\0");
        unsafe { exec_close_device(req as *mut IORequest); }
    } else {
        say(b"  ahi.device not available on this image\n\0");
    }

    unsafe {
        exec_free_sys_object(ASOT_IOREQUEST, req);
        exec_delete_msg_port(port);
    }

    if rc != 0 {
        say(b"FAIL: OpenDevice returned non-zero\n\0");
        return 3;
    }

    say(b"\n  => PASS: ahi.device open + close round-trip via the binding\n\0");
    say(b"========================================\n\0");
    0
}

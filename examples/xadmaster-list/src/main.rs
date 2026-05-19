//
// AmigaOS 4 xadmaster-list - walks xadmaster.library's client chain
// and prints the name of every registered archiver/filesystem
// decompressor.
//
// Demonstrates: the xadmaster wrapper, IFace dispatch via libauto-
// provided IxadMaster, and reading a linked-list-of-foreign-structs
// pattern from Rust.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_sys::wrappers::xadmaster::xadmaster_xad_get_client_info;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])           { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

// xadClient layout (libraries/xadmaster.h):
//   xc_Next         : *xadClient   (offset 0,  4 bytes)
//   xc_Version      : UWORD        (offset 4,  2)
//   xc_MasterVer    : UWORD        (offset 6,  2)
//   xc_ClientVer    : UWORD        (offset 8,  2)
//   xc_ClientRev    : UWORD        (offset 10, 2)
//   xc_RecogSize    : xadSize/u32  (offset 12, 4)
//   xc_Flags        : ULONG        (offset 16, 4)
//   xc_Identifier   : ULONG        (offset 20, 4)
//   xc_ArchiverName : STRPTR       (offset 24, 4)
const XC_NEXT_OFFSET:           usize = 0;
const XC_CLIENT_VERSION_OFFSET: usize = 8;
const XC_FLAGS_OFFSET:          usize = 16;
const XC_ARCHIVER_NAME_OFFSET:  usize = 24;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - xadmaster-list\n\0");
    say(b"========================================\n\0");

    // xadmaster.library + IxadMaster are auto-opened by -lauto; the
    // first call into the IFace forces the open. No manual setup.
    let head = unsafe { xadmaster_xad_get_client_info() };
    if head.is_null() {
        say(b"FAIL: xadGetClientInfo returned NULL\n\0");
        return 1;
    }

    let mut node = head as *const u8;
    let mut count: u32 = 0;
    while !node.is_null() {
        let name_ptr = unsafe {
            let p = node.add(XC_ARCHIVER_NAME_OFFSET) as *const u32;
            p.read_unaligned()
        };
        let client_ver = unsafe {
            let p = node.add(XC_CLIENT_VERSION_OFFSET) as *const u16;
            p.read_unaligned()
        };
        let flags = unsafe {
            let p = node.add(XC_FLAGS_OFFSET) as *const u32;
            p.read_unaligned()
        };

        if name_ptr != 0 {
            say(b"  - \0");
            unsafe { amiga_debug_str(name_ptr as *const u8); }
            unsafe {
                amiga_debug_fmt_u32x2(
                    b"   v%lu  flags=0x%lx\n\0".as_ptr(),
                    client_ver as u32,
                    flags,
                );
            }
        }
        count += 1;

        let next = unsafe {
            let p = node.add(XC_NEXT_OFFSET) as *const u32;
            p.read_unaligned()
        };
        if next == 0 || count > 1024 {
            break;
        }
        node = next as *const u8;
    }

    say_d(b"\n  Total clients walked              = \0", count);
    say(b"  => PASS: xadmaster client chain enumerated\n\0");
    say(b"========================================\n\0");
    0
}

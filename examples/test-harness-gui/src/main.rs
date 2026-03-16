//
// AmigaOS 4 Target-Side GUI/Screen Test Harness
//
// Tests GUI and screen operations. May not work in headless QEMU.
// Linked with clib4 (-mcrt=clib4) and -lauto (auto-open libraries).
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::tag::TagListBuilder;
use amigaos4::screen::PubScreen;
use amigaos4::window::*;
use amigaos4::gfx::DrawContext;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// Test framework
// ---------------------------------------------------------------------------

fn run_test(name: &[u8], test_fn: fn() -> bool) -> bool {
    unsafe {
        amiga_debug_str(b"  test \0".as_ptr());
        amiga_debug_str(name.as_ptr());
        amiga_debug_str(b" ... \0".as_ptr());
    }

    let passed = test_fn();

    unsafe {
        if passed {
            amiga_debug_str(b"PASS\n\0".as_ptr());
        } else {
            amiga_debug_str(b"FAIL\n\0".as_ptr());
        }
    }

    passed
}

// ---------------------------------------------------------------------------
// Sprint 4G: GUI/Screen Tests
// ---------------------------------------------------------------------------

fn test_pub_screen_lock_unlock() -> bool {
    // Lock the Workbench (default) screen, get DrawInfo, drop both
    let screen = match PubScreen::lock(None) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let _draw_info = match screen.draw_info() {
        Ok(di) => di,
        Err(_) => return false,
    };
    // draw_info dropped first (borrows screen), then screen dropped
    true
}

fn test_window_open_close() -> bool {
    let tags = TagListBuilder::new()
        .tag(WA_LEFT, 50)
        .tag(WA_TOP, 50)
        .tag(WA_WIDTH, 100)
        .tag(WA_HEIGHT, 80)
        .tag(WA_TITLE, b"Test\0".as_ptr() as u32)
        .tag(WA_IDCMP, IDCMP_CLOSEWINDOW)
        .tag(WA_CLOSE_GADGET, 1)
        .tag(WA_ACTIVATE, 0) // don't steal focus
        .build();
    let _win = match AmigaWindow::open(&tags) {
        Ok(w) => w,
        Err(_) => return false,
    };
    // Dropped immediately — close the window
    true
}

fn test_window_get_msg_none() -> bool {
    let tags = TagListBuilder::new()
        .tag(WA_LEFT, 50)
        .tag(WA_TOP, 50)
        .tag(WA_WIDTH, 100)
        .tag(WA_HEIGHT, 80)
        .tag(WA_TITLE, b"MsgTest\0".as_ptr() as u32)
        .tag(WA_IDCMP, IDCMP_CLOSEWINDOW)
        .tag(WA_CLOSE_GADGET, 1)
        .tag(WA_ACTIVATE, 0)
        .build();
    let win = match AmigaWindow::open(&tags) {
        Ok(w) => w,
        Err(_) => return false,
    };
    // No events should be pending immediately
    win.get_msg().is_none()
}

fn test_draw_context_basic() -> bool {
    let tags = TagListBuilder::new()
        .tag(WA_LEFT, 50)
        .tag(WA_TOP, 50)
        .tag(WA_WIDTH, 200)
        .tag(WA_HEIGHT, 150)
        .tag(WA_TITLE, b"DrawTest\0".as_ptr() as u32)
        .tag(WA_IDCMP, IDCMP_CLOSEWINDOW)
        .tag(WA_CLOSE_GADGET, 1)
        .tag(WA_ACTIVATE, 0)
        .build();
    let win = match AmigaWindow::open(&tags) {
        Ok(w) => w,
        Err(_) => return false,
    };
    let mut dc = unsafe { DrawContext::from_ptr(win.rast_port()) };
    dc.set_pen(1);
    dc.move_to(10, 10);
    dc.draw_to(100, 100);
    true
}

fn test_draw_context_text() -> bool {
    let tags = TagListBuilder::new()
        .tag(WA_LEFT, 50)
        .tag(WA_TOP, 50)
        .tag(WA_WIDTH, 300)
        .tag(WA_HEIGHT, 100)
        .tag(WA_TITLE, b"TextTest\0".as_ptr() as u32)
        .tag(WA_IDCMP, IDCMP_CLOSEWINDOW)
        .tag(WA_CLOSE_GADGET, 1)
        .tag(WA_ACTIVATE, 0)
        .build();
    let win = match AmigaWindow::open(&tags) {
        Ok(w) => w,
        Err(_) => return false,
    };
    let mut dc = unsafe { DrawContext::from_ptr(win.rast_port()) };
    dc.set_pen(1);
    dc.move_to(10, 30);
    dc.draw_text(b"Hello, Amiga!");
    let width = dc.text_width(b"Hello, Amiga!");
    width > 0
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        amiga_debug_str(b"========================================\n\0".as_ptr());
        amiga_debug_str(b"  Rust for AmigaOS 4 - GUI Test Harness\n\0".as_ptr());
        amiga_debug_str(b"========================================\n\0".as_ptr());
    }

    let mut passed: u32 = 0;
    let mut total: u32 = 0;

    let tests: &[(&[u8], fn() -> bool)] = &[
        (b"pub_screen_lock_unlock\0",   test_pub_screen_lock_unlock),
        (b"window_open_close\0",        test_window_open_close),
        (b"window_get_msg_none\0",      test_window_get_msg_none),
        (b"draw_context_basic\0",       test_draw_context_basic),
        (b"draw_context_text\0",        test_draw_context_text),
    ];

    for &(name, test_fn) in tests.iter() {
        total += 1;
        if run_test(name, test_fn) {
            passed += 1;
        }
    }

    unsafe {
        amiga_debug_str(b"----------------------------------------\n\0".as_ptr());
        amiga_debug_fmt_u32x2(
            b"%lu/%lu tests passed\n\0".as_ptr(),
            passed,
            total,
        );
        amiga_debug_str(b"========================================\n\0".as_ptr());
    }

    if passed == total { 0 } else { 1 }
}

//
// AmigaOS 4 Game Boy test — third-party Rust code on the toolchain.
//
// Embeds padme-core (a complete no_std Game Boy emulator from
// crates.io, pulled in like any cargo dependency) and runs Blargg's
// CPU-instruction test ROMs inside it. The ROM's own serial output is
// routed to the Amiga debug buffer; Blargg prints "Passed" when every
// opcode behaves exactly like real hardware.
//
// So a PASS here means: a substantial third-party Rust codebase,
// compiled unmodified for 32-bit big-endian PPC, executes an emulated
// Sharp LR35902 faithfully on AmigaOS.
//

#![no_std]
#![no_main]

extern crate alloc;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use padme_core::{AudioSpeaker, Pixel, Rom, Screen, SerialOutput, System};

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

/// AmigaOS shells scan the binary for a `$STACK:` cookie and launch it
/// with at least that much stack. padme's `System` is a large by-value
/// struct (the DMG framebuffer alone is ~92 KB), so its constructor
/// allocates an ~81 KB stack frame — a guaranteed overflow of the
/// default 64 KB shell stack (first run crashed with a DSI exactly
/// there). 2 MB gives plenty of headroom.
/// Kept alive by `-Wl,--undefined=STACK_COOKIE` in the Makefile, like
/// the library template's RomTag.
#[used]
#[no_mangle]
pub static STACK_COOKIE: [u8; 16] = *b"$STACK:2000000\n\0";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// Headless peripherals
// ---------------------------------------------------------------------------

/// No display — the test ROMs report over the link cable, not the LCD.
struct HeadlessScreen;

impl Screen for HeadlessScreen {
    fn set_pixel(&mut self, _pixel: &Pixel, _x: u8, _y: u8) {}
    fn update(&mut self) {}
}

/// No audio.
struct Mute;

impl AudioSpeaker for Mute {
    fn set_samples(&mut self, _left: f32, _right: f32) {}
}

/// Collects the ROM's serial output into a buffer shared with the
/// test driver (the `System` owns this value, so the driver keeps a
/// second handle); echoes complete lines to the Amiga debug buffer.
struct DebugSerial {
    all: Rc<RefCell<Vec<u8>>>,
    line: Vec<u8>,
}

impl DebugSerial {
    fn new(shared: Rc<RefCell<Vec<u8>>>) -> Self {
        Self { all: shared, line: Vec::new() }
    }
}

fn contains(buf: &RefCell<Vec<u8>>, needle: &[u8]) -> bool {
    buf.borrow().windows(needle.len()).any(|w| w == needle)
}

impl SerialOutput for DebugSerial {
    fn putchar(&mut self, ch: u8) {
        self.all.borrow_mut().push(ch);
        if ch == b'\n' {
            let line = String::from_utf8_lossy(&self.line);
            amigaos4::serial_println!("  gb| {}", line.trim_end());
            self.line.clear();
        } else {
            self.line.push(ch);
        }
    }
}

// ---------------------------------------------------------------------------
// Test driver
// ---------------------------------------------------------------------------

/// Hard cap on emulated frames: Blargg's full suite finishes in ~55
/// emulated seconds; 130s (at 60 fps) is comfortable margin.
const MAX_FRAMES: u32 = 130 * 60;

/// Run one ROM until its serial output says Passed/Failed (or the
/// frame cap is hit). Returns true on "Passed".
fn run_rom(name: &str, image: &'static [u8]) -> bool {
    amigaos4::serial_println!("  --- {} ({} KB) ---", name, image.len() / 1024);

    // padme's Rom borrows the buffer mutably (MBC RAM banking writes
    // go through it), so give it an owned copy.
    let mut buf: Vec<u8> = image.to_vec();
    let rom = match Rom::load(buf.as_mut_slice()) {
        Ok(r) => r,
        Err(_) => {
            amigaos4::serial_println!("  FAIL: Rom::load rejected {}", name);
            return false;
        }
    };

    let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    let mut gb = System::new(rom, HeadlessScreen, DebugSerial::new(output.clone()), Mute);

    let mut frames: u32 = 0;
    loop {
        gb.update_frame();
        frames += 1;

        // Probe the serial collector every 30 emulated frames.
        if frames % 30 == 0 {
            if contains(&output, b"Passed") {
                amigaos4::serial_println!("  {} => Passed ({} frames)", name, frames);
                return true;
            }
            if contains(&output, b"Failed") {
                amigaos4::serial_println!("  {} => FAILED (see gb| lines)", name);
                return false;
            }
        }

        if frames >= MAX_FRAMES {
            amigaos4::serial_println!("  {} => TIMEOUT after {} frames", name, frames);
            return false;
        }
    }
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("========================================");
    amigaos4::serial_println!("  Rust for AmigaOS 4 - Game Boy test");
    amigaos4::serial_println!("  (padme-core + Blargg cpu_instrs)");
    amigaos4::serial_println!("========================================");

    // Quick single test first (32 KB, ~3s emulated), then the full
    // 11-part suite (64 KB MBC1 multi-ROM, ~55s emulated).
    let quick = run_rom("06-ld r,r", include_bytes!("../roms/06-ld_r_r.gb"));
    if !quick {
        amigaos4::serial_println!("FAIL: quick ROM did not pass");
        return 1;
    }

    let full = run_rom("cpu_instrs (full suite)", include_bytes!("../roms/cpu_instrs.gb"));
    if !full {
        amigaos4::serial_println!("FAIL: full suite did not pass");
        return 2;
    }

    amigaos4::serial_println!("  => PASS: Blargg cpu_instrs on padme-core on AmigaOS");
    amigaos4::serial_println!("========================================");
    0
}

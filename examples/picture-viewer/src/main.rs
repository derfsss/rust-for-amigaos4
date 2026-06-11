//
// AmigaOS 4 picture-viewer (smoke test) - loads a picture file via
// the safe amigaos4::datatypes wrapper, queries its nominal
// dimensions, and lets RAII dispose the object.
//
// Demonstrates: DtPicture::load + dimensions() end-to-end (the safe
// layer over NewDTObjectA / GetDTAttrsA / DisposeDTObject).
//
// Note: this stops short of rendering the image to a window. That
// needs DTM_PROCLAYOUT + DTM_DRAW BOOPSI dispatch — reachable via
// DtPicture::as_ptr() with the amiga_do_method_* helpers.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4::amstr;
use amigaos4::datatypes::DtPicture;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("========================================");
    amigaos4::serial_println!("  Rust for AmigaOS 4 - picture-viewer");
    amigaos4::serial_println!("========================================");

    // A sample IFF brush that ships with every AmigaOS 4 install.
    let path = amstr!("SYS:Prefs/Presets/Patterns/Bricks/BricksBlue.brush");

    let pic = match DtPicture::load(path) {
        Ok(p) => p,
        Err(e) => {
            amigaos4::serial_println!("FAIL: DtPicture::load: {}", e);
            return 1;
        }
    };
    amigaos4::serial_println!("  Loaded: SYS:Prefs/Presets/Patterns/Bricks/BricksBlue.brush");

    let (w, h) = match pic.dimensions() {
        Ok(d) => d,
        Err(e) => {
            amigaos4::serial_println!("FAIL: dimensions: {}", e);
            return 2;
        }
    };
    amigaos4::serial_println!("  dimensions = {}x{}", w, h);

    if w == 0 || h == 0 {
        amigaos4::serial_println!("FAIL: dimensions came back zero");
        return 3;
    }

    // `pic` drops here -> DisposeDTObject + CloseLibrary.
    drop(pic);

    amigaos4::serial_println!("  => PASS: datatype load + attrs query + dispose round-trip ok");
    amigaos4::serial_println!("========================================");
    0
}

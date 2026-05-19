//
// AmigaOS 4 picture-viewer (smoke test) - loads a picture file via
// the datatypes wrapper, queries the BOOPSI attributes for the
// image's nominal width/height, prints them, then disposes the DT
// object.
//
// Demonstrates: the datatypes wrapper end-to-end, NewDTObjectA,
// GetDTAttrsA, and DisposeDTObject in concert.
//
// Note: this stops short of rendering the image to a window. That
// needs DTM_PROCLAYOUT + DTM_DRAW BOOPSI dispatch which is another
// example's worth of code; this one establishes that the load /
// query / dispose path works through the Rust binding.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_sys::wrappers::datatypes::{
    datatypes_new_dtobject_a, datatypes_get_dtattrs_a, datatypes_dispose_dtobject,
};
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])           { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

// DTA_ tag IDs from datatypes/datatypesclass.h.
// DTA_Dummy = TAG_USER + 0x1000 (= 0x80001000).
const DTA_DUMMY: u32         = TAG_USER + 0x1000;
const DTA_NOMINAL_VERT: u32  = DTA_DUMMY + 124;
const DTA_NOMINAL_HORIZ: u32 = DTA_DUMMY + 125;

// PDTA_ tag IDs from datatypes/pictureclass.h share the DTA_Dummy
// base (not a separate one).
const PDTA_REMAP: u32        = DTA_DUMMY + 211;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - picture-viewer\n\0");
    say(b"========================================\n\0");

    // Hard-coded path to a sample IFF brush that ships with every
    // AmigaOS 4 install.
    let path = b"SYS:Prefs/Presets/Patterns/Bricks/BricksBlue.brush\0";

    // Load the datatype object. PDTA_Remap=FALSE so picture.datatype
    // hands us truecolour data instead of trying to remap (we have no
    // target screen here).
    let mut tags: [TagItem; 2] = [
        TagItem { ti_Tag: PDTA_REMAP, ti_Data: 0 },
        TagItem { ti_Tag: TAG_DONE,   ti_Data: 0 },
    ];
    let obj = unsafe {
        datatypes_new_dtobject_a(path.as_ptr() as CONST_STRPTR, tags.as_mut_ptr() as *mut APTR)
    };
    if obj.is_null() {
        let err = unsafe { dos_io_err() };
        say_d(b"  IoErr after NewDTObjectA          = \0", err as u32);
        say(b"FAIL: NewDTObjectA returned NULL\n\0");
        return 1;
    }
    say(b"  Loaded: SYS:Prefs/Presets/Patterns/Bricks/BricksBlue.brush\n\0");

    // Query the nominal dimensions to confirm the picture parsed.
    let mut h: u32 = 0;
    let mut v: u32 = 0;
    let mut query: [TagItem; 3] = [
        TagItem { ti_Tag: DTA_NOMINAL_HORIZ, ti_Data: &mut h as *mut u32 as u32 },
        TagItem { ti_Tag: DTA_NOMINAL_VERT,  ti_Data: &mut v as *mut u32 as u32 },
        TagItem { ti_Tag: TAG_DONE,          ti_Data: 0 },
    ];
    let got = unsafe { datatypes_get_dtattrs_a(obj, query.as_mut_ptr() as *mut APTR) };
    say_d(b"  GetDTAttrsA hits                  = \0", got);
    say_d(b"  DTA_NominalHoriz                  = \0", h);
    say_d(b"  DTA_NominalVert                   = \0", v);

    unsafe { datatypes_dispose_dtobject(obj); }

    if h == 0 || v == 0 {
        say(b"FAIL: dimensions came back zero\n\0");
        return 2;
    }

    say(b"\n  => PASS: datatype load + attrs query + dispose round-trip ok\n\0");
    say(b"========================================\n\0");
    0
}

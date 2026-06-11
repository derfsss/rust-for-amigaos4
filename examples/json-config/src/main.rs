//
// AmigaOS 4 JSON demo — jansson from Rust.
//
// Links the SDK's clib4-built libjansson.a (Makefile: -ljansson) and
// parses a config document, reads typed fields, builds a new object,
// and serialises it back — the JSON round-trip every app with a
// config file needs.
//
// Same clib4-userland pattern as sqlite3-demo: extern "C" for the
// handful of functions used, one -l flag, no bindgen.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::ffi::c_void;
use core::panic::PanicInfo;
use amigaos4::amstr;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// jansson FFI
//
// json_decref/json_is_* are static-inline macros in jansson.h; the demo
// uses the exported functions only (json_delete frees an object whose
// refcount is 1 — true for everything we create here).
// ---------------------------------------------------------------------------

/// jansson's json_error_t: int line/column/position + 2 char arrays.
#[repr(C)]
struct JsonError {
    line: i32,
    column: i32,
    position: i32,
    source: [u8; 80],
    text: [u8; 160],
}

extern "C" {
    fn json_loads(input: *const u8, flags: usize, error: *mut JsonError) -> *mut c_void;
    fn json_object() -> *mut c_void;
    fn json_object_get(object: *const c_void, key: *const u8) -> *mut c_void;
    fn json_object_set_new(object: *mut c_void, key: *const u8, value: *mut c_void) -> i32;
    fn json_integer(value: i64) -> *mut c_void;
    fn json_string(value: *const u8) -> *mut c_void;
    fn json_integer_value(json: *const c_void) -> i64;
    fn json_string_value(json: *const c_void) -> *const u8;
    fn json_dumps(json: *const c_void, flags: usize) -> *mut u8;
    fn json_delete(json: *mut c_void);
    fn free(ptr: *mut c_void);
}

/// JSON_COMPACT — no whitespace in dumps output.
const JSON_COMPACT: usize = 0x20;

/// Read a C string into a Vec (capped).
unsafe fn cstr(p: *const u8) -> alloc::vec::Vec<u8> {
    let mut v = alloc::vec::Vec::new();
    if p.is_null() {
        return v;
    }
    let mut i = 0;
    while i < 1024 && *p.add(i) != 0 {
        v.push(*p.add(i));
        i += 1;
    }
    v
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("========================================");
    amigaos4::serial_println!("  Rust for AmigaOS 4 - JSON (jansson)");
    amigaos4::serial_println!("========================================");

    unsafe {
        // 1. Parse a config document.
        let doc = amstr!(r#"{"app":"AminetBrowser","width":640,"height":480,"depth":24}"#);
        let mut err = core::mem::zeroed::<JsonError>();
        let root = json_loads(doc.as_ptr(), 0, &mut err);
        if root.is_null() {
            amigaos4::serial_println!("FAIL: json_loads (line {})", err.line);
            return 1;
        }

        // 2. Read typed fields.
        let app = cstr(json_string_value(json_object_get(root, amstr!("app").as_ptr())));
        let width = json_integer_value(json_object_get(root, amstr!("width").as_ptr()));
        let height = json_integer_value(json_object_get(root, amstr!("height").as_ptr()));
        amigaos4::serial_println!(
            "  parsed: app={} size={}x{}",
            core::str::from_utf8(&app).unwrap_or("?"),
            width,
            height
        );
        if app != b"AminetBrowser" || width != 640 || height != 480 {
            amigaos4::serial_println!("FAIL: parsed values wrong");
            json_delete(root);
            return 2;
        }

        // 3. Build a new object and serialise it.
        let out = json_object();
        json_object_set_new(out, amstr!("machine").as_ptr(), json_string(amstr!("X5000").as_ptr()));
        json_object_set_new(out, amstr!("mhz").as_ptr(), json_integer(2000));
        let dumped_ptr = json_dumps(out, JSON_COMPACT) as *mut c_void;
        let dumped = cstr(dumped_ptr as *const u8);
        amigaos4::serial_println!("  dumped: {}", core::str::from_utf8(&dumped).unwrap_or("?"));

        let ok = dumped == br#"{"machine": "X5000", "mhz": 2000}"#.as_slice()
            || dumped == br#"{"machine":"X5000","mhz":2000}"#.as_slice();

        if !dumped_ptr.is_null() {
            free(dumped_ptr);
        }
        json_delete(out);
        json_delete(root);

        if !ok {
            amigaos4::serial_println!("FAIL: dumps mismatch");
            return 3;
        }
    }

    amigaos4::serial_println!("  => PASS: jansson parse + dumps round-trip from Rust");
    amigaos4::serial_println!("========================================");
    0
}

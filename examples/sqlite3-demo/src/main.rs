//
// AmigaOS 4 SQLite demo — the clib4 userland from Rust.
//
// Links the SDK's clib4-built libsqlite3.a (see Makefile: -lsqlite3)
// and drives it through plain extern "C" FFI: open an in-memory
// database, create a table, insert rows, and read them back with
// prepare/step/column.
//
// This is the pattern for the whole SDK/local/clib4 userland
// (sqlite, jansson, libpng, freetype, SDL2, ...): no bindgen, no
// build.rs — declare the functions you use, add the -l flag.
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
// sqlite3 FFI (the handful of calls this demo needs)
// ---------------------------------------------------------------------------

extern "C" {
    fn sqlite3_libversion() -> *const u8;
    fn sqlite3_open(filename: *const u8, db: *mut *mut c_void) -> i32;
    fn sqlite3_close(db: *mut c_void) -> i32;
    fn sqlite3_exec(
        db: *mut c_void,
        sql: *const u8,
        callback: *const c_void,
        arg: *mut c_void,
        errmsg: *mut *mut u8,
    ) -> i32;
    fn sqlite3_prepare_v2(
        db: *mut c_void,
        sql: *const u8,
        nbyte: i32,
        stmt: *mut *mut c_void,
        tail: *mut *const u8,
    ) -> i32;
    fn sqlite3_step(stmt: *mut c_void) -> i32;
    fn sqlite3_column_int(stmt: *mut c_void, col: i32) -> i32;
    fn sqlite3_column_text(stmt: *mut c_void, col: i32) -> *const u8;
    fn sqlite3_finalize(stmt: *mut c_void) -> i32;
}

const SQLITE_OK: i32 = 0;
const SQLITE_ROW: i32 = 100;
const SQLITE_DONE: i32 = 101;

/// Read a C string into a short Vec (capped).
unsafe fn cstr(p: *const u8) -> alloc::vec::Vec<u8> {
    let mut v = alloc::vec::Vec::new();
    if p.is_null() {
        return v;
    }
    let mut i = 0;
    while i < 256 && *p.add(i) != 0 {
        v.push(*p.add(i));
        i += 1;
    }
    v
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("========================================");
    amigaos4::serial_println!("  Rust for AmigaOS 4 - SQLite demo");
    amigaos4::serial_println!("========================================");

    unsafe {
        let ver = cstr(sqlite3_libversion());
        amigaos4::serial_println!("  sqlite version = {}", core::str::from_utf8(&ver).unwrap_or("?"));

        // 1. In-memory database.
        let mut db: *mut c_void = core::ptr::null_mut();
        if sqlite3_open(amstr!(":memory:").as_ptr(), &mut db) != SQLITE_OK || db.is_null() {
            amigaos4::serial_println!("FAIL: sqlite3_open");
            return 1;
        }

        // 2. Schema + rows in one exec batch.
        let sql = amstr!(
            "CREATE TABLE machines(id INTEGER PRIMARY KEY, name TEXT, mhz INTEGER);\
             INSERT INTO machines(name, mhz) VALUES('A1200', 14);\
             INSERT INTO machines(name, mhz) VALUES('X5000', 2000);\
             INSERT INTO machines(name, mhz) VALUES('Sam460', 1150);"
        );
        if sqlite3_exec(db, sql.as_ptr(), core::ptr::null(), core::ptr::null_mut(), core::ptr::null_mut()) != SQLITE_OK {
            amigaos4::serial_println!("FAIL: sqlite3_exec (schema/insert)");
            sqlite3_close(db);
            return 2;
        }
        amigaos4::serial_println!("  created table + 3 rows");

        // 3. Query back, fastest machine first.
        let query = amstr!("SELECT id, name, mhz FROM machines ORDER BY mhz DESC;");
        let mut stmt: *mut c_void = core::ptr::null_mut();
        if sqlite3_prepare_v2(db, query.as_ptr(), -1, &mut stmt, core::ptr::null_mut()) != SQLITE_OK {
            amigaos4::serial_println!("FAIL: sqlite3_prepare_v2");
            sqlite3_close(db);
            return 3;
        }

        let mut rows = 0;
        let mut first_name = alloc::vec::Vec::new();
        loop {
            match sqlite3_step(stmt) {
                SQLITE_ROW => {
                    let id = sqlite3_column_int(stmt, 0);
                    let name = cstr(sqlite3_column_text(stmt, 1));
                    let mhz = sqlite3_column_int(stmt, 2);
                    amigaos4::serial_println!(
                        "  row: id={} name={} mhz={}",
                        id,
                        core::str::from_utf8(&name).unwrap_or("?"),
                        mhz
                    );
                    if rows == 0 {
                        first_name = name;
                    }
                    rows += 1;
                }
                SQLITE_DONE => break,
                rc => {
                    amigaos4::serial_println!("FAIL: sqlite3_step rc={}", rc);
                    sqlite3_finalize(stmt);
                    sqlite3_close(db);
                    return 4;
                }
            }
        }
        sqlite3_finalize(stmt);
        sqlite3_close(db);

        if rows != 3 || first_name != b"X5000" {
            amigaos4::serial_println!("FAIL: expected 3 rows with X5000 first");
            return 5;
        }
    }

    amigaos4::serial_println!("  => PASS: SQLite in-memory round-trip from Rust");
    amigaos4::serial_println!("========================================");
    0
}

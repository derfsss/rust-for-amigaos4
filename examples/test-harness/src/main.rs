//
// AmigaOS 4 Target-Side Test Harness
//
// Exercises OS calls on QEMU and reports pass/fail via DebugPrintF.
// Linked with clib4 (-mcrt=clib4) and -lauto (auto-open libraries).
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::tag::TagListBuilder;
use amigaos4::mem::AmigaVec;
use amigaos4::port::AmigaMsgPort;
use amigaos4::time::Duration;
use amigaos4::fs;
use amigaos4::env;
use amigaos4::dos;
use amigaos4::io::Read;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// Test framework
// ---------------------------------------------------------------------------

/// Run a single test. Prints the test name and PASS/FAIL via DebugPrintF.
/// Returns true if the test passed.
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
// Original Tests
// ---------------------------------------------------------------------------

fn test_vec_alloc_push_pop() -> bool {
    let mut v: Vec<u32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    if v.len() != 3 {
        return false;
    }

    let popped = v.pop();
    if popped != Some(30) {
        return false;
    }

    if v.len() != 2 {
        return false;
    }

    // Verify remaining elements
    v[0] == 10 && v[1] == 20
}

fn test_string_format() -> bool {
    let s = String::from("Hello");
    if s.as_bytes() != b"Hello" {
        return false;
    }

    let formatted = alloc::format!("value={}", 42u32);
    if formatted.as_bytes() != b"value=42" {
        return false;
    }

    let combined = alloc::format!("{} {}", "Amiga", "OS4");
    combined.as_bytes() == b"Amiga OS4"
}

fn test_taglist_builder() -> bool {
    let tag_a: Tag = 0x8004_0001; // example user tag
    let tag_b: Tag = 0x8004_0002;

    let tags = TagListBuilder::new()
        .tag(tag_a, 100)
        .tag(tag_b, 200)
        .tag_if(false, tag_a, 999) // should become TAG_IGNORE
        .build();

    // Builder produces: [tag_a/100, tag_b/200, TAG_IGNORE/0, TAG_DONE/0]
    if tags.len() != 4 {
        return false;
    }

    if tags[0].ti_Tag != tag_a || tags[0].ti_Data != 100 {
        return false;
    }
    if tags[1].ti_Tag != tag_b || tags[1].ti_Data != 200 {
        return false;
    }
    // tag_if(false, ...) should produce TAG_IGNORE
    if tags[2].ti_Tag != TAG_IGNORE {
        return false;
    }
    // Last entry must be TAG_DONE
    tags[3].ti_Tag == TAG_DONE
}

fn test_amiga_vec() -> bool {
    let result = AmigaVec::alloc_cleared(256, MEMF_SHARED, 0xAA);
    match result {
        Ok(av) => {
            if av.len() != 256 {
                return false;
            }

            let slice = av.as_slice();

            // Verify all bytes were cleared to 0xAA
            for &b in slice.iter() {
                if b != 0xAA {
                    return false;
                }
            }

            true
            // av is dropped here, FreeVec called automatically
        }
        Err(_) => false,
    }
}

fn test_msg_port() -> bool {
    let result = AmigaMsgPort::new();
    match result {
        Ok(port) => {
            // Port pointer must be non-null (new() already checks, but verify)
            !port.as_ptr().is_null()
            // port is dropped here, DeleteMsgPort called automatically
        }
        Err(_) => false,
    }
}

fn test_duration_arithmetic() -> bool {
    let a = Duration::from_secs(2);
    let b = Duration::from_millis(500);

    // 2s + 500ms = 2500ms
    let sum = a + b;
    if sum.as_millis() != 2500 {
        return false;
    }

    // 2s - 500ms = 1500ms
    let diff = a - b;
    if diff.as_millis() != 1500 {
        return false;
    }

    // from_nanos round-trip
    let c = Duration::from_nanos(1_234_567_890);
    if c.as_secs() != 1 {
        return false;
    }
    if c.subsec_nanos() != 234_567_890 {
        return false;
    }

    // Zero
    Duration::ZERO.as_nanos() == 0
}

fn test_file_io() -> bool {
    let path = b"T:rust_test_harness.tmp\0";
    let test_data = b"AmigaOS4 Rust test data\n";

    // Write the file
    if fs::write_file(path, test_data).is_err() {
        return false;
    }

    // Read it back
    let read_back = match fs::read_to_vec(path) {
        Ok(data) => data,
        Err(_) => return false,
    };

    // Verify contents match
    if read_back.as_slice() != test_data {
        // Clean up before returning
        let _ = fs::remove_file(path);
        return false;
    }

    // Delete the temp file
    if fs::remove_file(path).is_err() {
        return false;
    }

    // Verify it's gone
    fs::metadata(path).is_err()
}

fn test_env_current_dir() -> bool {
    let result = env::current_dir();
    match result {
        Ok(cwd) => {
            // Current directory should be a non-empty path
            !cwd.is_empty()
        }
        Err(_) => false,
    }
}

// ---------------------------------------------------------------------------
// Sprint 4A: Core RAII Tests
// ---------------------------------------------------------------------------

fn test_amiga_vec_alloc_drop() -> bool {
    // Alloc + immediate drop, no crash
    let result = AmigaVec::alloc(128, MEMF_SHARED);
    match result {
        Ok(_av) => true, // dropped here
        Err(_) => false,
    }
}

fn test_amiga_vec_cleared_zeros() -> bool {
    let result = AmigaVec::alloc_cleared(64, MEMF_SHARED, 0x00);
    match result {
        Ok(av) => {
            let slice = av.as_slice();
            for &b in slice.iter() {
                if b != 0x00 {
                    return false;
                }
            }
            true
        }
        Err(_) => false,
    }
}

fn test_amiga_vec_multiple_cycle() -> bool {
    // 10x alloc/drop loop, no leak
    for _ in 0..10 {
        let result = AmigaVec::alloc(512, MEMF_SHARED);
        if result.is_err() {
            return false;
        }
        // dropped each iteration
    }
    true
}

fn test_amiga_vec_slice_write_read() -> bool {
    let result = AmigaVec::alloc(16, MEMF_SHARED);
    match result {
        Ok(mut av) => {
            let slice = av.as_mut_slice();
            for (i, b) in slice.iter_mut().enumerate() {
                *b = i as u8;
            }
            let slice = av.as_slice();
            for (i, &b) in slice.iter().enumerate() {
                if b != i as u8 {
                    return false;
                }
            }
            true
        }
        Err(_) => false,
    }
}

fn test_amiga_vec_len_is_empty() -> bool {
    let result = AmigaVec::alloc(32, MEMF_SHARED);
    match result {
        Ok(av) => {
            if av.len() != 32 {
                return false;
            }
            if av.is_empty() {
                return false;
            }
            true
        }
        Err(_) => false,
    }
}

fn test_msg_port_create_drop_cycle() -> bool {
    // 10x create/drop loop
    for _ in 0..10 {
        let result = AmigaMsgPort::new();
        if result.is_err() {
            return false;
        }
        // dropped each iteration
    }
    true
}

fn test_amiga_lock_shared() -> bool {
    // Lock T: directory, drop
    let result = dos::AmigaLock::shared(b"T:\0");
    match result {
        Ok(_lock) => true, // dropped here
        Err(_) => false,
    }
}

fn test_amiga_lock_name() -> bool {
    let result = dos::AmigaLock::shared(b"T:\0");
    match result {
        Ok(lock) => {
            match lock.name() {
                Ok(name) => !name.is_empty(),
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}

fn test_amiga_lock_parent() -> bool {
    // Lock T: and try to get its parent
    let result = dos::AmigaLock::shared(b"T:\0");
    match result {
        Ok(lock) => {
            // T: may or may not have a parent (depends on volume structure)
            // Just verify the call doesn't crash
            let _parent = lock.parent();
            true
        }
        Err(_) => false,
    }
}

fn test_amiga_lock_same_as() -> bool {
    let lock1 = match dos::AmigaLock::shared(b"T:\0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    let lock2 = match dos::AmigaLock::shared(b"T:\0") {
        Ok(l) => l,
        Err(_) => return false,
    };
    lock1.same_as(&lock2)
}

fn test_dir_scanner_enumerate() -> bool {
    let result = dos::DirScanner::open(b"T:\0");
    match result {
        Ok(mut scanner) => {
            // Just iterate until None — no crash
            let mut count = 0u32;
            while scanner.next().is_some() {
                count += 1;
                if count > 10000 {
                    break; // safety cap
                }
            }
            true
        }
        Err(_) => false,
    }
}

// ---------------------------------------------------------------------------
// Sprint 4B: File I/O Tests
// ---------------------------------------------------------------------------

fn test_file_open_nonexistent() -> bool {
    fs::File::open(b"T:__rust_nonexistent_file_xyz__\0").is_err()
}

fn test_file_read_eof() -> bool {
    let path = b"T:rust_test_eof.tmp\0";
    // Write an empty file
    if fs::write_file(path, b"").is_err() {
        return false;
    }
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => { let _ = fs::remove_file(path); return false; }
    };
    let mut buf = [0u8; 16];
    let n = match file.read(&mut buf) {
        Ok(n) => n,
        Err(_) => { let _ = fs::remove_file(path); return false; }
    };
    let _ = fs::remove_file(path);
    n == 0
}

fn test_file_write_empty() -> bool {
    let path = b"T:rust_test_empty.tmp\0";
    let result = fs::write_file(path, b"");
    let _ = fs::remove_file(path);
    result.is_ok()
}

fn test_file_metadata_size() -> bool {
    let path = b"T:rust_test_meta.tmp\0";
    let data = b"1234567890";
    if fs::write_file(path, data).is_err() {
        return false;
    }
    let ok = match fs::metadata(path) {
        Ok(m) => m.size() == 10,
        Err(_) => false,
    };
    let _ = fs::remove_file(path);
    ok
}

fn test_file_metadata_is_dir() -> bool {
    match fs::metadata(b"T:\0") {
        Ok(m) => m.is_dir(),
        Err(_) => false,
    }
}

fn test_create_dir_and_verify() -> bool {
    let path = b"T:rust_test_dir\0";
    // Clean up in case it exists from a previous run
    let _ = fs::remove_file(path);
    if fs::create_dir(path).is_err() {
        return false;
    }
    let ok = match fs::metadata(path) {
        Ok(m) => m.is_dir(),
        Err(_) => false,
    };
    // Clean up
    let _ = fs::remove_file(path);
    ok
}

fn test_remove_nonexistent() -> bool {
    fs::remove_file(b"T:__rust_nonexistent_remove_xyz__\0").is_err()
}

fn test_read_to_vec_4kb() -> bool {
    let path = b"T:rust_test_4kb.tmp\0";
    let data: Vec<u8> = (0..4096).map(|i| (i & 0xFF) as u8).collect();
    if fs::write_file(path, &data).is_err() {
        return false;
    }
    let read_back = match fs::read_to_vec(path) {
        Ok(d) => d,
        Err(_) => { let _ = fs::remove_file(path); return false; }
    };
    let _ = fs::remove_file(path);
    read_back == data
}

// ---------------------------------------------------------------------------
// Sprint 4C: Environment Tests
// ---------------------------------------------------------------------------

fn test_env_var_existing() -> bool {
    // PATH is typically set on AmigaOS
    match env::var(b"PATH\0") {
        Some(val) => !val.is_empty(),
        None => false,
    }
}

fn test_env_var_nonexistent() -> bool {
    env::var(b"RUST_NONEXISTENT_XYZ\0").is_none()
}

// ---------------------------------------------------------------------------
// Sprint 4D: Thread Tests
// ---------------------------------------------------------------------------

fn test_thread_spawn_join_u32() -> bool {
    let handle = match amigaos4::thread::spawn(|| 42u32) {
        Ok(h) => h,
        Err(_) => return false,
    };
    match handle.join() {
        Ok(val) => val == 42,
        Err(_) => false,
    }
}

fn test_thread_spawn_closure_capture() -> bool {
    let captured: u32 = 123;
    let handle = match amigaos4::thread::spawn(move || captured + 1) {
        Ok(h) => h,
        Err(_) => return false,
    };
    match handle.join() {
        Ok(val) => val == 124,
        Err(_) => false,
    }
}

fn test_thread_spawn_multiple() -> bool {
    let mut handles = Vec::new();
    for i in 0u32..5 {
        let handle = match amigaos4::thread::spawn(move || i * 10) {
            Ok(h) => h,
            Err(_) => return false,
        };
        handles.push(handle);
    }
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(val) => {
                if val != (i as u32) * 10 {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Sprint 4E: Async Executor Tests
// ---------------------------------------------------------------------------

fn test_executor_create_drop() -> bool {
    use amigaos4::async_rt::Executor;
    // Create + drop, signal bit freed
    match Executor::new() {
        Ok(_exec) => true, // dropped here
        Err(_) => false,
    }
}

fn test_executor_spawn_run() -> bool {
    use amigaos4::async_rt::Executor;
    let mut exec = match Executor::new() {
        Ok(e) => e,
        Err(_) => return false,
    };
    let ran = alloc::rc::Rc::new(core::cell::RefCell::new(false));
    let ran_clone = ran.clone();
    exec.spawn(async move {
        *ran_clone.borrow_mut() = true;
    });
    exec.run();
    let result = *ran.borrow();
    result
}

fn test_executor_block_on_value() -> bool {
    use amigaos4::async_rt::Executor;
    let mut exec = match Executor::new() {
        Ok(e) => e,
        Err(_) => return false,
    };
    let result: u32 = exec.block_on(async { 42 });
    result == 42
}

fn test_executor_multiple_tasks() -> bool {
    use amigaos4::async_rt::Executor;
    let mut exec = match Executor::new() {
        Ok(e) => e,
        Err(_) => return false,
    };
    let counter = alloc::rc::Rc::new(core::cell::RefCell::new(0u32));
    for _ in 0..3 {
        let c = counter.clone();
        exec.spawn(async move {
            *c.borrow_mut() += 1;
        });
    }
    exec.run();
    let result = *counter.borrow() == 3;
    result
}

// ---------------------------------------------------------------------------
// Sprint 4F: Serial/Debug Tests
// ---------------------------------------------------------------------------

fn test_serial_write_short() -> bool {
    use core::fmt::Write;
    write!(amigaos4::fmt::serial(), "Hi").is_ok()
}

fn test_serial_write_long() -> bool {
    use core::fmt::Write;
    // Write > 127 bytes (triggers chunking in flush_bytes)
    let long_msg = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz_\
                     ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz_end";
    write!(amigaos4::fmt::serial(), "{}", long_msg).is_ok()
}

fn test_serial_println_macro() -> bool {
    amigaos4::serial_println!("test {}", 42);
    true // if we get here, no crash
}

// ---------------------------------------------------------------------------
// Sprint 5C: Allocator Integration Tests
// ---------------------------------------------------------------------------

fn test_alloc_many_small() -> bool {
    // 1000 x 16-byte Vec allocations, drop all
    let mut vecs: Vec<Vec<u8>> = Vec::new();
    for _ in 0..1000 {
        let v: Vec<u8> = vec![0u8; 16];
        vecs.push(v);
    }
    vecs.len() == 1000
    // all dropped here
}

fn test_alloc_large() -> bool {
    // 1MB Vec allocation, no crash
    let v: Vec<u8> = vec![0u8; 1024 * 1024];
    v.len() == 1024 * 1024
}

fn test_alloc_alignment_32() -> bool {
    // repr(align(32)) Box, verify address alignment
    #[repr(align(32))]
    struct Aligned32 {
        _data: [u8; 64],
    }
    let boxed = alloc::boxed::Box::new(Aligned32 { _data: [0u8; 64] });
    let ptr = &*boxed as *const Aligned32 as usize;
    ptr % 32 == 0
}

fn test_alloc_zero_size() -> bool {
    // Vec::<u8>::new() works (zero initial capacity)
    let v: Vec<u8> = Vec::new();
    v.is_empty()
}

// ---------------------------------------------------------------------------
// Phase 9: Timer Tests
// ---------------------------------------------------------------------------

fn test_timer_open_close() -> bool {
    use amigaos4::timer::{AmigaTimer, TimerUnit};
    match AmigaTimer::open(TimerUnit::MicroHz) {
        Ok(_timer) => true, // dropped here — CloseDevice + cleanup
        Err(_) => false,
    }
}

fn test_timer_delay_short() -> bool {
    use amigaos4::timer::{AmigaTimer, TimerUnit, TimerVal};
    let mut timer = match AmigaTimer::open(TimerUnit::MicroHz) {
        Ok(t) => t,
        Err(_) => return false,
    };
    timer.delay(TimerVal::from_millis(50)).is_ok()
}

fn test_timer_get_sys_time() -> bool {
    use amigaos4::timer::{AmigaTimer, TimerUnit};
    let mut timer = match AmigaTimer::open(TimerUnit::MicroHz) {
        Ok(t) => t,
        Err(_) => return false,
    };
    let t = timer.get_sys_time();
    // System time should be nonzero (seconds since 1978)
    t.secs > 0
}

fn test_timer_get_up_time() -> bool {
    use amigaos4::timer;
    let t = timer::get_up_time();
    // Uptime should be nonzero
    t.secs > 0 || t.micros > 0
}

fn test_timer_open_vblank() -> bool {
    use amigaos4::timer::{AmigaTimer, TimerUnit};
    match AmigaTimer::open(TimerUnit::VBlank) {
        Ok(_timer) => true,
        Err(_) => false,
    }
}

// ---------------------------------------------------------------------------
// Phase 9: Clipboard Tests
// ---------------------------------------------------------------------------

fn test_clipboard_write_read() -> bool {
    use amigaos4::clipboard;
    let test_data = b"Rust clipboard test";

    // Write to clipboard
    if clipboard::write_text(test_data).is_err() {
        return false;
    }

    // Read back
    match clipboard::read_text() {
        Ok(data) => data == test_data,
        Err(_) => false,
    }
}

fn test_clipboard_write_empty() -> bool {
    use amigaos4::clipboard;
    // Writing empty text should succeed
    clipboard::write_text(b"").is_ok()
}

fn test_clipboard_read_after_write() -> bool {
    use amigaos4::clipboard;
    let msg = b"Hello from AmigaOS 4 Rust!";
    if clipboard::write_text(msg).is_err() {
        return false;
    }
    match clipboard::read_text() {
        Ok(data) => {
            if data.len() < msg.len() {
                return false;
            }
            &data[..msg.len()] == msg
        }
        Err(_) => false,
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        amiga_debug_str(b"========================================\n\0".as_ptr());
        amiga_debug_str(b"  Rust for AmigaOS 4 - Test Harness\n\0".as_ptr());
        amiga_debug_str(b"========================================\n\0".as_ptr());
    }

    let mut passed: u32 = 0;
    let mut total: u32 = 0;

    let tests: &[(&[u8], fn() -> bool)] = &[
        // Original tests
        (b"vec_alloc_push_pop\0",     test_vec_alloc_push_pop),
        (b"string_format\0",          test_string_format),
        (b"taglist_builder\0",        test_taglist_builder),
        (b"amiga_vec\0",              test_amiga_vec),
        (b"msg_port\0",              test_msg_port),
        (b"duration_arithmetic\0",    test_duration_arithmetic),
        (b"file_io\0",               test_file_io),
        (b"env_current_dir\0",       test_env_current_dir),
        // Sprint 4A: Core RAII
        (b"amiga_vec_alloc_drop\0",           test_amiga_vec_alloc_drop),
        (b"amiga_vec_cleared_zeros\0",        test_amiga_vec_cleared_zeros),
        (b"amiga_vec_multiple_cycle\0",       test_amiga_vec_multiple_cycle),
        (b"amiga_vec_slice_write_read\0",     test_amiga_vec_slice_write_read),
        (b"amiga_vec_len_is_empty\0",         test_amiga_vec_len_is_empty),
        (b"msg_port_create_drop_cycle\0",     test_msg_port_create_drop_cycle),
        (b"amiga_lock_shared\0",              test_amiga_lock_shared),
        (b"amiga_lock_name\0",                test_amiga_lock_name),
        (b"amiga_lock_parent\0",              test_amiga_lock_parent),
        (b"amiga_lock_same_as\0",             test_amiga_lock_same_as),
        (b"dir_scanner_enumerate\0",          test_dir_scanner_enumerate),
        // Sprint 4B: File I/O
        (b"file_open_nonexistent\0",          test_file_open_nonexistent),
        (b"file_read_eof\0",                  test_file_read_eof),
        (b"file_write_empty\0",               test_file_write_empty),
        (b"file_metadata_size\0",             test_file_metadata_size),
        (b"file_metadata_is_dir\0",           test_file_metadata_is_dir),
        (b"create_dir_and_verify\0",          test_create_dir_and_verify),
        (b"remove_nonexistent\0",             test_remove_nonexistent),
        (b"read_to_vec_4kb\0",               test_read_to_vec_4kb),
        // Sprint 4C: Environment
        (b"env_var_existing\0",               test_env_var_existing),
        (b"env_var_nonexistent\0",            test_env_var_nonexistent),
        // Sprint 4D: Threads
        (b"thread_spawn_join_u32\0",          test_thread_spawn_join_u32),
        (b"thread_spawn_closure_capture\0",   test_thread_spawn_closure_capture),
        (b"thread_spawn_multiple\0",          test_thread_spawn_multiple),
        // Sprint 4E: Async Executor
        (b"executor_create_drop\0",           test_executor_create_drop),
        (b"executor_spawn_run\0",             test_executor_spawn_run),
        (b"executor_block_on_value\0",        test_executor_block_on_value),
        (b"executor_multiple_tasks\0",        test_executor_multiple_tasks),
        // Sprint 4F: Serial/Debug
        (b"serial_write_short\0",             test_serial_write_short),
        (b"serial_write_long\0",              test_serial_write_long),
        (b"serial_println_macro\0",           test_serial_println_macro),
        // Sprint 5C: Allocator Integration
        (b"alloc_many_small\0",               test_alloc_many_small),
        (b"alloc_large\0",                    test_alloc_large),
        (b"alloc_alignment_32\0",             test_alloc_alignment_32),
        (b"alloc_zero_size\0",                test_alloc_zero_size),
        // Phase 9: Timer
        (b"timer_open_close\0",              test_timer_open_close),
        (b"timer_delay_short\0",             test_timer_delay_short),
        (b"timer_get_sys_time\0",            test_timer_get_sys_time),
        (b"timer_get_up_time\0",             test_timer_get_up_time),
        (b"timer_open_vblank\0",             test_timer_open_vblank),
        // Phase 9: Clipboard
        (b"clipboard_write_read\0",          test_clipboard_write_read),
        (b"clipboard_write_empty\0",         test_clipboard_write_empty),
        (b"clipboard_read_after_write\0",    test_clipboard_read_after_write),
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

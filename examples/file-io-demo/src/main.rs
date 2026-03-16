//
// File I/O Demo — AmigaOS 4 + Rust + clib4
//
// Demonstrates the amigaos4::fs module: creating, writing, reading,
// querying metadata, and deleting files and directories.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::fs;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== File I/O Demo ===");

    let path = b"T:rust_demo.txt\0";

    // --- Write a file ---
    amigaos4::serial_println!("Creating file T:rust_demo.txt ...");
    let content = b"Hello from Rust on AmigaOS 4!\nLine two of the demo file.\nLine three: the end.\n";
    if let Err(e) = fs::write_file(path, content) {
        amigaos4::serial_println!("ERROR: failed to write file: {:?}", e);
        return 1;
    }
    amigaos4::serial_println!("File written ({} bytes).", content.len());

    // --- Read the file back ---
    amigaos4::serial_println!("Reading file back ...");
    match fs::read_to_vec(path) {
        Ok(data) => {
            amigaos4::serial_println!("Read {} bytes.", data.len());
            // Print the contents (it's valid UTF-8 since we wrote it)
            if let Ok(text) = core::str::from_utf8(&data) {
                amigaos4::serial_println!("Contents:\n{}", text);
            } else {
                amigaos4::serial_println!("(contents are not valid UTF-8)");
            }
        }
        Err(e) => {
            amigaos4::serial_println!("ERROR: failed to read file: {:?}", e);
            return 1;
        }
    }

    // --- Query metadata ---
    amigaos4::serial_println!("Querying metadata ...");
    match fs::metadata(path) {
        Ok(meta) => {
            amigaos4::serial_println!("  Size:   {} bytes", meta.size());
            amigaos4::serial_println!("  Is dir: {}", meta.is_dir());
        }
        Err(e) => {
            amigaos4::serial_println!("ERROR: failed to stat file: {:?}", e);
            return 1;
        }
    }

    // --- Delete the file ---
    amigaos4::serial_println!("Deleting T:rust_demo.txt ...");
    if let Err(e) = fs::remove_file(path) {
        amigaos4::serial_println!("ERROR: failed to delete file: {:?}", e);
        return 1;
    }
    amigaos4::serial_println!("File deleted.");

    // --- Create a directory ---
    let dir_path = b"T:rust_demo_dir\0";
    amigaos4::serial_println!("Creating directory T:rust_demo_dir ...");
    if let Err(e) = fs::create_dir(dir_path) {
        amigaos4::serial_println!("ERROR: failed to create directory: {:?}", e);
        return 1;
    }

    // --- Verify it is a directory ---
    match fs::metadata(dir_path) {
        Ok(meta) => {
            amigaos4::serial_println!("  Is dir: {}", meta.is_dir());
        }
        Err(e) => {
            amigaos4::serial_println!("ERROR: failed to stat directory: {:?}", e);
            return 1;
        }
    }

    // --- Remove the directory ---
    // Note: fs::remove_file maps to unlink(), which may not remove directories
    // on all systems. On AmigaOS 4 / clib4 this typically works for empty dirs.
    amigaos4::serial_println!("Removing directory T:rust_demo_dir ...");
    if let Err(e) = fs::remove_file(dir_path) {
        amigaos4::serial_println!("WARNING: remove_file on directory failed: {:?}", e);
        amigaos4::serial_println!("(unlink may not work for directories on this system)");
    } else {
        amigaos4::serial_println!("Directory removed.");
    }

    amigaos4::serial_println!("=== File I/O Demo complete ===");
    0
}

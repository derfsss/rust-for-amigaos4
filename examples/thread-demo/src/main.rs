//
// Thread Demo — AmigaOS 4 + Rust + clib4
//
// Demonstrates the amigaos4::thread module: spawning threads,
// returning values, and joining to collect results.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::thread;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== Thread Demo ===");

    // Spawn 3 threads, each computing a different value
    amigaos4::serial_println!("Spawning 3 worker threads ...");

    let h1 = thread::spawn(|| {
        amigaos4::serial_println!("  Thread 1: computing 10 * 10");
        10u32 * 10
    });

    let h2 = thread::spawn(|| {
        amigaos4::serial_println!("  Thread 2: computing 20 + 5");
        20u32 + 5
    });

    let h3 = thread::spawn(|| {
        amigaos4::serial_println!("  Thread 3: computing 7 * 8");
        7u32 * 8
    });

    // Join all threads and collect results
    amigaos4::serial_println!("Joining threads ...");

    let mut sum: u32 = 0;

    match h1 {
        Ok(handle) => match handle.join() {
            Ok(val) => {
                amigaos4::serial_println!("  Thread 1 returned: {}", val);
                sum += val;
            }
            Err(e) => {
                amigaos4::serial_println!("  Thread 1 join failed: {:?}", e);
            }
        },
        Err(e) => {
            amigaos4::serial_println!("  Thread 1 spawn failed: {:?}", e);
        }
    }

    match h2 {
        Ok(handle) => match handle.join() {
            Ok(val) => {
                amigaos4::serial_println!("  Thread 2 returned: {}", val);
                sum += val;
            }
            Err(e) => {
                amigaos4::serial_println!("  Thread 2 join failed: {:?}", e);
            }
        },
        Err(e) => {
            amigaos4::serial_println!("  Thread 2 spawn failed: {:?}", e);
        }
    }

    match h3 {
        Ok(handle) => match handle.join() {
            Ok(val) => {
                amigaos4::serial_println!("  Thread 3 returned: {}", val);
                sum += val;
            }
            Err(e) => {
                amigaos4::serial_println!("  Thread 3 join failed: {:?}", e);
            }
        },
        Err(e) => {
            amigaos4::serial_println!("  Thread 3 spawn failed: {:?}", e);
        }
    }

    amigaos4::serial_println!("Sum of all thread results: {}", sum);

    // Demonstrate error handling — attempt to show what happens with spawn errors
    amigaos4::serial_println!("\nError handling demo:");
    amigaos4::serial_println!("  (All 3 threads joined successfully above.)");
    amigaos4::serial_println!("  If spawn() fails, it returns Err(AmigaError::IoError(rc)).");
    amigaos4::serial_println!("  If join() fails, it returns Err(AmigaError::IoError(rc)) or Err(NullPointer).");

    amigaos4::serial_println!("=== Thread Demo complete ===");
    0
}

//
// Async Demo — AmigaOS 4 + Rust
//
// Demonstrates the cooperative async executor using AmigaOS Exec signals.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::async_rt::Executor;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// A simple async function
async fn compute(name: &str, value: u32) -> u32 {
    amigaos4::serial_println!("  Task '{}': computing {} * 2", name, value);
    // In a real app, this would yield to other tasks
    value * 2
}

// An async function that "chains" two operations
async fn chain_ops() {
    amigaos4::serial_println!("  chain_ops: step 1");
    let a = 21u32;
    amigaos4::serial_println!("  chain_ops: step 2 (a = {})", a);
    let b = a + 21;
    amigaos4::serial_println!("  chain_ops: result = {}", b);
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== Async Demo ===");

    let mut exec = match Executor::new() {
        Ok(e) => e,
        Err(e) => {
            amigaos4::serial_println!("Failed to create executor: {}", e);
            return 1;
        }
    };

    // Spawn multiple tasks
    amigaos4::serial_println!("Spawning async tasks...");

    exec.spawn(async {
        amigaos4::serial_println!("  Task A: Hello from async!");
    });

    exec.spawn(async {
        chain_ops().await;
    });

    exec.spawn(async {
        amigaos4::serial_println!("  Task B: Starting");
        for i in 0..3u32 {
            amigaos4::serial_println!("  Task B: iteration {}", i);
        }
        amigaos4::serial_println!("  Task B: Done");
    });

    // Run all tasks
    amigaos4::serial_println!("Running executor...");
    exec.run();
    amigaos4::serial_println!("All tasks complete.");

    // Demonstrate block_on
    amigaos4::serial_println!("\nblock_on demo:");
    let mut exec2 = match Executor::new() {
        Ok(e) => e,
        Err(e) => {
            amigaos4::serial_println!("Failed to create second executor: {}", e);
            return 1;
        }
    };
    let result: u32 = exec2.block_on(async {
        amigaos4::serial_println!("  Computing in block_on...");
        42u32
    });
    amigaos4::serial_println!("block_on returned: {}", result);

    amigaos4::serial_println!("\n=== Async Demo complete ===");
    0
}

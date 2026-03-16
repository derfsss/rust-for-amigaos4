//
// Timer Demo — AmigaOS 4 + Rust + clib4
//
// Demonstrates the amigaos4::time module: Instant, Duration,
// elapsed timing, and Duration arithmetic.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::time::{Instant, Duration};

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== Timer Demo ===");

    // --- Measure elapsed time for busy work ---
    amigaos4::serial_println!("Starting busy-work benchmark ...");
    let start = Instant::now();

    // Busy work: build and drop a Vec 1000 times
    for _ in 0..1000 {
        let v = alloc::vec![0u8; 256];
        // Prevent the compiler from optimising this away
        core::hint::black_box(&v);
        drop(v);
    }

    let elapsed = start.elapsed();
    amigaos4::serial_println!(
        "Busy work took: {} secs, {} ms, {} ns",
        elapsed.as_secs(),
        elapsed.as_millis(),
        elapsed.as_nanos()
    );

    // --- Duration construction ---
    amigaos4::serial_println!("\nDuration construction:");

    let d1 = Duration::from_secs(2);
    amigaos4::serial_println!(
        "  from_secs(2):   {} s, {} ms, {} ns",
        d1.as_secs(),
        d1.as_millis(),
        d1.as_nanos()
    );

    let d2 = Duration::from_millis(1500);
    amigaos4::serial_println!(
        "  from_millis(1500): {} s, {} ms, {} ns",
        d2.as_secs(),
        d2.as_millis(),
        d2.as_nanos()
    );

    let d3 = Duration::from_nanos(500_000_000);
    amigaos4::serial_println!(
        "  from_nanos(500_000_000): {} s, {} ms, {} ns",
        d3.as_secs(),
        d3.as_millis(),
        d3.as_nanos()
    );

    // --- Duration arithmetic ---
    amigaos4::serial_println!("\nDuration arithmetic:");

    let sum = d1 + d2;
    amigaos4::serial_println!(
        "  2s + 1500ms = {} ms",
        sum.as_millis()
    );

    let diff = d1 - d2;
    amigaos4::serial_println!(
        "  2s - 1500ms = {} ms",
        diff.as_millis()
    );

    // Subtraction saturates at zero
    let sat = d2 - d1;
    amigaos4::serial_println!(
        "  1500ms - 2s = {} ms (saturates at zero)",
        sat.as_millis()
    );

    // --- Duration::ZERO ---
    amigaos4::serial_println!("\nDuration::ZERO:");
    let zero = Duration::ZERO;
    amigaos4::serial_println!(
        "  ZERO: {} s, {} ms, {} ns",
        zero.as_secs(),
        zero.as_millis(),
        zero.as_nanos()
    );

    amigaos4::serial_println!("=== Timer Demo complete ===");
    0
}

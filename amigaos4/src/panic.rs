//! Reusable panic handler that prints diagnostics to the serial debug output.
//!
//! # Usage
//!
//! In your crate root:
//!
//! ```ignore
//! use core::panic::PanicInfo;
//!
//! #[panic_handler]
//! fn panic(info: &PanicInfo) -> ! {
//!     amigaos4::panic::default_panic_handler(info)
//! }
//! ```

use core::fmt::Write;
use core::panic::PanicInfo;
use crate::fmt::Serial;

/// Default panic handler -- prints panic details to the serial debug line,
/// then enters an infinite spin loop.
///
/// Output format:
/// - `PANIC at file.rs:42: the panic message`
/// - `PANIC at file.rs:42` (if no message)
/// - `PANIC: the panic message` (if no location)
/// - `PANIC` (if neither)
pub fn default_panic_handler(info: &PanicInfo) -> ! {
    let mut s = Serial;

    let _ = s.write_str("PANIC");

    if let Some(loc) = info.location() {
        let _ = write!(s, " at {}:{}", loc.file(), loc.line());
    }

    let msg = info.message();
    if let Some(literal) = msg.as_str() {
        // Fast path: the panic message is a plain string literal.
        if !literal.is_empty() {
            let _ = write!(s, ": {}", literal);
        }
    } else {
        // The message contains format arguments -- render it.
        let _ = write!(s, ": {}", msg);
    }

    let _ = s.write_str("\n");

    loop {
        core::hint::spin_loop();
    }
}

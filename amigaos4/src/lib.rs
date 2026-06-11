//! Safe Rust wrappers and std-like modules for AmigaOS 4.1.
//!
//! # Panic Handler
//!
//! Use the provided default panic handler in your crate root:
//!
//! ```ignore
//! #[panic_handler]
//! fn panic(info: &core::panic::PanicInfo) -> ! {
//!     amigaos4::panic::default_panic_handler(info)
//! }
//! ```
//!
//! # Debug Output
//!
//! ```ignore
//! amigaos4::serial_println!("Hello from Rust!");
//! amigaos4::serial_println!("value = {}", 42);
//! ```
//!
//! # Features
//!
//! - `app` (default) — enables `fs`, `time`, `env`, `thread`, `net` modules (requires clib4)
//! - `driver` — minimal set for device drivers (no clib4 dependency)

#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate alloc;

// Always available (pure Rust or Exec-only, no clib4 dependency):
pub mod asl;
pub mod cstr;
pub mod error;
pub mod tag;
pub mod mem;
pub mod port;
pub mod screen;
pub mod boopsi;
pub mod window;
pub mod gfx;
pub mod requester;
pub mod reaction;
pub mod io;
pub mod fmt;
pub mod menu;
pub mod panic;
pub mod dos;
pub mod locale;
pub mod async_rt;
pub mod timer;
pub mod clipboard;

// Internal: pure parsing helpers for the network modules. Always compiled
// (the modules using it are PPC-only) so its tests run on host cargo test.
pub(crate) mod parse;

// Internal: RAII OpenLibrary/GetInterface for runtime-opened interfaces.
pub(crate) mod iface;

// Application-only (require clib4 POSIX functions). Doubly gated: the
// user must opt in via the feature *and* the build must be for the PPC
// target — these modules wrap clib4 symbols that only exist at link
// time inside the AmigaOS Docker toolchain. Host builds (cargo test on
// x86_64) skip them entirely.
//
// `time` is the exception: it stays available on the host so that the
// pure-Rust `Duration` arithmetic remains unit-testable. The PPC-only
// pieces (the `clock_gettime` extern and `Instant::now`) are gated
// inside `time.rs`.
#[cfg(all(feature = "fs", target_arch = "powerpc"))]
pub mod fs;
#[cfg(feature = "time")]
pub mod time;
#[cfg(all(feature = "env", target_arch = "powerpc"))]
pub mod env;
#[cfg(all(feature = "thread", target_arch = "powerpc"))]
pub mod thread;
#[cfg(all(feature = "net", target_arch = "powerpc"))]
pub mod net;
#[cfg(all(feature = "net", target_arch = "powerpc"))]
pub mod dns;
#[cfg(all(feature = "net", target_arch = "powerpc"))]
pub mod http;

pub use error::{AmigaError, Result};

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
pub mod panic;
pub mod dos;
pub mod locale;
pub mod async_rt;
pub mod timer;
pub mod clipboard;

// Application-only (require clib4 POSIX functions):
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "env")]
pub mod env;
#[cfg(feature = "thread")]
pub mod thread;
#[cfg(feature = "net")]
pub mod net;
#[cfg(feature = "net")]
pub mod dns;
#[cfg(feature = "net")]
pub mod http;

pub use error::{AmigaError, Result};

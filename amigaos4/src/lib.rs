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
pub mod io;
pub mod fmt;

// Application-only (require clib4 POSIX functions):
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "env")]
pub mod env;
#[cfg(feature = "thread")]
pub mod thread;

pub use error::{AmigaError, Result};

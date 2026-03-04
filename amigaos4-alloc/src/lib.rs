//! amigaos4-alloc — Global allocator backends for Rust-on-AmigaOS
//!
//! Two backends, selectable by Cargo feature:
//!
//! - `clib4` (default) — wraps clib4's malloc/free. For normal applications
//!   linked with `-mcrt=clib4`.
//! - `exec` — wraps IExec->AllocVecTagList/FreeVec. For device drivers and
//!   handlers built with `-nostartfiles -nodefaultlibs`.
//!
//! Usage (application):
//! ```toml
//! amigaos4-alloc = { path = "../amigaos4-alloc" }  # default = clib4
//! ```
//!
//! Usage (driver):
//! ```toml
//! amigaos4-alloc = { path = "../amigaos4-alloc", default-features = false, features = ["exec"] }
//! ```
//!
//! Then in your binary crate:
//! ```ignore
//! #[global_allocator]
//! static ALLOCATOR: amigaos4_alloc::Clib4Allocator = amigaos4_alloc::Clib4Allocator;
//! // or
//! #[global_allocator]
//! static ALLOCATOR: amigaos4_alloc::ExecAllocator = amigaos4_alloc::ExecAllocator;
//! ```

#![no_std]

#[cfg(feature = "clib4")]
mod clib4_alloc;
#[cfg(feature = "clib4")]
pub use clib4_alloc::Clib4Allocator;

#[cfg(feature = "exec")]
mod exec_alloc;
#[cfg(feature = "exec")]
pub use exec_alloc::ExecAllocator;

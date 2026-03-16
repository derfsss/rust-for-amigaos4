# no_std Ecosystem Guide for Rust on AmigaOS 4

This guide documents which popular `no_std` crates work with the Rust for AmigaOS 4
toolchain, how to integrate them, and workarounds for limitations such as the
unavailability of proc macros during cross-compilation.

---

## Compatible no_std Crates

Every crate listed below has been selected because it compiles for a bare-metal
`powerpc-unknown-none` target (our LLVM target triple) or because it explicitly
supports `no_std` with an optional `alloc` feature.  Our build environment
provides `core` and `alloc` but **not** `std`.

### 1. Data Structures

#### heapless — fixed-capacity collections

Fixed-size `Vec`, `String`, `Deque`, and maps that live on the stack or in a
static.  No heap allocation required; ideal for interrupt-safe or
memory-constrained AmigaOS programs.

```toml
[dependencies]
heapless = "0.8"
```

* `default-features = false` — not needed; `heapless` is `no_std` by default.
* `features = ["alloc"]` — not needed.
* PPC notes — pure Rust, no platform-specific code.  Works out of the box.

#### arrayvec — stack-allocated vectors

Similar concept to `heapless` but with a slightly different API.  Provides
`ArrayVec` and `ArrayString`.

```toml
[dependencies]
arrayvec = { version = "0.7", default-features = false }
```

* `default-features = false` — **required**; the default feature set enables `std`.
* `features = ["alloc"]` — not needed.
* PPC notes — pure Rust.  No issues.

---

### 2. Serialization

#### serde — the standard serialization framework

The de facto serialization framework for Rust.  Most `no_std` crates that do
serialization depend on `serde`.

```toml
[dependencies]
serde = { version = "1", default-features = false, features = ["alloc", "derive"] }
```

* `default-features = false` — **required** to disable `std`.
* `features = ["alloc"]` — **required** to enable `Serialize`/`Deserialize` for
  heap types (`String`, `Vec`, etc.).
* `features = ["derive"]` — enables `#[derive(Serialize, Deserialize)]`.
  **See the proc-macro section below** — the `derive` feature uses proc macros
  that compile on the *host*, not the target.  Because `serde_derive` is a proc
  macro crate it runs at build time on your host machine, so it works fine even
  though the final binary targets PPC.
* PPC notes — pure Rust data model.  No issues.

#### serde_json — JSON serialization

```toml
[dependencies]
serde_json = { version = "1", default-features = false, features = ["alloc"] }
```

* `default-features = false` — **required**.
* `features = ["alloc"]` — **required** (needs `Vec`, `String`).
* PPC notes — no platform-specific code.

#### postcard — compact, embedded-friendly binary format

A `serde`-compatible binary format designed for constrained environments.  Very
small overhead, deterministic encoding.

```toml
[dependencies]
postcard = { version = "1", default-features = false, features = ["alloc"] }
```

* `default-features = false` — **required**.
* `features = ["alloc"]` — optional; enables `to_allocvec()` and similar
  heap-based helpers.  Without it you must supply your own buffers.
* PPC notes — pure Rust.  Excellent fit for AmigaOS IPC where you want to pass
  structured data through message ports.

---

### 3. Parsing

#### nom — parser combinators

The classic Rust parser combinator library.  Supports `no_std` with byte-slice
and `&str` input.

```toml
[dependencies]
nom = { version = "7", default-features = false, features = ["alloc"] }
```

* `default-features = false` — **required**.
* `features = ["alloc"]` — optional; needed only if you want parsers that return
  owned `Vec`/`String`.
* PPC notes — pure Rust.  No issues.

#### winnow — modern parser combinators

A more ergonomic successor to `nom` with better error messages.

```toml
[dependencies]
winnow = { version = "0.6", default-features = false, features = ["alloc"] }
```

* `default-features = false` — **required**.
* `features = ["alloc"]` — optional; same purpose as with `nom`.
* PPC notes — pure Rust.

---

### 4. Bit Manipulation

#### bitflags — type-safe bit flags

Creates strongly-typed flag sets.  This is a natural fit for AmigaOS, which uses
bitmask flags extensively (window flags, screen modes, signal masks, etc.).

```toml
[dependencies]
bitflags = "2"
```

* `default-features = false` — not needed; `bitflags` is `no_std` by default.
* `features = ["alloc"]` — not needed.
* PPC notes — pure Rust.  Zero overhead.

Example wrapping AmigaOS window flags:

```rust
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WindowFlags: u32 {
        const SIZEGADGET    = 0x0001;
        const DRAGBAR       = 0x0002;
        const DEPTHGADGET   = 0x0004;
        const CLOSEGADGET   = 0x0008;
        const SIZEBRIGHT    = 0x0010;
        const SIZEBBOTTOM   = 0x0020;
        const ACTIVATE      = 0x1000;
        const RMBTRAP       = 0x0001_0000;
    }
}

let flags = WindowFlags::DRAGBAR | WindowFlags::DEPTHGADGET | WindowFlags::CLOSEGADGET;
```

---

### 5. Formatting

#### ufmt — micro-formatting

A much smaller alternative to `core::fmt`.  Produces significantly less code
than the standard `write!`/`format!` machinery, which matters when binary size
is a concern.

```toml
[dependencies]
ufmt = "0.2"
```

* `default-features = false` — not needed; `no_std` by default.
* `features = ["alloc"]` — not needed.
* PPC notes — pure Rust.

---

### 6. Hashing

#### crc32fast — fast CRC-32 checksums

Hardware-accelerated where possible, with a pure-Rust fallback.

```toml
[dependencies]
crc32fast = { version = "1", default-features = false }
```

* `default-features = false` — **required** to disable `std`.
* PPC notes — the PPC target will use the software fallback, which is still
  fast.

#### fnv — Fowler-Noll-Vo hash

A simple, non-cryptographic hash useful for hash maps.

```toml
[dependencies]
fnv = { version = "1", default-features = false }
```

* `default-features = false` — **required**.
* PPC notes — trivial pure Rust.

#### xxhash-rust — xxHash family

Very fast non-cryptographic hashing (xxh3, xxh32, xxh64).

```toml
[dependencies]
xxhash-rust = { version = "0.8", features = ["xxh3"] }
```

* `default-features = false` — not needed; `no_std` by default.
* Choose the hash variant you need via features: `xxh3`, `xxh32`, `xxh64`.
* PPC notes — pure Rust.  The `xxh32` variant is the best fit for 32-bit PPC
  since it operates on 32-bit words.

---

### 7. Random Number Generation

#### rand_core + rand_xorshift — lightweight PRNG

`rand_core` provides the `RngCore` trait; `rand_xorshift` provides a simple
xorshift PRNG that has no OS dependencies.

```toml
[dependencies]
rand_core = { version = "0.6", default-features = false }
rand_xorshift = "0.3"
```

* `default-features = false` on `rand_core` — **required** to avoid pulling in
  `std` and `getrandom`.
* PPC notes — pure Rust.  You must seed the generator yourself (for example,
  from the AmigaOS `timer.device` or `ReadEClock()`).

---

### 8. Error Handling

#### thiserror-core — derive Error for no_std

A `no_std`-compatible fork of the popular `thiserror` crate.  Lets you derive
the `core::error::Error` trait (stabilized since Rust 1.81).

```toml
[dependencies]
thiserror-core = { version = "1", default-features = false }
```

* `default-features = false` — **required**.
* PPC notes — proc macro crate, so it runs on the host.  The generated code is
  pure `core`.  No issues.

---

### 9. Math

#### libm — math functions for no_std

Provides `sinf`, `cosf`, `sqrtf`, `expf`, and many more — all in pure Rust.
Essential when you do not have a libc math library or want deterministic
soft-float behavior.

```toml
[dependencies]
libm = "0.2"
```

* `default-features = false` — not needed; `no_std` by default.
* PPC notes — pure Rust software implementations.  No hardware FPU usage.
  Performance is acceptable for non-inner-loop use; for heavy math, consider
  calling AmigaOS's `mathieeedoubtrans.library` via FFI.

---

### 10. Encoding

#### base64 — Base64 encoding/decoding

```toml
[dependencies]
base64 = { version = "0.22", default-features = false, features = ["alloc"] }
```

* `default-features = false` — **required**.
* `features = ["alloc"]` — **required** for `encode()`/`decode()` that return
  `String`/`Vec<u8>`.  Without `alloc` you can still encode/decode into
  caller-provided slices.
* PPC notes — pure Rust.

#### hex — hexadecimal encoding/decoding

```toml
[dependencies]
hex = { version = "0.4", default-features = false, features = ["alloc"] }
```

* `default-features = false` — **required**.
* `features = ["alloc"]` — **required** for owned-string helpers.
* PPC notes — pure Rust.

---

## Proc Macro Workarounds

### Why proc macros are special

Proc macros (custom derives, attribute macros, function-like macros) are Rust
compiler plugins that run **on the host** during compilation.  They are compiled
for the host architecture, not the target.

In our cross-compilation setup this actually means most proc macro crates *do*
work — `serde_derive`, `thiserror-core`, and `bitflags` derive macros all run
on the host (x86_64 Linux/Windows) and emit token streams that are then compiled
for PPC.

The situation where proc macros **do not** work is:

1. **When the proc macro itself links against a target-only library** — this is
   rare but happens with some OS-specific derive macros.
2. **When build.rs scripts try to compile test programs for the target** — some
   crates probe target features by compiling and running small programs, which
   fails on a cross-compilation host that cannot execute PPC binaries.

### Workaround 1: build.rs code generation

If you need to generate repetitive Rust code (for example, tables of AmigaOS
tag constants), use a `build.rs` script that runs on the host.

**Example**: generating tag constant tables from a TOML file.

Create `tags.toml`:

```toml
[[tags]]
name = "WA_Left"
value = "0x80000064"

[[tags]]
name = "WA_Top"
value = "0x80000065"

[[tags]]
name = "WA_Width"
value = "0x80000066"
```

Create `build.rs`:

```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("generated_tags.rs");

    // Read and parse the TOML file
    let toml_src = fs::read_to_string("tags.toml").unwrap();
    let mut output = String::new();

    for line in toml_src.lines() {
        let line = line.trim();
        if line.starts_with("name") {
            let name = line.split('"').nth(1).unwrap();
            output.push_str(&format!("pub const {}: u32 = ", name));
        } else if line.starts_with("value") {
            let value = line.split('"').nth(1).unwrap();
            output.push_str(&format!("{};\n", value));
        }
    }

    fs::write(dest, output).unwrap();
    println!("cargo:rerun-if-changed=tags.toml");
}
```

Use the generated file in your code:

```rust
mod tags {
    include!(concat!(env!("OUT_DIR"), "/generated_tags.rs"));
}

fn open_window() {
    let left = tags::WA_Left;
    // ...
}
```

### Workaround 2: pre-generated files with include!()

For data that rarely changes, you can commit the generated file directly and
use `include!()`:

```
src/
  main.rs
  generated/
    amiga_enums.rs    ← checked in, generated by a script
```

```rust
// In main.rs or lib.rs
mod generated {
    include!("generated/amiga_enums.rs");
}
```

This avoids any build-time complexity.  Run your generation script manually
whenever the source data changes.

---

## Integration Pattern: Adding a no_std Crate

Follow this step-by-step process to add any `no_std` crate to your AmigaOS
project.

### Step 1 — Add the dependency with correct features

In your project's `Cargo.toml`:

```toml
[dependencies]
# Always check whether default-features must be disabled
some-crate = { version = "x.y", default-features = false, features = ["alloc"] }
```

### Step 2 — Verify no_std compatibility

The crate **must not**:

- Use anything from `std` (file I/O, networking, threads, etc.)
- Call OS-specific APIs (Linux syscalls, Windows API, etc.)
- Depend on `getrandom` or other crates that require OS entropy
- Use inline assembly for a different architecture (x86 SSE intrinsics, etc.)

A quick check: search the crate's `Cargo.toml` for a `std` feature and ensure
you are not enabling it.  Search the crate source for `use std::` — if the only
occurrences are behind `#[cfg(feature = "std")]` you are fine.

### Step 3 — Cross-compilation sanity check

Build with:

```sh
cargo build --release --target powerpc-amigaos.json -Z build-std=core,alloc
```

If the build fails with linker or architecture errors, the crate may contain
platform-specific code.  Check the error messages and look for `cfg` gates you
might need to work around.

### Step 4 — Working example with bitflags

Here is a complete, minimal example that uses `bitflags` in an AmigaOS program.

`Cargo.toml`:

```toml
[package]
name = "flags-demo"
version = "0.1.0"
edition = "2021"
autobins = false

[lib]
name = "flags_demo"
crate-type = ["staticlib"]
path = "src/main.rs"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "s"
lto = true

[dependencies]
amigaos4-sys = { path = "../../amigaos4-sys" }
amigaos4 = { path = "../../amigaos4" }
amigaos4-alloc = { path = "../../amigaos4-alloc" }
bitflags = "2"
```

`src/main.rs`:

```rust
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use bitflags::bitflags;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Signals: u32 {
        const CTRL_C = 1 << 12;
        const CTRL_D = 1 << 13;
        const CTRL_E = 1 << 14;
        const CTRL_F = 1 << 15;
    }
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let sigs = Signals::CTRL_C | Signals::CTRL_D;
    amigaos4::serial_println!("Signal mask: 0x{:08x}", sigs.bits());
    amigaos4::serial_println!("Contains CTRL_C? {}", sigs.contains(Signals::CTRL_C));
    0
}
```

This compiles to a static library, which is then linked with ppc-amigaos-gcc
in Docker exactly like the `hello` example.

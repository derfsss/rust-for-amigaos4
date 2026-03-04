# Rust for AmigaOS 4

Write native AmigaOS 4.1 applications and device drivers in Rust.

**Status:** Tested on QEMU (`-M amigaone`). 129 SDK interfaces bound. Two allocator backends. Full `no_std` support.

> **Made with AI** — This project (code, bindings, build infrastructure, and documentation) was built using [Claude Code](https://claude.ai/claude-code) by Anthropic.

---

## Tested Versions

This project was developed and tested with the following exact versions. **Other versions are not tested and are not supported.**

| Component | Version | Notes |
|-----------|---------|-------|
| Rust | `rustc 1.96.0-nightly (80381278a 2026-03-01)` | Nightly required for `-Zbuild-std` |
| AmigaOS SDK | `54.16 (22.08.2022)` | AmigaOS 4.1 Final Edition SDK |
| Cross-compiler | `ppc-amigaos-gcc 11.5.0` (adtools build) | Inside Docker image |
| C library | `clib4 2.0.0 (16.08.2025)` | Inside Docker image |
| Docker image | `walkero/amigagccondocker:os4-gcc11` (built 2025-08-18) | Contains GCC + SDK + clib4 |
| QEMU | `qemu-system-ppc -M amigaone` | Test target |

---

## Software & Links

| Software | Purpose | Link |
|----------|---------|------|
| Rust (rustup) | Host compiler | https://rustup.rs/ |
| LLVM | PPC backend (bundled with Rust) | https://llvm.org/ |
| adtools (ppc-amigaos-gcc) | AmigaOS cross-compiler | https://github.com/sba1/adtools |
| clib4 | POSIX-compatible C library for AmigaOS 4 | https://github.com/afxgroup/clib4 |
| Docker | Container runtime for cross-compiler | https://www.docker.com/ |
| amigagccondocker | Pre-built Docker image with GCC + SDK | https://hub.docker.com/r/walkero/amigagccondocker |
| AmigaOS 4.1 SDK | Headers, libraries, autodocs | https://www.hyperion-entertainment.com/ |
| QEMU | PPC emulator for testing | https://www.qemu.org/ |

---

## What Works

- **Full `no_std` Rust** — `core` and `alloc` crates compile to PowerPC
- **`Vec`, `String`, `format!`, `Box`** — heap allocation via global allocator
- **Two allocator backends** — `Clib4Allocator` (malloc/free) for apps, `ExecAllocator` (AllocVecTagList/FreeVec) for drivers
- **129 AmigaOS SDK interface bindings** — Exec, DOS, Intuition, Graphics, Utility, and 124 more, all feature-gated
- **Direct vtable dispatch** — call any interface method from Rust via `#[repr(C)]` structs (no overhead)
- **C glue for varargs** — 5 varargs-only methods (DebugPrintF, IDoMethod, DoGadgetMethod, IDoSuperMethod, ICoerceMethod) wrapped as fixed-arity C functions
- **Safe RAII wrappers** — `TagListBuilder`, `AmigaVec`, `AmigaMsgPort`, `AmigaObject` (auto-cleanup on drop)
- **PPC inline assembly** — cache flush/invalidate, MMIO read/write (8/16/32-bit), memory barriers (eieio, sync, lwsync)
- **Application mode** — full programs with clib4, `-lauto`, `main()` entry point
- **Driver mode** — device handlers with no CRT, manual OpenLibrary, `_start()` → `rust_handler_main()` entry
- **POSIX-like modules** (app mode only) — file I/O (`fs`), time (`Instant`/`Duration`), environment vars (`env`), threads (`spawn`/`join` via pthreads)
- **debug_print!/debug_println!** — serial debug output via IExec->DebugPrintF
- **Standalone staticlibs** — pure Rust `.a` files (no allocator, no OS deps) that link into any C project
- **Automated build scripts** — `build.sh`/`build.bat` handle both Rust compilation and Docker linking

## What Does Not Work

- **No `std` crate** — this is `no_std` only; the Rust standard library does not support AmigaOS
- **No stack unwinding** — panic strategy is `abort`; no `catch_unwind`, no `?` in `main()`
- **No 64-bit atomics** — PPC G3/G4 supports only 32-bit atomic operations (`max-atomic-width: 32`)
- **No dynamic linking** — Rust produces static libraries only; no `.so` or shared library output
- **No Rust-native varargs** — the 5 varargs methods require C glue wrappers (provided)
- **No async/await** — no runtime, no executor; use OS-native message ports and signals instead
- **No proc macros** — proc macros require host-target execution; cross-compilation prevents this
- **No Cargo test runner** — `cargo test` cannot run on the host; test on QEMU or real hardware
- **No crates.io ecosystem** — most crates assume `std`; only `no_std`-compatible crates work
- **No GUI bindings yet** — ReAction/BOOPSI gadget class bindings are not included (use raw interface calls)
- **No network bindings** — bsdsocket.library interface is bound but no safe wrappers exist
- **Not tested on real hardware** — only verified on QEMU (`-M amigaone`); real AmigaOne/Sam/X5000 may have differences

---

## How It Works

A two-stage cross-compilation pipeline:

```
 Stage 1 (Your Machine)              Stage 2 (Docker)
┌─────────────────────┐    ┌──────────────────────────────┐
│ Rust source code    │    │ ppc-amigaos-gcc links:       │
│         ↓           │    │   C glue (.o)                │
│ cargo +nightly      │    │ + Rust staticlib (.a)        │
│         ↓           │    │ + clib4 / SDK libs           │
│ libmyapp.a (PPC)    │───→│         ↓                    │
└─────────────────────┘    │ Native AmigaOS executable    │
                           └──────────────────────────────┘
```

1. **Host** (Windows/Linux/macOS): `cargo` compiles Rust to a PowerPC static library using a custom target spec and LLVM's PPC backend
2. **Docker**: `ppc-amigaos-gcc` links the Rust `.a` with C glue code and SDK libraries into a native AmigaOS binary

The "fake linker" trick makes this work: cargo expects a linker, but the real linking happens in Docker. A stub script creates an empty output file so cargo succeeds.

---

## Quick Start

### 1. Run Setup

```bash
# Linux / macOS
chmod +x setup.sh
./setup.sh

# Windows
setup.bat
```

This installs Rust nightly + rust-src and pulls the Docker cross-compiler image.

### 2. Build the Hello World Example

```bash
# Linux / macOS
chmod +x build.sh
./build.sh examples/hello

# Windows
build.bat examples\hello
```

That's it. The output is `examples/hello/hello` — a native AmigaOS 4 executable.

### 3. Build the Driver Example

```bash
# Linux / macOS
./build.sh examples/hello-driver

# Windows
build.bat examples\hello-driver
```

### Clean

```bash
./build.sh examples/hello clean
```

---

## Create Your Own Project

### Application (with clib4, -lauto)

Copy `templates/app/` to a new directory under this repo:

```bash
cp -r templates/app myproject
```

Then edit:
1. **`Cargo.toml`** — change `name` and `[lib] name`
2. **`Makefile`** — change `TARGET` and `RUST_LIB` to match
3. **`src/main.rs`** — write your code

Build:
```bash
./build.sh myproject
```

### Driver / Handler (no clib4)

Copy `templates/driver/`:

```bash
cp -r templates/driver mydriver
```

Edit the same files, then:
```bash
./build.sh mydriver
```

---

## Repository Layout

```
rust-for-amigaos4/
├── setup.sh / setup.bat       # One-time setup (Rust nightly + Docker image)
├── build.sh / build.bat       # Build any project: ./build.sh <path>
├── .gitignore
│
├── target-spec/               # Cross-compilation infrastructure
│   ├── powerpc-amigaos.json   # Custom Rust target specification
│   ├── fake-linker.bat        # Windows stub linker
│   └── fake-linker.sh         # Linux/macOS stub linker
│
├── amigaos4-sys/              # Raw FFI bindings (129 interfaces)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs             # Feature-gated re-exports
│   │   ├── types.rs           # Fundamental types, constants, opaque structs
│   │   ├── glue.rs            # extern "C" declarations for C glue
│   │   ├── ppc_asm.rs         # PPC inline asm (cache, MMIO, barriers)
│   │   ├── interfaces/        # 129 interface vtable structs
│   │   │   ├── exec.rs        # ExecIFace (268 methods)
│   │   │   ├── dos.rs         # DOSIFace
│   │   │   ├── intuition.rs
│   │   │   └── ...
│   │   └── wrappers/          # Convenience functions per interface
│   │       ├── exec.rs        # exec_find_task(), exec_alloc_vec(), ...
│   │       └── ...
│   └── glue/
│       └── amiga_glue.c       # C wrappers for varargs methods
│
├── amigaos4/                  # Safe wrappers and std-like modules
│   ├── Cargo.toml
│   └── src/
│       ├── error.rs           # AmigaError enum + Result<T>
│       ├── tag.rs             # TagListBuilder (safe tag construction)
│       ├── mem.rs             # AmigaVec (RAII memory)
│       ├── port.rs            # AmigaMsgPort (RAII)
│       ├── boopsi.rs          # AmigaObject (RAII BOOPSI)
│       ├── fmt.rs             # debug_print!/debug_println! macros
│       ├── fs.rs              # File I/O (app-only)
│       ├── time.rs            # Instant/Duration (app-only)
│       ├── env.rs             # current_dir, var (app-only)
│       └── thread.rs          # spawn/join via pthreads (app-only)
│
├── amigaos4-alloc/            # Global allocator backends
│   ├── Cargo.toml
│   └── src/
│       ├── clib4_alloc.rs     # malloc/free (for applications)
│       └── exec_alloc.rs      # AllocVecTagList/FreeVec (for drivers)
│
├── templates/
│   ├── app/                   # Application starter (clib4, -lauto)
│   │   ├── Cargo.toml
│   │   ├── .cargo/config.toml
│   │   ├── Makefile
│   │   ├── powerpc-amigaos.json
│   │   ├── fake-linker.bat / .sh
│   │   └── src/main.rs
│   └── driver/                # Driver/handler starter (no CRT)
│       ├── Cargo.toml
│       ├── .cargo/config.toml
│       ├── Makefile
│       ├── powerpc-amigaos.json
│       ├── fake-linker.bat / .sh
│       ├── src/main.rs
│       └── src/driver_glue.c
│
└── examples/
    ├── hello/                 # Working app example
    └── hello-driver/          # Working driver example
```

---

## Application vs Driver Mode

| | Application | Driver |
|-|-------------|--------|
| **CRT** | clib4 (`-mcrt=clib4`) | None (`-nostartfiles`) |
| **Libraries** | `-lauto` (auto-open) | Manual `OpenLibrary`/`GetInterface` |
| **Entry point** | `main()` | `_start()` in `driver_glue.c` → `rust_handler_main()` |
| **Allocator** | `Clib4Allocator` (malloc/free) | `ExecAllocator` (AllocVecTagList/FreeVec) |
| **POSIX modules** | `fs`, `time`, `env`, `thread` | Not available |
| **Link flags** | `-mcrt=clib4 -lauto` | `-nostartfiles -nodefaultlibs -lgcc` |

Both modes support `Vec`, `String`, `format!`, `Box` via the global allocator.

---

## Architecture Deep Dive

### How AmigaOS Interfaces Work

AmigaOS 4 uses **interface vtables** — C structs of function pointers. Every method takes the interface pointer as the first argument:

```c
// C code
IExec->FindTask(NULL);  // GCC inserts IExec as hidden first arg

// What actually happens at the ABI level:
(*IExec->FindTask)(IExec, NULL);  // explicit first arg
```

Rust calls these directly via `repr(C)` structs:

```rust
// Rust (direct vtable dispatch — no wrapper needed)
unsafe {
    let task = ((*IExec).FindTask)(IExec, core::ptr::null());
}

// Or using the convenience wrapper:
unsafe {
    let task = exec_find_task(core::ptr::null());
}
```

### Why C Glue Is Needed

Five AmigaOS methods use C varargs with **no** non-varargs alternative:

| Method | Purpose |
|--------|---------|
| `IExec->DebugPrintF(fmt, ...)` | Serial debug output |
| `IIntuition->IDoMethod(obj, msg, ...)` | BOOPSI method dispatch |
| `IIntuition->DoGadgetMethod(gad, win, req, msg, ...)` | Gadget method dispatch |
| `IIntuition->IDoSuperMethod(cl, obj, msg, ...)` | Superclass dispatch |
| `IIntuition->ICoerceMethod(cl, obj, msg, ...)` | Coerced class dispatch |

Rust FFI cannot call C varargs functions. The solution: fixed-arity C wrappers in `amigaos4-sys/glue/amiga_glue.c`:

```c
uint32 amiga_do_method_3(Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3) {
    return IIntuition->IDoMethod(obj, mid, a1, a2, a3);
}
```

```rust
// Rust calls the fixed-arity wrapper
extern "C" {
    fn amiga_do_method_3(obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32) -> u32;
}
```

All other varargs methods (`NewObject`, `SetAttrs`, `OpenWindow`, etc.) have `*A` or `*TagList` partners that take `*const TagItem` — these are callable directly from Rust.

### Feature-Gated Interfaces

`amigaos4-sys` has 129 interfaces. Enable only what you need:

```toml
[dependencies]
amigaos4-sys = { path = "../../amigaos4-sys",
                 features = ["exec", "dos", "intuition", "graphics"] }
```

Default features: `exec`, `dos`, `intuition`, `utility`.

For the full SDK:
```toml
amigaos4-sys = { path = "../../amigaos4-sys", features = ["full-sdk"] }
```

### Safe Wrappers (amigaos4 crate)

RAII wrappers prevent resource leaks:

```rust
use amigaos4::tag::TagListBuilder;
use amigaos4::mem::AmigaVec;

// TagListBuilder — safe tag array construction
let tags = TagListBuilder::new()
    .tag(AVT_TYPE, MEMF_SHARED)
    .tag_if(need_clear, AVT_CLEAR_WITH_VALUE, 0)
    .build();

// AmigaVec — RAII memory allocation (auto-freed on drop)
{
    let buf = AmigaVec::alloc(4096, MEMF_SHARED)?;
    let slice = buf.as_mut_slice();
    // use the buffer...
} // automatically freed here

// AmigaMsgPort — RAII message port
{
    let port = AmigaMsgPort::new()?;
    // use port.as_ptr() with OS calls...
} // automatically deleted here
```

### PPC Inline Assembly

The `ppc_asm` module provides hardware primitives for driver development:

```rust
use amigaos4_sys::ppc_asm::*;

unsafe {
    // Cache management (DMA buffer prep)
    cache_flush(buf_ptr, buf_len);       // dcbst — CPU writes visible to device
    cache_invalidate(buf_ptr, buf_len);  // dcbf — device writes visible to CPU

    // MMIO access (PCI registers, byte-reversed for LE on BE PPC)
    let status = mmio_r32(bar_addr + OFFSET);
    mmio_w32(bar_addr + OFFSET, value);

    // Memory barriers
    eieio_barrier();   // I/O ordering
    sync_barrier();    // full memory barrier
    lwsync_barrier();  // lightweight sync
}
```

---

## Custom Target Specification

The custom target spec (`powerpc-amigaos.json`) and fake linker scripts are in `target-spec/` and pre-copied into each template and example.

Key fields in `powerpc-amigaos.json`:
- `"os": "none"` — bare-metal target; `#![no_std]` only
- `"linker": "fake-linker.bat"` — stub linker (Linux/macOS: override in `.cargo/config.toml`)
- `"panic-strategy": "abort"` — no unwinding support
- `"max-atomic-width": 32` — PPC G3/G4 atomics
- `"use-ctors-section": true` — required for clib4 constructor chains

For Linux/macOS hosts, add this to your `.cargo/config.toml`:

```toml
[target.powerpc-amigaos]
linker = "./fake-linker.sh"
```

---

## Standalone Staticlib (No Allocator)

For pure computation modules (wire format parsers, hash functions, codecs) that replace a C source file — no `amigaos4-sys` dependency needed:

```toml
# Cargo.toml
[package]
name = "my-codec"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]

[profile.release]
opt-level = "s"
lto = true
panic = "abort"
```

```rust
// src/lib.rs
#![no_std]

#[no_mangle]
pub extern "C" fn my_encode(buf: *mut u8, len: u32) -> u32 {
    // Pure data transformation — no allocator, no OS calls
    len
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}
```

The `.a` links into any C project via `ppc-amigaos-gcc`.

---

## Manual Build (Without Scripts)

If you prefer not to use the build scripts:

### Stage 1: Compile Rust (Host)

```bash
cd examples/hello
cargo +nightly build --release
```

Output: `target/powerpc-amigaos/release/libhello.a`

### Stage 2: Link (Docker)

**Linux / macOS:**
```bash
# From the repo root
docker run --rm \
    -v "$(pwd)":/repo \
    -w /repo/examples/hello \
    walkero/amigagccondocker:os4-gcc11 \
    make clean

docker run --rm \
    -v "$(pwd)":/repo \
    -w /repo/examples/hello \
    walkero/amigagccondocker:os4-gcc11 \
    make all
```

**Windows (WSL2):**
```bash
wsl sh -c "docker run --rm \
    -v /mnt/w/path/to/rust-for-amigaos4:/repo \
    -w /repo/examples/hello \
    walkero/amigagccondocker:os4-gcc11 \
    make clean && \
docker run --rm \
    -v /mnt/w/path/to/rust-for-amigaos4:/repo \
    -w /repo/examples/hello \
    walkero/amigagccondocker:os4-gcc11 \
    make all"
```

Always run `make clean` before `make all` — stale `.o` files cause confusing bugs.

---

## Troubleshooting

### "`.json` target specs require -Zjson-target-spec"

Add to `.cargo/config.toml`:
```toml
[unstable]
json-target-spec = true
```
(Already included in all templates and examples.)

### "undefined reference to `memset`" / `memcpy`

Add to `.cargo/config.toml`:
```toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
```
(Already included in all templates and examples.)

### AllocVecTagList returns NULL

Check your MEMF constants. The correct values are:
```rust
MEMF_PRIVATE = 1 << 11  // 0x800 — CPU-only memory (fast)
MEMF_SHARED  = 1 << 12  // 0x1000 — DMA-capable memory
```

**NOT** `MEMF_PRIVATE = 2` (that's `MEMF_CHIP` — 68k chip RAM, doesn't exist on PPC).

### "fake-linker" not found

- **Windows**: Ensure `fake-linker.bat` is in the project root (same directory as `powerpc-amigaos.json`)
- **Linux/macOS**: Run `chmod +x fake-linker.sh` and add to `.cargo/config.toml`:
  ```toml
  [target.powerpc-amigaos]
  linker = "./fake-linker.sh"
  ```

### Link errors about missing IExec / IIntuition

- **Driver mode**: Ensure `driver_glue.c` defines `struct ExecIFace *IExec = NULL;`
- **Application mode**: Ensure you link with `-lauto` (provides all `I*` globals)

### GCC synthesizes memset from loops

Add `-fno-tree-loop-distribute-patterns` to CFLAGS. GCC `-O2` converts zero-fill loops to `memset` calls, which fail if newlib/clib4 aren't linked. (Already included in driver template.)

---

## Tested Platform

- QEMU (`qemu-system-ppc -M amigaone`) — application and driver modes

## License

MIT

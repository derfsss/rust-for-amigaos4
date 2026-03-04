# amigaos4-sys Crate Guide

## Overview

`amigaos4-sys` provides raw FFI bindings for AmigaOS 4.1 library interfaces.
Non-varargs methods are called directly via vtable dispatch with zero overhead.
Varargs methods use fixed-arity C glue wrappers.

## Adding as a dependency

```toml
[dependencies]
# Default: exec, dos, intuition, utility (4 core interfaces)
amigaos4-sys = { path = "../amigaos4-sys" }

# Add specific interfaces
amigaos4-sys = { path = "../amigaos4-sys", features = ["graphics", "layers"] }

# All 129 interfaces from the full AmigaOS SDK
amigaos4-sys = { path = "../amigaos4-sys", features = ["full-sdk"] }
```

The crate is `#![no_std]` and has no dependencies.

## Feature flags

Interfaces are gated behind Cargo features. The default set compiles only
the 4 core interfaces used by most programs:

| Feature | Default | Interfaces |
|---------|---------|------------|
| `exec` | yes | ExecIFace, MMUIFace, DebugIFace |
| `dos` | yes | DOSIFace |
| `intuition` | yes | IntuitionIFace |
| `utility` | yes | UtilityIFace |
| `graphics` | no | GraphicsIFace |
| `layers` | no | LayersIFace |
| `ahi` | no | AHIIFace, AHISubIFace |
| `amissl` | no | AmiSSLIFace, AmiSSLMasterIFace |
| ... | no | 121 more interfaces |
| `full-sdk` | no | All 129 interfaces |

To disable defaults and pick only what you need:

```toml
amigaos4-sys = { path = "../amigaos4-sys", default-features = false, features = ["exec", "dos"] }
```

## Usage

```rust
#![no_std]
extern crate amigaos4_sys;
use amigaos4_sys::*;

// Direct vtable dispatch (non-varargs)
let task = unsafe { exec_find_task(core::ptr::null()) };
let mem = unsafe { exec_avail_mem(0) };

// Or use raw vtable syntax
let task = unsafe { ((*IExec).FindTask)(IExec, core::ptr::null()) };

// C glue (varargs methods)
unsafe { amiga_debug_str(b"Hello!\n\0".as_ptr()); }
```

## Module structure

```
src/
  lib.rs              # Feature-gated module declarations + pub use
  types.rs            # APTR, STRPTR, ULONG, TagItem, Node, InterfaceData, 215 opaques
  interfaces/         # Per-interface vtable struct modules (129 total)
    mod.rs            # Feature-gated pub mod declarations
    exec.rs           # ExecIFace (268 fields), MMUIFace, DebugIFace
    dos.rs            # DOSIFace (309 fields)
    intuition.rs      # IntuitionIFace (234 fields)
    utility.rs        # UtilityIFace (78 fields)
    graphics.rs       # GraphicsIFace
    ...               # 124 more interface modules
  wrappers/           # Global pointers + convenience functions (19 modules)
    mod.rs
    exec.rs           # IExec global + exec_* inline wrappers
    dos.rs            # IDOS global + dos_* inline wrappers
    ...
  glue.rs             # extern "C" declarations for C glue functions
glue/
  amiga_glue.c        # C wrappers for varargs methods (50+ functions)
```

All types from `types.rs` are always available regardless of feature flags.
Interface and wrapper modules are conditionally compiled.

## Compiling the C glue

The `glue/amiga_glue.c` file must be compiled by `ppc-amigaos-gcc` and linked
into the final binary. Add it to your Makefile:

```makefile
SYS_GLUE_SRC = ../amigaos4-sys/glue/amiga_glue.c
SYS_GLUE_OBJ = build/sys_glue.o

$(SYS_GLUE_OBJ): $(SYS_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET): $(SYS_GLUE_OBJ) $(RUST_LIB)
	$(CC) $(LDFLAGS) -o $@ $(SYS_GLUE_OBJ) $(RUST_LIB) $(LIBS)
```

If you don't call any varargs methods, you can skip compiling the glue entirely.

## What's included

### Interface vtable structs (129 total)

Core interfaces (compiled by default):

| Module | Struct | Fields | Library |
|--------|--------|--------|---------|
| `interfaces/exec.rs` | ExecIFace | 268 | exec.library |
| `interfaces/exec.rs` | MMUIFace | 10 | exec.library (secondary) |
| `interfaces/exec.rs` | DebugIFace | 18 | exec.library (secondary) |
| `interfaces/dos.rs` | DOSIFace | 309 | dos.library |
| `interfaces/intuition.rs` | IntuitionIFace | 234 | intuition.library |
| `interfaces/utility.rs` | UtilityIFace | 78 | utility.library |

Additional interfaces (125 more, feature-gated):

Graphics, Layers, AHI, AmiSSL, ASL, Bitmap, BulletInfo, CamdDriver,
Chooser, Clicktab, Clipboard, CxBroker, DataBrowser, DataTypes,
DiskFont, ExpansionInterface, Filler, FuelGauge, GadTools, GetColor,
GetFile, GetFont, GetScreenMode, IFFParse, Icon, Input, Integer,
Keymap, Label, Layout, ListBrowser, Locale, LowLevel, MPEGA, NewIcon,
P96, Partition, PenMap, Picasso96, PopupMenu, RadioButton,
Ramdrive, RealTime, Requester, Resource, Scroller, Serial, Slider,
Space, SpeedBar, StatusBar, String, TextEditor, Timezone, Timer,
VirtioBlock, VirtioNet, VirtioSCSI, Window, Workbench, and many more.

### Global pointers

```rust
extern "C" {
    pub static IExec: *mut ExecIFace;
    pub static IDOS: *mut DOSIFace;
    pub static IIntuition: *mut IntuitionIFace;
    pub static IUtility: *mut UtilityIFace;
}
```

These are initialised by clib4's `crtbegin.o` when linked with `-lauto`.
MMUIFace and DebugIFace are secondary interfaces obtained via `GetInterface`.

### Convenience wrappers

Every non-varargs, non-private method has an `#[inline]` wrapper:

```rust
// Instead of:
let task = unsafe { ((*IExec).FindTask)(IExec, name) };

// You can write:
let task = unsafe { exec_find_task(name) };
```

Naming convention: `{interface}_{snake_case_method}` — e.g. `exec_find_task`,
`dos_open`, `intuition_new_object_a`, `utility_find_tag_item`.

### C glue wrappers

For the 5 varargs methods with no non-varargs alternative:

| Rust function | C function | Method |
|---------------|------------|--------|
| `amiga_debug_str(s)` | `amiga_debug_str` | DebugPrintF("%s", s) |
| `amiga_debug_fmt_u32(fmt, a)` | `amiga_debug_fmt_u32` | DebugPrintF(fmt, a) |
| `amiga_do_method_N(obj, mid, ...)` | `amiga_do_method_N` | IDoMethod |
| `amiga_do_gadget_method_N(gad, win, req, mid, ...)` | ... | DoGadgetMethod |
| `amiga_do_super_method_N(cl, obj, mid, ...)` | ... | IDoSuperMethod |
| `amiga_coerce_method_N(cl, obj, mid, ...)` | ... | ICoerceMethod |

Where N = number of extra arguments (0-8 for most, 0-6 for DoGadgetMethod).

## Regenerating bindings

If the SDK headers change, regenerate with amigaos4-bindgen:

```bash
cd projects/amigaos4-bindgen

# Split output mode (generates complete crate source tree)
cargo run -- /path/to/SDK/include/interfaces \
    --split-output ../amigaos4-sys/src \
    --report full_sdk_report.txt

# Then copy generated files into the crate:
# - types.rs, lib.rs → src/
# - interfaces/*.rs → src/interfaces/
# - wrappers/*.rs → src/wrappers/
# - features.toml → merge into Cargo.toml [features] section
```

The `--split-output` mode generates per-interface modules with feature
gates. The monolithic `-o` mode is still available for single-file output.

## Global allocator

The crate does NOT include a `#[global_allocator]`. If you use `alloc`,
define one in your binary crate:

```rust
extern crate alloc;

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

struct Clib4Allocator;
unsafe impl core::alloc::GlobalAlloc for Clib4Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { malloc(layout.size()) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _: core::alloc::Layout) {
        unsafe { free(ptr) }
    }
}

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;
```

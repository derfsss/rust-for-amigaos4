# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Type layout tests: TagItem size/alignment, TAG_DONE/USER/IGNORE values (5 tests)

## [0.2.0] - 2026-03-15

Initial release.

### Added

- 129 feature-gated interface vtable structs covering the full AmigaOS 4.1 SDK
  (ExecIFace with 268 methods, DOSIFace with 309 methods, IntuitionIFace with
  234 methods, and 126 more).
- Fundamental types, constants, and opaque struct declarations in `types.rs`.
- 19 convenience wrapper modules with inline functions for direct vtable
  dispatch.
- C glue declarations for 5 varargs-only methods: `DebugPrintF`, `IDoMethod`,
  `DoGadgetMethod`, `IDoSuperMethod`, and `ICoerceMethod`.
- C glue implementation in `amiga_glue.c` with 50+ fixed-arity C wrappers for
  varargs functions that cannot be called directly from Rust.
- PPC inline assembly module (`ppc_asm.rs`) providing cache flush/invalidate,
  MMIO read/write for 8/16/32-bit widths, and memory barriers.
- `full-sdk` meta-feature that enables all 129 interface modules at once.
- Default features: `exec`, `dos`, `intuition`, `utility`.

[Unreleased]: https://github.com/anthropic/rust-for-amigaos4/compare/amigaos4-sys-v0.2.0...HEAD
[0.2.0]: https://github.com/anthropic/rust-for-amigaos4/releases/tag/amigaos4-sys-v0.2.0

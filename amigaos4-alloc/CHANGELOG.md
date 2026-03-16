# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Zero-sized type verification tests for both allocators (2 tests)

## [0.1.0] - 2026-03-15

Initial release.

### Added

- `Clib4Allocator` global allocator backed by `malloc`/`free`, suitable for
  applications linked with clib4.
- `ExecAllocator` global allocator backed by `AllocVecTagList`/`FreeVec`,
  suitable for drivers and other contexts where clib4 is unavailable, with
  transparent handling of alignment requests greater than 16 bytes.
- Feature-gated backend selection: `clib4` (default) or `exec`.

[Unreleased]: https://github.com/anthropic/rust-for-amigaos4/compare/amigaos4-alloc-v0.1.0...HEAD
[0.1.0]: https://github.com/anthropic/rust-for-amigaos4/releases/tag/amigaos4-alloc-v0.1.0

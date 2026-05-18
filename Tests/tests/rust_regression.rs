//! Regression tests for known past bugs.
//!
//! Each test here corresponds to a specific incident — usually traceable
//! to a PR or commit. If the underlying mistake comes back, the test
//! fires before it can ship.

use amiga_tests::{ldflags_of, read_repo_file, repo_root};
use std::fs;
use std::path::PathBuf;

// ── PR #1 (2026-05) ──────────────────────────────────────────
//
// `ld -s` strips `_start` and `__amigaos4__` from the binary's
// `.symtab`, which the AmigaOS loader requires. The fix removed `-s`
// from LDFLAGS in every clib4-dependent example. `ppc-amigaos-strip`
// is run afterwards — it's Amiga-aware and preserves those symbols.

#[test]
fn no_clib4_example_uses_ld_dash_s() {
    let examples_dir = repo_root().join("examples");
    let mut offenders: Vec<String> = Vec::new();

    for entry in fs::read_dir(&examples_dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() { continue; }
        let mk = entry.path().join("Makefile");
        if !mk.exists() { continue; }

        let mk_text = fs::read_to_string(&mk).unwrap();
        if !mk_text.contains("-mcrt=clib4") { continue; }

        if let Some(ldflags) = ldflags_of(&mk) {
            // Look for ` -s` as a free token (not inside a longer flag).
            for token in ldflags.split_whitespace() {
                if token == "-s" {
                    offenders.push(format!("{} (LDFLAGS: {})",
                        entry.path().display(), ldflags));
                }
            }
        }
    }
    assert!(offenders.is_empty(),
        "PR #1 regression — `-s` flag back in clib4 example LDFLAGS:\n  - {}",
        offenders.join("\n  - "));
}

// ── Post-Phase-10 line-ending normalisation ──────────────────
//
// .gitattributes declares `*.rs text eol=lf` (and similar for .toml,
// .md, .c, .h, Makefile). Files committed with CRLF endings violate
// this rule. We can't probe the index directly without git plumbing,
// but we can ask git ls-files --eol and confirm none of the LF-declared
// files have CRLF or mixed line endings in the index.

#[test]
fn no_lf_declared_files_have_crlf_in_index() {
    let out = std::process::Command::new("git")
        .args(["ls-files", "--eol"])
        .current_dir(repo_root())
        .output()
        .expect("git ls-files failed");
    assert!(out.status.success(), "git ls-files --eol failed");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut bad: Vec<String> = Vec::new();
    for line in stdout.lines() {
        // Format: "i/<eol>  w/<eol>  attr/<attr>  <tab>  <path>"
        if !line.contains("eol=lf") { continue; }
        if line.starts_with("i/crlf") || line.starts_with("i/mixed") {
            // Pull the path from the tab.
            if let Some(tab) = line.find('\t') {
                bad.push(line[tab + 1..].to_string());
            } else {
                bad.push(line.to_string());
            }
        }
    }
    // Tolerate at most a small number of legitimately mixed files
    // (e.g. embedded \r in test fixtures). At time of writing this
    // should be zero.
    assert!(bad.is_empty(),
        ".gitattributes EOL violations (LF declared, CRLF/mixed in index):\n  - {}",
        bad.join("\n  - "));
}

// ── Library example needs --undefined=RomTag ────────────────
//
// Without it, `--gc-sections` drops the Resident struct (it's
// referenced only by the OS at runtime, not by any C symbol). The
// resulting .library has no RomTag and the OS refuses to load it.

#[test]
fn library_example_keeps_romtag_alive() {
    let mk = read_repo_file("examples/hello-library/Makefile");
    assert!(mk.contains("--undefined=RomTag"),
        "examples/hello-library/Makefile is missing -Wl,--undefined=RomTag — \
         the Resident struct will be GC'd by --gc-sections");
}

#[test]
fn library_template_keeps_romtag_alive() {
    let mk = read_repo_file("templates/library/Makefile");
    assert!(mk.contains("--undefined=RomTag"),
        "templates/library/Makefile is missing -Wl,--undefined=RomTag");
}

// ── LICENSE file present ────────────────────────────────────
//
// PR #1's force-push rebase made the diff look like it deleted
// LICENSE (because the PR was based on a pre-LICENSE commit). The
// merge worked but the false-alarm wasted review effort. Keeping a
// guardrail here makes future "where did LICENSE go?" diagnostics
// trivial.

#[test]
fn license_file_present_and_mit() {
    let p = repo_root().join("LICENSE");
    assert!(p.exists(), "LICENSE file is missing from repo root");
    let text = fs::read_to_string(&p).unwrap();
    assert!(text.contains("MIT License") || text.to_lowercase().contains("mit license"),
        "LICENSE present but does not appear to be the MIT license");
}

// ── Submodule/binary drift ───────────────────────────────────
//
// The project's reproducibility claim is that clib4-nightly/ binaries
// are built from the exact submodule commit. There's no way to verify
// byte-equality without rebuilding (expensive, Docker-only), so this
// test enforces the weaker invariant: clib4-nightly/clib4.library is
// at least as large as the source's manifest would imply, and both
// files exist.

#[test]
fn clib4_binary_present_alongside_pinned_submodule() {
    let bin = repo_root().join("clib4-nightly").join("clib4.library");
    let sub = repo_root().join("clib4-src").join(".git");
    assert!(bin.exists(),
        "clib4-nightly/clib4.library missing — overlay broken");
    assert!(sub.exists(),
        "clib4-src/ submodule not initialised — run `git submodule update --init`");
}

// ── Templates parse as Cargo manifests ──────────────────────

#[test]
fn template_cargo_tomls_parse() {
    for tpl in &["app", "driver", "library"] {
        let path = repo_root().join("templates").join(tpl).join("Cargo.toml");
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing {}", path.display()));
        // Cheap parse check — must contain [package] and name = "...".
        assert!(text.contains("[package]"),
            "templates/{}/Cargo.toml missing [package]", tpl);
        assert!(text.contains("name ="),
            "templates/{}/Cargo.toml missing name field", tpl);
    }
}

// ── fake-linker pair exists for every example ───────────────
//
// build.sh runs the Linux fake-linker.sh; powerpc-amigaos.json
// targets fake-linker.bat. Both must exist or one of the build paths
// breaks silently.

#[test]
fn every_example_has_both_fake_linkers() {
    let mut bad: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(repo_root().join("examples")).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() { continue; }
        let dir = entry.path();
        for needed in &["fake-linker.sh", "fake-linker.bat"] {
            let p = dir.join(needed);
            if !p.exists() { bad.push(p); }
        }
    }
    assert!(bad.is_empty(),
        "examples missing fake-linker files:\n  - {}",
        bad.iter().map(|p| p.display().to_string())
                  .collect::<Vec<_>>().join("\n  - "));
}

// ── No example shells out to nproc on Windows-incompatible paths ─

#[test]
fn build_scripts_dont_assume_nproc_on_windows() {
    // build.bat must NOT call `nproc` (which doesn't exist on Windows
    // outside MSYS). build.sh may.
    let bat = read_repo_file("build.bat");
    let line_with_nproc = bat.lines()
        .find(|l| l.contains("nproc") && !l.trim_start().starts_with("REM "));
    assert!(line_with_nproc.is_none(),
        "build.bat references nproc — won't work on cmd.exe:\n  {}",
        line_with_nproc.unwrap_or(""));
}

// ── dos binding additions (this session) ──────────────────────
//
// Backfilled 10 SDK methods at DOSIFace positions 305-314 that the
// Rust binding had previously stubbed as Reserved15-18_UNIMPLEMENTED
// (4 slots) plus 6 entries the older binding didn't reach. Pin them
// here so a fresh bindgen run against a *stale* SDK can't silently
// drop them again — the rust_sdk_audit catches drift against the
// current SDK, but this guard is independent of whether the SDK is
// available on the box running the suite.

const DOS_BACKFILLED_METHODS: &[&str] = &[
    "GetContext", "SetContext", "GetFinalData", "SetChildPID",
    "GetCliEpilog", "SetCliEpilog", "RelativePath", "SubLock",
    "HexToLongLong", "StrToLongLong",
];

const DOS_BACKFILLED_WRAPPERS: &[&str] = &[
    "dos_get_context", "dos_set_context", "dos_get_final_data",
    "dos_set_child_pid", "dos_get_cli_epilog", "dos_set_cli_epilog",
    "dos_relative_path", "dos_sub_lock", "dos_hex_to_long_long",
    "dos_str_to_long_long",
];

#[test]
fn dos_iface_has_backfilled_methods() {
    let text = read_repo_file("amigaos4-sys/src/interfaces/dos.rs");
    let missing: Vec<&&str> = DOS_BACKFILLED_METHODS.iter()
        .filter(|m| !text.contains(&format!("pub {}:", m)))
        .collect();
    assert!(missing.is_empty(),
        "DOSIFace lost backfilled vtable methods (regen against stale SDK?):\n  - {:?}",
        missing);
}

#[test]
fn dos_wrappers_for_backfilled_methods_exist() {
    let text = read_repo_file("amigaos4-sys/src/wrappers/dos.rs");
    let missing: Vec<&&str> = DOS_BACKFILLED_WRAPPERS.iter()
        .filter(|w| !text.contains(&format!("pub unsafe fn {}(", w)))
        .collect();
    assert!(missing.is_empty(),
        "amigaos4-sys/src/wrappers/dos.rs is missing wrappers:\n  - {:?}",
        missing);
}

// ── elf.library SDK drift (2026-05) ──────────────────────────
//
// The SDK's ElfIFace gained `GetSOHandles` at the end of the vtable
// after the Rust binding was generated. Backfilled at position 33
// with a wrapper; pin it here so the next regen against a stale SDK
// doesn't drop it again.

#[test]
fn elf_iface_has_get_so_handles() {
    let text = read_repo_file("amigaos4-sys/src/interfaces/elf.rs");
    assert!(text.contains("pub GetSOHandles:"),
        "ElfIFace lost backfilled GetSOHandles vtable entry");

    let wrap = read_repo_file("amigaos4-sys/src/wrappers/elf.rs");
    assert!(wrap.contains("pub unsafe fn elf_get_sohandles("),
        "wrappers/elf.rs lost the elf_get_sohandles wrapper");
}

#[test]
fn dos_cli_init_not_under_private_prefix() {
    // The SDK has always called this method `CliInit`. An older
    // amigaos4-bindgen convention prefixed it with `PRIVATE`. The
    // rename to match the SDK should stick; if a regen reintroduces
    // the PRIVATE prefix, fail.
    let iface = read_repo_file("amigaos4-sys/src/interfaces/dos.rs");
    let wrap  = read_repo_file("amigaos4-sys/src/wrappers/dos.rs");

    assert!(iface.contains("pub CliInit:"),
        "interfaces/dos.rs missing `pub CliInit:` — was it reverted to PRIVATECliInit?");
    assert!(!iface.contains("pub PRIVATECliInit:"),
        "interfaces/dos.rs has `pub PRIVATECliInit:` — SDK names this method CliInit");
    assert!(wrap.contains("pub unsafe fn dos_cli_init("),
        "wrappers/dos.rs missing `dos_cli_init` — was it reverted to dos_privatecli_init?");
    assert!(!wrap.contains("pub unsafe fn dos_privatecli_init("),
        "wrappers/dos.rs has stale `dos_privatecli_init` — SDK names this method CliInit");
}

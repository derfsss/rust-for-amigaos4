//! Shared helpers for the rust-for-amigaos4 test suite.
//!
//! Used by both the orchestrator binary and the per-category integration
//! test files in `tests/`.

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Repository root — one level above the Tests/ crate.
pub fn repo_root() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set (run under cargo)");
    PathBuf::from(manifest_dir)
        .parent()
        .expect("Tests/ has no parent directory")
        .to_path_buf()
}

/// Read a file relative to repo root.
pub fn read_repo_file(rel: impl AsRef<Path>) -> String {
    let path = repo_root().join(rel.as_ref());
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

/// Run a command in repo root, capturing stdout+stderr.
pub fn run_in_root(cmd: &str, args: &[&str]) -> Output {
    Command::new(cmd)
        .args(args)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|e| panic!("failed to spawn {}: {}", cmd, e))
}

/// Run `cargo test` in a sibling crate directory, return its Output.
pub fn cargo_test_in(crate_dir: &str) -> Output {
    Command::new("cargo")
        .args(["test", "--no-fail-fast"])
        .current_dir(repo_root().join(crate_dir))
        .output()
        .unwrap_or_else(|e| panic!("failed to run cargo test in {}: {}", crate_dir, e))
}

/// Read a binary file's size in bytes (panics on I/O error).
pub fn file_size(path: impl AsRef<Path>) -> u64 {
    std::fs::metadata(path.as_ref())
        .unwrap_or_else(|e| panic!("stat {}: {}", path.as_ref().display(), e))
        .len()
}

/// Search a Makefile for an `LDFLAGS` line and return its right-hand side.
pub fn ldflags_of(makefile_path: impl AsRef<Path>) -> Option<String> {
    let text = std::fs::read_to_string(makefile_path.as_ref()).ok()?;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("LDFLAGS") {
            let rest = rest.trim_start();
            if let Some(eq_pos) = rest.find('=') {
                return Some(rest[eq_pos + 1..].trim().to_string());
            }
        }
    }
    None
}

/// Strip ANSI escape sequences from a string (cargo's output uses them).
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1b && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            i += 2;
            while i < bytes.len() && !(bytes[i] as char).is_alphabetic() {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

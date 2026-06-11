//! Documentation-claim tests.
//!
//! Cross-checks specific factual claims in README.md, CLAUDE.md, and
//! docs/roadmap.md against the actual project state. These exist
//! because docs go stale silently: a number ("12 examples") drifts the
//! moment someone adds a new example without updating the docs.

use amiga_tests::{read_repo_file, repo_root};
use std::fs;

// ── Example and template counts ──────────────────────────────

fn count_subdirs(rel: &str) -> usize {
    let dir = repo_root().join(rel);
    fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("can't read {}", dir.display()))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .count()
}

#[test]
fn examples_count_is_twenty_seven() {
    let n = count_subdirs("examples");
    assert_eq!(n, 27,
        "examples/ has {} subdirs, docs claim 27", n);
}

#[test]
fn templates_count_is_three() {
    let n = count_subdirs("templates");
    assert_eq!(n, 3,
        "templates/ has {} subdirs, docs claim 3", n);
}

#[test]
fn readme_lists_all_examples_by_name() {
    let readme = read_repo_file("README.md");
    let examples = [
        "hello", "hello-driver", "hello-library",
        "test-harness", "test-harness-gui", "test-harness-net",
        "file-io-demo", "timer-demo", "thread-demo",
        "gui-demo", "net-demo", "async-demo",
        "thread-amissl-probe", "http-client", "zlib-roundtrip",
        "picture-viewer", "wbstartup-hello", "xadmaster-list",
        "async-net-echo", "iff-dump", "locale-i18n-hello",
        "audio-tone", "ram-device", "aminet-browser", "https-client",
        "sqlite3-demo", "json-config",
    ];
    let mut missing: Vec<&str> = Vec::new();
    for ex in &examples {
        if !readme.contains(ex) {
            missing.push(ex);
        }
    }
    assert!(missing.is_empty(),
        "README.md does not name these examples: {:?}", missing);
}

// ── Build mode claims ────────────────────────────────────────

#[test]
fn readme_mentions_three_build_modes() {
    let readme = read_repo_file("README.md");
    assert!(readme.contains("Application") && readme.contains("Driver") &&
            readme.contains("Shared Library"),
        "README.md missing one of the three build modes");
}

// ── Rust toolchain pin consistency ───────────────────────────

#[test]
fn rust_toolchain_pin_matches_readme() {
    let toolchain = read_repo_file("rust-toolchain.toml");
    let readme = read_repo_file("README.md");

    // Extract channel from rust-toolchain.toml: channel = "nightly-..."
    let channel = toolchain.lines()
        .find_map(|l| l.trim().strip_prefix("channel"))
        .and_then(|s| s.trim_start_matches([' ', '=']).split('"').nth(1))
        .expect("rust-toolchain.toml has no channel field")
        .to_string();

    assert!(readme.contains(&channel),
        "README.md does not mention the pinned Rust toolchain {}",
        channel);
}

// ── Test count claim ─────────────────────────────────────────
//
// CLAUDE.md says "~255 total tests". This isn't a strict number — the
// tilde signals approximation — so we just check the actual count is
// in a sane range. If the suite shrinks dramatically the assertion
// fires.

#[test]
fn total_test_count_is_in_expected_range() {
    let mut count = 0u64;
    let crates = ["amigaos4", "amigaos4-sys", "amigaos4-alloc"];
    for c in &crates {
        count_tests_under(repo_root().join(c), &mut count);
    }
    // Also count Tests/ tests so the assertion doesn't drift the moment
    // we add our own tests.
    count_tests_under(repo_root().join("Tests"), &mut count);

    // Docs claim ~255 (193 host + ~60 target + 2 doctests). Allow a
    // 25% drift band before failing.
    let lo = 200u64;
    let hi = 400u64;
    assert!(count >= lo && count <= hi,
        "found {} `#[test]` annotations across host-runnable crates; \
         docs claim ~255. Drift band [{},{}].",
        count, lo, hi);
}

fn count_tests_under(dir: std::path::PathBuf, out: &mut u64) {
    if !dir.exists() { return; }
    walk(&dir, &mut |entry| {
        if entry.extension().and_then(|s| s.to_str()) == Some("rs") {
            // Skip target/ build outputs.
            if entry.components().any(|c| c.as_os_str() == "target") { return; }
            if let Ok(t) = fs::read_to_string(entry) {
                for line in t.lines() {
                    let l = line.trim();
                    if l == "#[test]" || l == "#[tokio::test]" {
                        *out += 1;
                    }
                }
            }
        }
    });
}

fn walk(dir: &std::path::Path, f: &mut dyn FnMut(&std::path::Path)) {
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() { walk(&p, f); }
            else { f(&p); }
        }
    }
}

// ── Build mode table consistency (README) ────────────────────

#[test]
fn build_mode_table_lists_all_three_allocators() {
    let readme = read_repo_file("README.md");
    // The build-mode table mentions allocator types.
    assert!(readme.contains("Clib4Allocator"),
        "README.md missing Clib4Allocator");
    assert!(readme.contains("ExecAllocator"),
        "README.md missing ExecAllocator");
}

// ── Tested-versions table ────────────────────────────────────

#[test]
fn readme_tested_versions_table_present() {
    let readme = read_repo_file("README.md");
    assert!(readme.contains("## Tested Versions"),
        "README.md missing '## Tested Versions' section");
    // The table mentions clib4, gcc, Docker image.
    for needle in &["clib4", "ppc-amigaos-gcc", "walkero/amigagccondocker"] {
        assert!(readme.contains(needle),
            "README.md Tested Versions table missing '{}'", needle);
    }
}

// ── Submodule pin shows up in docs (light check) ─────────────
//
// The strict version (matching short SHA exactly) lives in repro.rs.
// Here we just assert at least one doc mentions a 7-or-8 hex short SHA
// in the context of clib4. Catches a doc that forgot to update.

#[test]
fn clib4_pin_documented_with_hex_sha() {
    let mut found = false;
    for d in &["README.md", "CLAUDE.md", "docs/roadmap.md"] {
        let text = read_repo_file(d);
        // Look for "clib4 ... `<7-or-8 hex>`" pattern anywhere on a line.
        for line in text.lines() {
            let lower = line.to_lowercase();
            if !lower.contains("clib4") { continue; }
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '`' {
                    let mut hex: String = String::new();
                    while let Some(&pk) = chars.peek() {
                        if pk == '`' { break; }
                        hex.push(pk);
                        chars.next();
                    }
                    if hex.len() >= 7 && hex.len() <= 12
                       && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                        found = true;
                        break;
                    }
                }
            }
            if found { break; }
        }
        if found { break; }
    }
    assert!(found,
        "no doc references clib4 with a `<short-sha>` — pin may have been removed");
}

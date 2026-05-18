//! SDK signature audit — every vtable method must have a wrapper.
//!
//! For each `amigaos4-sys/src/wrappers/<name>.rs` file, parse the
//! corresponding `amigaos4-sys/src/interfaces/<name>.rs` to enumerate
//! the vtable methods, then verify each one has a Rust wrapper with
//! the expected name. Catches:
//!   - vtable methods added in a regen that didn't get wrappers
//!   - wrapper renames that broke the naming convention
//!   - wrappers deleted by mistake
//!
//! Method kinds and what they require:
//!   - regular methods (`pub Foo: unsafe extern "C" fn(...)`) — must
//!     have a wrapper.
//!   - base-interface methods (`Obtain`, `Release`) — no wrapper
//!     needed; framework owns lifecycle.
//!   - `<Name>_UNIMPLEMENTED: APTR` — placeholder slot, no wrapper.
//!   - varargs (`<Name>: APTR` for varargs entries) — handled via C
//!     glue, not Rust wrappers.
//!
//! Per the user's gap policy, the audit fails on any miss.

use amiga_tests::repo_root;
use std::collections::BTreeSet;
use std::fs;

/// Convert PascalCase to snake_case the way amigaos4-bindgen does.
///
/// Bindgen's rule: insert an underscore ONLY when transitioning from a
/// lowercase letter or digit to an uppercase letter. Acronyms and any
/// following word merge into a single token. Examples (verified against
/// the wrapper files):
///   `AddHead`       -> `add_head`
///   `RawDoFmt`      -> `raw_do_fmt`
///   `LockGUIPrefs`  -> `lock_guiprefs`
///   `OpenGUIPlugin` -> `open_guiplugin`
///   `ObtainIPluginList` -> `obtain_iplugin_list`
fn snake_case(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() * 2);
    for i in 0..chars.len() {
        let c = chars[i];
        if c.is_ascii_uppercase() && i > 0 {
            let prev = chars[i - 1];
            if prev.is_ascii_lowercase() || prev.is_ascii_digit() {
                out.push('_');
            }
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

#[derive(Debug, PartialEq, Eq)]
enum MethodKind {
    /// `pub Foo: unsafe extern "C" fn(...)` — wrapper required.
    Regular,
    /// `pub Foo_UNIMPLEMENTED: APTR` — placeholder slot.
    Unimplemented,
    /// `pub Foo: APTR, // (varargs)` — handled by C glue.
    Varargs,
}

/// Parsed vtable method.
#[derive(Debug)]
struct Method {
    name: String,
    kind: MethodKind,
}

/// Parse a vtable struct file. Looks for lines of the form:
///   `    pub <Name>: unsafe extern "C" fn(...)`   -> Regular
///   `    pub <Name>_UNIMPLEMENTED: APTR,`         -> Unimplemented
///   `    pub <Name>: APTR, ...`                   -> Varargs
fn parse_vtable_methods(text: &str) -> Vec<Method> {
    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("pub ") { continue; }
        let after = &trimmed[4..];
        let colon = match after.find(':') {
            Some(c) => c,
            None => continue,
        };
        let name = after[..colon].trim().to_string();
        let rhs = after[colon + 1..].trim();

        // Skip non-method fields like `data: InterfaceData`.
        if name == "data" { continue; }

        if rhs.starts_with("unsafe extern \"C\" fn") {
            out.push(Method { name, kind: MethodKind::Regular });
        } else if rhs.starts_with("APTR") {
            // APTR slot — either UNIMPLEMENTED or varargs.
            if name.ends_with("_UNIMPLEMENTED") {
                out.push(Method { name, kind: MethodKind::Unimplemented });
            } else {
                out.push(Method { name, kind: MethodKind::Varargs });
            }
        }
    }
    out
}

/// Extract every `pub unsafe fn <name>(` from a wrapper file.
fn parse_wrapper_names(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("pub unsafe fn ") {
            if let Some(paren) = rest.find('(') {
                out.insert(rest[..paren].trim().to_string());
            }
        }
    }
    out
}

/// Base interface methods inherited from the framework — they exist
/// in every vtable but the user-facing wrappers never expose them.
/// Most interfaces declare Expunge/Clone as `_UNIMPLEMENTED` slots
/// (filtered earlier as `MethodKind::Unimplemented`); some (e.g.
/// LowLevelIFace) implement them as real entries, which still get
/// no user-facing wrapper.
fn is_base_interface_method(name: &str) -> bool {
    matches!(name, "Obtain" | "Release" | "Expunge" | "Clone")
}

/// Interfaces we audit. Filename basename in both wrappers/ and
/// interfaces/ (uppercase mapping handled by parsing the struct decl
/// inside the wrapper file).
const INTERFACES: &[&str] = &[
    "application", "asl", "commodities", "datatypes", "diskfont", "dos",
    "exec", "gadtools", "graphics", "icon", "iffparse", "intuition",
    "keymap", "layers", "locale", "lowlevel", "misc", "popupmenu",
    "rexxsys", "timer", "utility", "version", "workbench",
];

/// One row of the audit table: which wrapper we expected for which
/// vtable method, and what we found.
struct AuditEntry {
    interface: &'static str,
    method: String,
    expected_wrapper: String,
}

fn audit_interface(basename: &'static str) -> Vec<AuditEntry> {
    let iface_path = repo_root()
        .join("amigaos4-sys/src/interfaces")
        .join(format!("{}.rs", basename));
    let wrap_path = repo_root()
        .join("amigaos4-sys/src/wrappers")
        .join(format!("{}.rs", basename));

    let iface_text = fs::read_to_string(&iface_path)
        .unwrap_or_else(|_| panic!("missing {}", iface_path.display()));
    let wrap_text = fs::read_to_string(&wrap_path)
        .unwrap_or_else(|_| panic!("missing {}", wrap_path.display()));

    let methods = parse_vtable_methods(&iface_text);
    let wrappers = parse_wrapper_names(&wrap_text);

    let mut gaps = Vec::new();
    for m in &methods {
        if m.kind != MethodKind::Regular { continue; }
        if is_base_interface_method(&m.name) { continue; }

        let expected = format!("{}_{}", basename, snake_case(&m.name));
        if !wrappers.contains(&expected) {
            gaps.push(AuditEntry {
                interface: basename,
                method: m.name.clone(),
                expected_wrapper: expected,
            });
        }
    }
    gaps
}

// ── Top-level audit ──────────────────────────────────────────

#[test]
fn every_vtable_method_has_a_wrapper() {
    let mut all_gaps: Vec<AuditEntry> = Vec::new();
    for iface in INTERFACES {
        all_gaps.extend(audit_interface(iface));
    }
    if !all_gaps.is_empty() {
        let mut msg = format!("\n{} regular vtable method(s) without a Rust wrapper:\n",
                              all_gaps.len());
        for e in &all_gaps {
            msg.push_str(&format!("  {}::{} -> expected wrapper `{}`\n",
                                  e.interface, e.method, e.expected_wrapper));
        }
        panic!("{}", msg);
    }
}

#[test]
fn audited_interfaces_all_have_source_files() {
    // Sanity: every entry in INTERFACES has both files. Catches typos
    // in the audit list before the real audit reports thousands of
    // "missing wrappers".
    for iface in INTERFACES {
        let i = repo_root().join("amigaos4-sys/src/interfaces").join(format!("{}.rs", iface));
        let w = repo_root().join("amigaos4-sys/src/wrappers").join(format!("{}.rs", iface));
        assert!(i.exists(), "missing {}", i.display());
        assert!(w.exists(), "missing {}", w.display());
    }
}

#[test]
fn audit_covers_every_wrapper_file() {
    // Every wrappers/*.rs file (except the special mod.rs) must be in
    // INTERFACES. Catches the case where someone adds a new wrapper
    // file but forgets to register it for the audit.
    let wrappers_dir = repo_root().join("amigaos4-sys/src/wrappers");
    let mut on_disk: BTreeSet<String> = BTreeSet::new();
    for entry in fs::read_dir(&wrappers_dir).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if !name.ends_with(".rs") { continue; }
        let stem = name.trim_end_matches(".rs").to_string();
        if stem == "mod" { continue; }
        on_disk.insert(stem);
    }
    let registered: BTreeSet<String> = INTERFACES.iter().map(|s| s.to_string()).collect();

    let unregistered: Vec<&String> = on_disk.difference(&registered).collect();
    let stale: Vec<&String> = registered.difference(&on_disk).collect();

    assert!(unregistered.is_empty(),
        "wrapper file(s) exist on disk but missing from INTERFACES audit list: {:?}",
        unregistered);
    assert!(stale.is_empty(),
        "INTERFACES audit list references missing wrapper file(s): {:?}",
        stale);
}

// ── 129-interface coverage ───────────────────────────────────
//
// Only 19 interfaces have Rust convenience wrappers; the other 110
// are raw FFI struct bindings usable via direct vtable dispatch. The
// audit above covers the 19 in depth. This test covers the other 110
// at the level of "every feature flag has an interface file".

#[test]
fn every_amigaos4_sys_feature_has_an_interface_file() {
    let cargo_toml = fs::read_to_string(
        repo_root().join("amigaos4-sys/Cargo.toml")
    ).expect("amigaos4-sys/Cargo.toml missing");

    let interfaces_dir = repo_root().join("amigaos4-sys/src/interfaces");
    let on_disk: BTreeSet<String> = fs::read_dir(&interfaces_dir).unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.ends_with(".rs") && n != "mod.rs")
        .map(|n| n.trim_end_matches(".rs").to_string())
        .collect();

    let mut features: BTreeSet<String> = BTreeSet::new();
    let mut in_features = false;
    for line in cargo_toml.lines() {
        let l = line.trim();
        if l == "[features]" { in_features = true; continue; }
        if l.starts_with('[') && l != "[features]" { in_features = false; }
        if !in_features { continue; }
        if let Some(eq) = l.find('=') {
            let name = l[..eq].trim();
            let rhs = l[eq + 1..].trim();
            // Single-interface feature: `name = []`. Skip aggregates
            // like `default` and `full-sdk`.
            if rhs == "[]" && !name.is_empty() && !name.starts_with('#')
               && name != "default" {
                features.insert(name.to_string());
            }
        }
    }

    let missing_files: Vec<&String> = features.difference(&on_disk).collect();
    let orphan_files: Vec<&String> = on_disk.difference(&features).collect();

    assert!(missing_files.is_empty(),
        "{} feature flag(s) without a matching interfaces/<name>.rs:\n  - {}",
        missing_files.len(),
        missing_files.iter().map(|s| s.as_str())
                     .collect::<Vec<_>>().join("\n  - "));
    assert!(orphan_files.is_empty(),
        "{} interfaces/<name>.rs file(s) without a feature flag:\n  - {}",
        orphan_files.len(),
        orphan_files.iter().map(|s| s.as_str())
                    .collect::<Vec<_>>().join("\n  - "));
}

// ── snake_case unit tests ────────────────────────────────────

#[test]
fn snake_case_simple() {
    assert_eq!(snake_case("AddHead"), "add_head");
    assert_eq!(snake_case("Obtain"), "obtain");
    assert_eq!(snake_case("AllocVec"), "alloc_vec");
}

#[test]
fn snake_case_acronyms() {
    // Bindgen merges acronyms with the following word — see snake_case docs.
    assert_eq!(snake_case("RawDoFmt"), "raw_do_fmt");
    assert_eq!(snake_case("LockGUIPrefs"), "lock_guiprefs");
    assert_eq!(snake_case("OpenGUIPlugin"), "open_guiplugin");
    assert_eq!(snake_case("ObtainIPluginList"), "obtain_iplugin_list");
    assert_eq!(snake_case("MakeInterface"), "make_interface");
}

#[test]
fn snake_case_with_digits() {
    assert_eq!(snake_case("Foo32Bar"), "foo32_bar");
}

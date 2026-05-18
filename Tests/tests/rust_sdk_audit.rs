//! SDK cross-check — every method declared in the AmigaOS 4 SDK
//! interface header must be present in the Rust binding.
//!
//! Runs opportunistically: if the SDK is found at
//! `C:\msys64\home\rich_\sdk` (or `$AMIGA_SDK\include\include_h\interfaces`),
//! the audit executes and fails the build on any gap. If the SDK isn't
//! available, every test in this file passes with a notice — the suite
//! shouldn't be useless on machines that don't host the SDK.
//!
//! What it cross-checks (per the user's "fail on gap" policy):
//!   - Regular methods: every SDK `APICALL ... (*Name)(...)` must be a
//!     `pub Name: unsafe extern "C" fn(...)` field in the Rust struct.
//!     Skipped: `Obtain` and `Release` (framework, no wrapper expected).
//!   - Varargs methods (SDK ends in `, ...)`) must be present as `pub
//!     Name: APTR,` in the Rust struct.
//!
//! What it does NOT check (yet):
//!   - Argument count or types (would require parsing C declarations
//!     fully; the existing rust_signature_audit verifies wrapper names
//!     against vtable field names, which catches argument-count drift
//!     indirectly at link time on PPC).

use amiga_tests::repo_root;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

const ENV_VAR: &str = "AMIGA_SDK";
const DEFAULT_SDK: &str = r"C:\msys64\home\rich_\sdk";

/// Resolve the SDK interfaces directory, if available.
fn sdk_interfaces_dir() -> Option<PathBuf> {
    let root = std::env::var(ENV_VAR).ok().map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_SDK));
    let ifaces = root.join("include").join("include_h").join("interfaces");
    if ifaces.is_dir() { Some(ifaces) } else { None }
}

/// `(rust_basename, rust_struct_name, sdk_filename_override)`. The
/// SDK filename matches the Rust basename unless explicitly overridden.
const INTERFACES: &[(&str, &str, Option<&str>)] = &[
    ("application", "ApplicationIFace", None),
    ("asl", "AslIFace", None),
    ("commodities", "CommoditiesIFace", None),
    ("datatypes", "DataTypesIFace", None),
    ("diskfont", "DiskfontIFace", None),
    ("dos", "DOSIFace", None),
    ("exec", "ExecIFace", None),
    ("gadtools", "GadToolsIFace", None),
    ("graphics", "GraphicsIFace", None),
    ("icon", "IconIFace", None),
    ("iffparse", "IFFParseIFace", None),
    ("intuition", "IntuitionIFace", None),
    ("keymap", "KeymapIFace", None),
    ("layers", "LayersIFace", None),
    ("locale", "LocaleIFace", None),
    ("lowlevel", "LowLevelIFace", None),
    ("misc", "MiscIFace", None),
    ("popupmenu", "PopupMenuIFace", None),
    ("rexxsys", "RexxSysIFace", Some("rexxsyslib")),
    ("timer", "TimerIFace", None),
    ("utility", "UtilityIFace", None),
    ("version", "VersionIFace", None),
    ("workbench", "WorkbenchIFace", Some("wb")),
];

/// Base methods inherited from the Interface framework; the SDK header
/// declares them, but they have no corresponding user-callable wrapper.
fn is_base_method(name: &str) -> bool {
    matches!(name, "Obtain" | "Release" | "Expunge" | "Clone")
}

// ── SDK header parser ────────────────────────────────────────

#[derive(Debug, Default)]
struct Methods {
    regular: BTreeSet<String>,
    varargs: BTreeSet<String>,
}

/// Find the body of `struct <name>` (between the opening `{` and the
/// matching closing `};`). Returns empty slice if not found.
fn extract_struct_body<'a>(text: &'a str, struct_name: &str) -> &'a str {
    let needle = format!("struct {}", struct_name);
    let start = match text.find(&needle) {
        Some(i) => i,
        None => return "",
    };
    let end = text[start..].find("\n};")
        .map(|e| start + e)
        .unwrap_or(text.len());
    &text[start..end]
}

/// Parse the SDK struct body for vtable method names. Format varies
/// slightly: both `RetType APICALL (*Name)(...)` and `APICALL RetType
/// (*Name)(...)` appear across the SDK. Varargs methods end with
/// `, ...)` in the argument list (track via balanced paren scan).
fn parse_sdk_methods(body: &str) -> Methods {
    let mut out = Methods::default();
    let bytes = body.as_bytes();
    let pat = b"APICALL";
    let mut i = 0;
    while i + pat.len() <= bytes.len() {
        if &bytes[i..i + pat.len()] != pat { i += 1; continue; }
        let mut j = i + pat.len();
        // Find `(*Name)`.
        while j < bytes.len() && bytes[j] != b'(' { j += 1; }
        if j >= bytes.len() { break; }
        // Must be `(*` — otherwise this APICALL token is in a comment
        // or something else.
        if j + 1 >= bytes.len() || bytes[j + 1] != b'*' { i = j; continue; }
        let name_start = j + 2;
        let mut name_end = name_start;
        while name_end < bytes.len() &&
              (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_') {
            name_end += 1;
        }
        if name_end == name_start { i = name_end; continue; }
        let name = std::str::from_utf8(&bytes[name_start..name_end])
            .unwrap_or("").to_string();
        // Skip the closing `)` of `(*Name)`.
        if name_end >= bytes.len() || bytes[name_end] != b')' { i = name_end; continue; }
        let args_start = name_end + 1;
        // Walk balanced parens to find the arg-list close.
        let mut depth = 0;
        let mut k = args_start;
        let mut args_open = None;
        while k < bytes.len() {
            match bytes[k] {
                b'(' => {
                    depth += 1;
                    if depth == 1 { args_open = Some(k + 1); }
                }
                b')' => {
                    depth -= 1;
                    if depth == 0 { break; }
                }
                _ => {}
            }
            k += 1;
        }
        if depth != 0 { i = args_start; continue; }
        let args = if let Some(a) = args_open {
            std::str::from_utf8(&bytes[a..k]).unwrap_or("")
        } else { "" };

        if args.contains("...") {
            out.varargs.insert(name);
        } else {
            out.regular.insert(name);
        }

        i = k + 1;
    }
    out
}

// ── Rust struct parser ───────────────────────────────────────

#[derive(Default)]
struct RustFields {
    regular: BTreeSet<String>,
    varargs: BTreeSet<String>,
}

fn parse_rust_fields(text: &str) -> RustFields {
    let mut out = RustFields::default();
    for line in text.lines() {
        let l = line.trim_start();
        if !l.starts_with("pub ") { continue; }
        let after = &l[4..];
        let colon = match after.find(':') {
            Some(c) => c,
            None => continue,
        };
        let name = after[..colon].trim().to_string();
        if name == "data" { continue; }
        let rhs = after[colon + 1..].trim_start();
        if rhs.starts_with("unsafe extern \"C\" fn") {
            out.regular.insert(name);
        } else if rhs.starts_with("APTR") && !name.ends_with("_UNIMPLEMENTED") {
            out.varargs.insert(name);
        }
    }
    out
}

// ── The audit ────────────────────────────────────────────────

fn audit_one(
    sdk_iface_dir: &PathBuf,
    rs_iface_dir: &PathBuf,
    basename: &str,
    struct_name: &str,
    sdk_override: Option<&str>,
) -> Result<(), String> {
    let sdk_stem = sdk_override.unwrap_or(basename);
    let sdk_path = sdk_iface_dir.join(format!("{}.h", sdk_stem));
    let rs_path = rs_iface_dir.join(format!("{}.rs", basename));

    let sdk_text = fs::read_to_string(&sdk_path)
        .map_err(|e| format!("can't read {}: {}", sdk_path.display(), e))?;
    let rs_text = fs::read_to_string(&rs_path)
        .map_err(|e| format!("can't read {}: {}", rs_path.display(), e))?;

    let sdk_methods = parse_sdk_methods(extract_struct_body(&sdk_text, struct_name));
    let rust_fields = parse_rust_fields(&rs_text);

    let missing_reg: Vec<&String> = sdk_methods.regular.iter()
        .filter(|m| !is_base_method(m))
        .filter(|m| !rust_fields.regular.contains(m.as_str()))
        .collect();
    let missing_var: Vec<&String> = sdk_methods.varargs.iter()
        .filter(|m| !rust_fields.varargs.contains(m.as_str()))
        .collect();

    if missing_reg.is_empty() && missing_var.is_empty() {
        return Ok(());
    }

    let mut msg = format!("\n  {} ({}):", basename, struct_name);
    if !missing_reg.is_empty() {
        msg.push_str(&format!("\n    regular methods in SDK but missing from Rust ({}):",
                              missing_reg.len()));
        for n in &missing_reg {
            msg.push_str(&format!("\n      - {}", n));
        }
    }
    if !missing_var.is_empty() {
        msg.push_str(&format!("\n    varargs methods in SDK but missing from Rust ({}):",
                              missing_var.len()));
        for n in &missing_var {
            msg.push_str(&format!("\n      - {}", n));
        }
    }
    Err(msg)
}

#[test]
fn every_sdk_method_has_a_rust_binding() {
    let sdk_iface_dir = match sdk_interfaces_dir() {
        Some(d) => d,
        None => {
            eprintln!("[skip] AmigaOS 4 SDK not found at {} (set {} to override).",
                      DEFAULT_SDK, ENV_VAR);
            return;
        }
    };
    let rs_iface_dir = repo_root().join("amigaos4-sys/src/interfaces");

    let mut errors: Vec<String> = Vec::new();
    for &(basename, struct_name, sdk_override) in INTERFACES {
        if let Err(msg) = audit_one(&sdk_iface_dir, &rs_iface_dir,
                                     basename, struct_name, sdk_override) {
            errors.push(msg);
        }
    }

    if !errors.is_empty() {
        let total: usize = errors.iter()
            .map(|e| e.matches("      -").count()).sum();
        panic!("{} SDK method(s) missing from Rust bindings:{}",
               total, errors.join(""));
    }
}

// ── Unit tests for the parsers ───────────────────────────────

#[test]
fn sdk_parser_handles_both_apicall_orders() {
    // exec.h-style: "RetType APICALL (*Name)(...)"
    let exec_style = r#"
struct Foo {
    ULONG APICALL (*Obtain)(struct Foo *Self);
    void APICALL (*Bar)(struct Foo *Self, int x);
};
"#;
    let m = parse_sdk_methods(extract_struct_body(exec_style, "Foo"));
    assert!(m.regular.contains("Obtain"));
    assert!(m.regular.contains("Bar"));

    // dos.h-style: "APICALL RetType (*Name)(...)"
    let dos_style = r#"
struct Foo {
    APICALL ULONG (*Obtain)(struct Foo *Self);
    APICALL void (*Bar)(struct Foo *Self, int x);
};
"#;
    let m = parse_sdk_methods(extract_struct_body(dos_style, "Foo"));
    assert!(m.regular.contains("Obtain"));
    assert!(m.regular.contains("Bar"));
}

#[test]
fn sdk_parser_classifies_varargs_correctly() {
    let text = r#"
struct Foo {
    void APICALL (*MakeTags)(struct Foo *Self, const struct TagItem * tags);
    void APICALL (*MakeTagsVar)(struct Foo *Self, ...);
};
"#;
    let m = parse_sdk_methods(extract_struct_body(text, "Foo"));
    assert!(m.regular.contains("MakeTags"));
    assert!(m.varargs.contains("MakeTagsVar"));
    assert!(!m.regular.contains("MakeTagsVar"));
}

#[test]
fn sdk_parser_handles_nested_parens_in_args() {
    // RawDoFmt has a function-pointer argument with its own parens.
    let text = r#"
struct Foo {
    APTR APICALL (*RawDoFmt)(struct Foo *Self, CONST_STRPTR fmt, CONST_APTR data, void (*PutChProc)(), APTR PutChData);
};
"#;
    let m = parse_sdk_methods(extract_struct_body(text, "Foo"));
    assert!(m.regular.contains("RawDoFmt"),
        "RawDoFmt should be parsed despite nested parens in its arg list");
}

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
    ("acpi", "ACPIIFace", None),
    ("amigaguide", "AmigaGuideIFace", None),
    ("amigainput", "AIN_IFace", None),
    ("amisslmaster", "AmiSSLMasterIFace", None),
    ("application", "ApplicationIFace", None),
    ("arexx", "ARexxIFace", None),
    ("asl", "AslIFace", None),
    ("battclock", "BattClockIFace", None),
    ("battmem", "BattMemIFace", None),
    ("bevel", "BevelIFace", None),
    ("bitmap_gc", "BitMapIFace", Some("bitmap")),
    ("blankermodule", "BlankerModuleIFace", None),
    ("bootsd", "BootSDIFace", None),
    ("btree", "BTreeIFace", None),
    ("bullet", "BulletIFace", None),
    ("button", "ButtonIFace", None),
    ("bzip2", "BZip2IFace", None),
    ("card", "CardIFace", Some("cardres")),
    ("catweasel", "CWIFace", None),
    ("checkbox", "CheckBoxIFace", None),
    ("chooser", "ChooserIFace", None),
    ("cia", "CIAIFace", None),
    ("clicktab", "ClickTabIFace", None),
    ("colorwheel", "ColorWheelIFace", None),
    ("commodities", "CommoditiesIFace", None),
    ("console", "ConsoleIFace", None),
    ("cyrus", "CyrusIFace", None),
    ("datatypes", "DataTypesIFace", None),
    ("datebrowser", "DateBrowserIFace", None),
    ("debug", "DebugIFace", Some("exec")),
    ("diffview", "DiffViewIFace", None),
    ("diskfont", "DiskfontIFace", None),
    ("diskio", "DiskIOIFace", None),
    ("docky", "DockyIFace", None),
    ("dos", "DOSIFace", None),
    ("drawlist", "DrawListIFace", None),
    ("elf", "ElfIFace", None),
    ("exec", "ExecIFace", None),
    ("expansion", "ExpansionIFace", None),
    ("filesysbox", "FileSysBoxIFace", None),
    ("filler", "FillerIFace", None),
    ("fsldma", "fslDMAIFace", None),
    ("fuelgauge", "FuelGaugeIFace", None),
    ("gadtools", "GadToolsIFace", None),
    ("getcolor", "GetColorIFace", None),
    ("getfile", "GetFileIFace", None),
    ("getfont", "GetFontIFace", None),
    ("getscreenmode", "GetScreenModeIFace", None),
    ("glyph", "GlyphIFace", None),
    ("graphics", "GraphicsIFace", None),
    ("hunk", "HunkIFace", None),
    ("i2c", "I2CIFace", None),
    ("icon", "IconIFace", None),
    ("iconmodule", "IconModuleIFace", None),
    ("iffparse", "IFFParseIFace", None),
    ("input", "InputIFace", None),
    ("integer", "IntegerIFace", None),
    ("intuition", "IntuitionIFace", None),
    ("keymap", "KeymapIFace", None),
    ("label", "LabelIFace", None),
    ("layers", "LayersIFace", None),
    ("layout", "LayoutIFace", None),
    ("listbrowser", "ListBrowserIFace", None),
    ("locale", "LocaleIFace", None),
    ("lowlevel", "LowLevelIFace", None),
    ("lzma", "LZMAIFace", None),
    ("misc", "MiscIFace", None),
    ("mmu", "MMUIFace", Some("exec")),
    ("mounter", "MounterIFace", None),
    ("nv", "NVIFace", Some("nonvolatile")),
    ("palette", "PaletteIFace", None),
    ("partition", "PartitionIFace", None),
    ("pci", "PCIIFace", Some("expansion")),
    ("penmap", "PenMapIFace", None),
    ("perfmon", "PerformanceMonitorIFace", Some("performancemonitor")),
    ("picture", "PictureIFace", None),
    ("popupmenu", "PopupMenuIFace", None),
    ("potgo", "PotgoIFace", None),
    ("radiobutton", "RadioButtonIFace", None),
    ("ramdrive", "RamdriveIFace", None),
    ("realtime", "RealTimeIFace", None),
    ("requester", "RequesterIFace", None),
    ("resource", "ResourceIFace", None),
    ("rexxsys", "RexxSysIFace", Some("rexxsyslib")),
    ("screenblanker", "ScreenBlankerIFace", None),
    ("scroller", "ScrollerIFace", None),
    ("sketchboard", "SketchBoardIFace", None),
    ("slider", "SliderIFace", None),
    ("space", "SpaceIFace", None),
    ("speedbar", "SpeedBarIFace", None),
    ("string_gc", "StringIFace", Some("string")),
    ("textclip", "TextClipIFace", None),
    ("texteditor", "TextEditorIFace", None),
    ("timer", "TimerIFace", None),
    ("timesync", "TimesyncIFace", None),
    ("timezone", "TimezoneIFace", None),
    ("usbfd", "USBFDIFace", None),
    ("usbhcd", "USBHCDIFace", None),
    ("utility", "UtilityIFace", None),
    ("version", "VersionIFace", None),
    ("virtual_gc", "VirtualIFace", Some("virtual")),
    ("window", "WindowIFace", None),
    ("workbench", "WorkbenchIFace", Some("wb")),
    ("xena", "XenaIFace", None),
];

/// Base methods inherited from the Interface framework, plus
/// `Reserved<N>` placeholder slots that several interfaces use. None
/// have user-callable wrappers.
fn is_base_method(name: &str) -> bool {
    if matches!(name, "Obtain" | "Release" | "Expunge" | "Clone") {
        return true;
    }
    if let Some(rest) = name.strip_prefix("Reserved") {
        if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    false
}

// ── SDK header parser ────────────────────────────────────────

#[derive(Debug, Default)]
struct Methods {
    regular: BTreeSet<String>,
    varargs: BTreeSet<String>,
}

/// Find the body of `struct <name>` (between its declaration and the
/// matching closing `};`). Returns empty slice if not found.
///
/// Must distinguish a struct *declaration* from incidental uses of
/// the name as a *type* in another struct's field (e.g. `struct
/// I2CIFace * APICALL (*GetBusByNumber)(...)` inside
/// `I2CResourceIFace`). The declaration is followed by whitespace
/// (often a newline) then `{`; type uses are followed by `*` or `,`
/// or other punctuation. Scan for occurrences of `struct <name>` and
/// pick the first one whose trailing context looks like an opening
/// brace.
fn extract_struct_body<'a>(text: &'a str, struct_name: &str) -> &'a str {
    let needle = format!("struct {}", struct_name);
    let mut search_from = 0;
    let start = loop {
        let Some(rel) = text[search_from..].find(&needle) else { return ""; };
        let abs = search_from + rel;
        let after = abs + needle.len();
        // Look past whitespace; require the next non-whitespace char to be `{`.
        let mut k = after;
        let bytes = text.as_bytes();
        while k < bytes.len() && (bytes[k] == b' ' || bytes[k] == b'\t'
                                  || bytes[k] == b'\r' || bytes[k] == b'\n') {
            k += 1;
        }
        if k < bytes.len() && bytes[k] == b'{' {
            break abs;
        }
        // Not a declaration — keep searching.
        search_from = after;
    };
    let end = text[start..].find("\n};")
        .map(|e| start + e)
        .unwrap_or(text.len());
    &text[start..end]
}

/// Parse the SDK struct body for vtable method names.
///
/// Anchors on the function-pointer syntax `(*Name)(args)` rather than
/// the `APICALL` keyword. Most SDK headers attach APICALL to every
/// method, but a few (notably `lzma.h`) omit it — those methods expose
/// a standard-C ABI library directly and don't use the AmigaOS
/// calling convention. Walking the bytes directly catches both.
///
/// Method declarations live at struct-body depth 0; the `(*Name)` may
/// also appear inside an argument list (e.g. `void (*PutChProc)()` as
/// an argument to RawDoFmt) and must be ignored — paren depth tracking
/// handles that.
fn parse_sdk_methods(body: &str) -> Methods {
    let mut out = Methods::default();
    let bytes = body.as_bytes();
    let mut depth: i32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'(' && depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            // Try to parse `(*Name)(args)`. Extract Name.
            let name_start = i + 2;
            let mut name_end = name_start;
            while name_end < bytes.len() &&
                  (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_') {
                name_end += 1;
            }
            if name_end > name_start &&
               name_end < bytes.len() && bytes[name_end] == b')' {
                // Skip whitespace between `)` and the args `(`.
                let mut k = name_end + 1;
                while k < bytes.len() && bytes[k].is_ascii_whitespace() { k += 1; }
                if k < bytes.len() && bytes[k] == b'(' {
                    // This is a method declaration. Walk args with
                    // balanced parens to find the closing `)` and
                    // detect varargs.
                    let args_open = k + 1;
                    let mut arg_depth = 1;
                    let mut m = args_open;
                    while m < bytes.len() && arg_depth > 0 {
                        match bytes[m] {
                            b'(' => arg_depth += 1,
                            b')' => arg_depth -= 1,
                            _ => {}
                        }
                        if arg_depth > 0 { m += 1; }
                    }
                    if arg_depth == 0 {
                        let args = std::str::from_utf8(&bytes[args_open..m])
                            .unwrap_or("");
                        let name = std::str::from_utf8(&bytes[name_start..name_end])
                            .unwrap_or("").to_string();
                        if args.contains("...") {
                            out.varargs.insert(name);
                        } else {
                            out.regular.insert(name);
                        }
                        i = m + 1;
                        continue;
                    }
                }
            }
        }
        match c {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        i += 1;
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
fn sdk_parser_extracts_correct_struct_when_name_appears_as_field_type() {
    // i2c.h has two structs and the second is `I2CIFace`. The first
    // (I2CResourceIFace) has methods that take `struct I2CIFace *` as
    // arguments. A naive needle match picks up the I2CResourceIFace
    // body when asked for I2CIFace, mixing the wrong methods in.
    let text = r#"
struct ResourceFace
{
    struct InterfaceData Data;
    uint32 APICALL (*Obtain)(struct ResourceFace *Self);
    struct TargetFace * APICALL (*GetByNumber)(struct ResourceFace *Self, uint32 num);
};

struct TargetFace
{
    struct InterfaceData Data;
    uint32 APICALL (*Obtain)(struct TargetFace *Self);
    int32 APICALL (*Probe)(struct TargetFace *Self, uint32 addr);
    void APICALL (*Lock)(struct TargetFace *Self);
};
"#;
    let m = parse_sdk_methods(extract_struct_body(text, "TargetFace"));
    assert!(m.regular.contains("Probe"),
        "should pick up TargetFace's own methods");
    assert!(m.regular.contains("Lock"),
        "should pick up TargetFace's own methods");
    assert!(!m.regular.contains("GetByNumber"),
        "must not include ResourceFace's GetByNumber, which references TargetFace as a field type");
}

#[test]
fn sdk_parser_handles_methods_without_apicall() {
    // lzma.h-style: every lzma_* method-pointer omits APICALL because
    // it exposes liblzma's plain C ABI. Only the framework methods
    // (Obtain/Release/Expunge/Clone) keep APICALL.
    let text = r#"
struct Foo {
    struct InterfaceData Data;

    uint32 APICALL (*Obtain)(struct Foo *Self);
    void APICALL (*Release)(struct Foo *Self);
    lzma_ret (*lzma_code)(lzma_stream *strm, lzma_action action);
    void (*lzma_end)(lzma_stream *strm);
    uint64_t (*lzma_memusage)(const lzma_stream *strm);
};
"#;
    let m = parse_sdk_methods(extract_struct_body(text, "Foo"));
    assert!(m.regular.contains("Obtain"));
    assert!(m.regular.contains("Release"));
    assert!(m.regular.contains("lzma_code"),
        "non-APICALL method `lzma_code` should be parsed");
    assert!(m.regular.contains("lzma_end"));
    assert!(m.regular.contains("lzma_memusage"));
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
    assert!(!m.regular.contains("PutChProc"),
        "the inner fn-pointer arg `PutChProc` must NOT be treated as a method");
}

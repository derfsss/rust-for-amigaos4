//
// Aminet Browser — the rust-for-amigaos4 dogfood app.
//
// Exercises most of the crate in one real program:
//   application.library registration (single instance)
//   menuclass menu bar (Project: About / Quit)
//   listbrowser gadget listing recent Aminet uploads
//   HTTP client (with redirects) fetching the RECENT index + files
//   ASL save requester choosing where to write a download
//   fs writing the downloaded file
//   EasyRequest result dialogs
//
// Network use is best-effort: if aminet.net is unreachable (offline
// QEMU), a built-in sample list is shown and downloads are skipped.
//

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::amstr;
use amigaos4::application::AppRegistration;
use amigaos4::asl::{FileMode, FileRequester};
use amigaos4::menu::MenuBuilder;
use amigaos4::reaction::*;
use amigaos4::requester::easy_request;
use amigaos4::tag::TagListBuilder;
use amigaos4::window::*;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

const GID_LIST: u32 = 1;
const GID_DOWNLOAD: u32 = 2;
const GID_QUIT: u32 = 3;

const MID_ABOUT: u32 = 100;
const MID_QUIT: u32 = 101;

const AMINET_HOST: &[u8] = amstr!("aminet.net");

/// One entry from the RECENT index: "path/file.lha  dir  size  description".
struct Entry {
    /// e.g. "util/misc/Tool.lha"
    path: Vec<u8>,
    /// Display line for the listbrowser (null-terminated).
    label: Vec<u8>,
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== Aminet Browser ===");

    // Single-instance registration; non-fatal if application.library
    // is missing (e.g. stripped-down install).
    let _app = AppRegistration::new(amstr!("AminetBrowser"))
        .unique()
        .register()
        .ok();

    // Fetch the recent-uploads index (best-effort).
    let (entries, online) = fetch_recent();
    amigaos4::serial_println!(
        "aminet-browser: {} entries ({})",
        entries.len(),
        if online { "live" } else { "offline sample" }
    );

    // Listbrowser nodes from the entries.
    let mut nodes = match ListBrowserNodes::new() {
        Ok(n) => n,
        Err(e) => {
            amigaos4::serial_println!("listbrowser.gadget unavailable: {}", e);
            return 1;
        }
    };
    for entry in &entries {
        if nodes.push(&entry.label).is_err() {
            amigaos4::serial_println!("node alloc failed");
            return 1;
        }
    }

    // Layout: listbrowser over a button row.
    let layout = match build_ui(&nodes) {
        Ok(l) => l,
        Err(e) => {
            amigaos4::serial_println!("layout failed: {}", e);
            return 1;
        }
    };

    // Menu bar.
    let strip = match MenuBuilder::new()
        .menu(amstr!("Project"))
        .item(MID_ABOUT, amstr!("About..."))
        .separator()
        .item_key(MID_QUIT, amstr!("Quit"), amstr!("Q"))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            amigaos4::serial_println!("menu failed: {}", e);
            return 1;
        }
    };

    let win_tags = TagListBuilder::new()
        .tag(WA_LEFT, 80)
        .tag(WA_TOP, 60)
        .tag(WA_WIDTH, 460)
        .tag(WA_HEIGHT, 280)
        .tag(WA_TITLE, amstr!("Aminet Browser (Rust)").as_ptr() as u32)
        .tag(
            WA_IDCMP,
            IDCMP_CLOSEWINDOW | IDCMP_GADGETUP | IDCMP_MENUPICK | IDCMP_VANILLAKEY,
        )
        .tag(WA_CLOSE_GADGET, 1)
        .tag(WA_DRAG_BAR, 1)
        .tag(WA_DEPTH_GADGET, 1)
        .tag(WA_SIZE_GADGET, 1)
        .tag(WA_ACTIVATE, 1)
        .tag(WA_GADGETS, layout.into_raw() as u32)
        .build();

    let win = match AmigaWindow::open(&win_tags) {
        Ok(w) => w,
        Err(e) => {
            amigaos4::serial_println!("window failed: {}", e);
            return 1;
        }
    };
    let _ = strip.attach(&win);

    let mut selected: usize = 0;
    event_loop(&win, |event| match event {
        Event::Close => false,
        Event::GadgetUp { id, code } => match id {
            GID_LIST => {
                selected = code as usize;
                true
            }
            GID_DOWNLOAD => {
                if let Some(entry) = entries.get(selected) {
                    download(entry, online);
                }
                true
            }
            GID_QUIT => false,
            _ => true,
        },
        Event::MenuPick => {
            let mut quit = false;
            strip.each_select(|mid| match mid {
                MID_ABOUT => {
                    easy_request(
                        amstr!("About"),
                        amstr!("Aminet Browser - written in Rust\nwith the amigaos4 crate."),
                        amstr!("OK"),
                    );
                }
                MID_QUIT => quit = true,
                _ => {}
            });
            !quit
        }
        Event::Key(27) => false,
        _ => true,
    });

    strip.detach(&win);
    drop(win);
    // `nodes` must outlive the window/gadget; dropped here, after.
    drop(nodes);

    amigaos4::serial_println!("=== Aminet Browser done ===");
    0
}

// ---------------------------------------------------------------------------
// UI
// ---------------------------------------------------------------------------

fn build_ui(nodes: &ListBrowserNodes) -> amigaos4::Result<amigaos4::boopsi::AmigaObject> {
    let list = listbrowser(GID_LIST, nodes)?;
    let dl_btn = button(GID_DOWNLOAD, amstr!("_Download..."))?;
    let quit_btn = button(GID_QUIT, amstr!("_Quit"))?;

    let buttons = LayoutBuilder::horizontal()
        .add(dl_btn)
        .add(quit_btn)
        .build()?;

    LayoutBuilder::vertical().add(list).add(buttons).build()
}

// ---------------------------------------------------------------------------
// Aminet access
// ---------------------------------------------------------------------------

/// Fetch http://aminet.net/info/recent.txt and parse up to 20 entries;
/// fall back to a static sample when offline.
fn fetch_recent() -> (Vec<Entry>, bool) {
    if let Ok(resp) = amigaos4::http::get(AMINET_HOST, 80, amstr!("/info/recent.txt")) {
        if resp.status_code == 200 {
            let entries = parse_recent(&resp.body);
            if !entries.is_empty() {
                return (entries, true);
            }
        }
    }

    // Offline fallback: a recognisable sample list.
    let sample: &[&[u8]] = &[
        b"util/misc/ExampleTool.lha   (offline sample)",
        b"game/think/RustMine.lha     (offline sample)",
        b"comm/net/Fetcher.lha        (offline sample)",
    ];
    let entries = sample
        .iter()
        .map(|s| {
            let mut label = s.to_vec();
            label.push(0);
            Entry {
                path: first_word(s).to_vec(),
                label,
            }
        })
        .collect();
    (entries, false)
}

/// Parse RECENT-index lines: first whitespace-separated word is the
/// file path, the whole line becomes the label. Skips blanks/comments.
fn parse_recent(body: &[u8]) -> Vec<Entry> {
    let mut entries = Vec::new();
    for line in body.split(|&b| b == b'\n') {
        let line = trim(line);
        if line.is_empty() || line[0] == b'#' || line[0] == b'|' {
            continue;
        }
        let path = first_word(line);
        if path.is_empty() || !path.contains(&b'/') {
            continue;
        }
        let mut label = line.to_vec();
        if label.len() > 60 {
            label.truncate(60);
        }
        label.push(0);
        entries.push(Entry {
            path: path.to_vec(),
            label,
        });
        if entries.len() >= 20 {
            break;
        }
    }
    entries
}

fn trim(mut s: &[u8]) -> &[u8] {
    while let Some((&f, rest)) = s.split_first() {
        if f == b' ' || f == b'\t' || f == b'\r' {
            s = rest;
        } else {
            break;
        }
    }
    while let Some((&l, rest)) = s.split_last() {
        if l == b' ' || l == b'\t' || l == b'\r' {
            s = rest;
        } else {
            break;
        }
    }
    s
}

fn first_word(s: &[u8]) -> &[u8] {
    let end = s
        .iter()
        .position(|&b| b == b' ' || b == b'\t')
        .unwrap_or(s.len());
    &s[..end]
}

/// Download an entry: fetch over HTTP, ask for a target with the ASL
/// save requester, write with fs, report with a requester.
fn download(entry: &Entry, online: bool) {
    if !online {
        easy_request(
            amstr!("Aminet Browser"),
            amstr!("Offline sample entry - nothing to download."),
            amstr!("OK"),
        );
        return;
    }

    // "util/misc/Tool.lha" -> request "/util/misc/Tool.lha"
    let mut url_path = Vec::with_capacity(entry.path.len() + 2);
    url_path.push(b'/');
    url_path.extend_from_slice(&entry.path);
    url_path.push(0);

    let resp = match amigaos4::http::get(AMINET_HOST, 80, &url_path) {
        Ok(r) if r.status_code == 200 => r,
        _ => {
            easy_request(
                amstr!("Aminet Browser"),
                amstr!("Download failed."),
                amstr!("OK"),
            );
            return;
        }
    };

    // Suggest the file's own name in the save requester.
    let fname = entry
        .path
        .rsplit(|&b| b == b'/')
        .next()
        .unwrap_or(&entry.path);
    let mut initial = fname.to_vec();
    initial.push(0);

    let pick = FileRequester::new(FileMode::Save)
        .title(amstr!("Save download as..."))
        .initial_drawer(amstr!("RAM:"))
        .initial_file(&initial)
        .show();

    match pick {
        Ok(Some(sel)) => {
            let ok = amigaos4::fs::write_file(&sel.path, &resp.body).is_ok();
            easy_request(
                amstr!("Aminet Browser"),
                if ok {
                    amstr!("Saved.")
                } else {
                    amstr!("Write failed.")
                },
                amstr!("OK"),
            );
        }
        Ok(None) => {} // cancelled
        Err(_) => {
            easy_request(
                amstr!("Aminet Browser"),
                amstr!("Requester failed."),
                amstr!("OK"),
            );
        }
    }
}

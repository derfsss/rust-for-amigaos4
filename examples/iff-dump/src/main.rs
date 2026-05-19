//
// AmigaOS 4 iff-dump - opens an IFF FORM file, walks chunks
// step-by-step with iffparse.library, and prints each chunk's
// four-letter ID, four-letter Type, and byte size.
//
// Demonstrates: the iffparse wrapper end-to-end, the
// InitIFFasDOS pattern for reading a DOS file through iffparse, and
// reading ContextNode fields from Rust.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_sys::wrappers::iffparse::{
    iffparse_alloc_iff, iffparse_init_iffas_dos, iffparse_open_iff,
    iffparse_parse_iff, iffparse_current_chunk, iffparse_close_iff, iffparse_free_iff,
};
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])           { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

// MODE_OLDFILE from dos/dos.h
const MODE_OLDFILE: i32 = 1005;

// iffparse constants (libraries/iffparse.h)
const IFFF_READ:        i32 = 0;
const IFFPARSE_STEP:    i32 = 1;
const IFFERR_EOC:       i32 = -2;
const IFFERR_EOF:       i32 = -1;

// ContextNode field offsets (libraries/iffparse.h)
//   struct MinNode    cn_Node;     (offset 0,  8 bytes: two Node ptrs)
//   LONG              cn_ID;       (offset 8,  4)
//   LONG              cn_Type;     (offset 12, 4)
//   LONG              cn_Size;     (offset 16, 4)
const CN_ID_OFFSET:   usize = 8;
const CN_TYPE_OFFSET: usize = 12;
const CN_SIZE_OFFSET: usize = 16;

// IFFHandle fields:
//   ULONG iff_Stream;  (offset 0, 4)
//   ULONG iff_Flags;   (offset 4, 4)
//   ...

/// Convert a four-byte big-endian IFF tag into a printable C string
/// in the provided 5-byte buffer. Returns a pointer suitable for
/// amiga_debug_str (NUL-terminated).
fn fourcc(out: &mut [u8; 5], id: u32) {
    out[0] = ((id >> 24) & 0xff) as u8;
    out[1] = ((id >> 16) & 0xff) as u8;
    out[2] = ((id >>  8) & 0xff) as u8;
    out[3] = ( id        & 0xff) as u8;
    // Replace any non-printable with '.' so the debug print stays sane.
    for b in &mut out[..4] {
        if *b < 0x20 || *b > 0x7e { *b = b'.'; }
    }
    out[4] = 0;
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - iff-dump\n\0");
    say(b"========================================\n\0");

    let path = b"SYS:Prefs/Presets/Patterns/Bricks/BricksBlue.brush\0";

    // 1. Open the DOS file we'll feed iffparse.
    let fh = unsafe { dos_open(path.as_ptr() as CONST_STRPTR, MODE_OLDFILE) };
    if fh == 0 {
        say(b"FAIL: dos_open returned 0\n\0");
        return 1;
    }

    // 2. Allocate the IFFHandle, point its iff_Stream at our DOS file,
    //    and set it up for AmigaDOS reads.
    let iff = unsafe { iffparse_alloc_iff() };
    if iff.is_null() {
        unsafe { dos_close(fh); }
        say(b"FAIL: AllocIFF returned NULL\n\0");
        return 2;
    }
    unsafe {
        // iff_Stream is the first field of IFFHandle.
        (iff as *mut u32).write(fh);
        iffparse_init_iffas_dos(iff);
    }

    // 3. OpenIFF for reading.
    let rc = unsafe { iffparse_open_iff(iff, IFFF_READ) };
    if rc != 0 {
        unsafe { iffparse_free_iff(iff); dos_close(fh); }
        say_d(b"  OpenIFF rc                       = \0", rc as u32);
        say(b"FAIL: OpenIFF refused\n\0");
        return 3;
    }

    // 4. Walk chunks. ParseIFF(IFFPARSE_STEP) returns 0 on each new
    //    chunk entered, IFFERR_EOC when leaving a group, IFFERR_EOF
    //    at end. CurrentChunk(iff) gives the ContextNode for the
    //    chunk we just walked into.
    let mut chunk_count: u32 = 0;
    let mut top_form_type: u32 = 0;
    loop {
        let rc = unsafe { iffparse_parse_iff(iff, IFFPARSE_STEP) };
        if rc == IFFERR_EOF { break; }
        if rc == IFFERR_EOC { continue; }
        if rc != 0 {
            say_d(b"  ParseIFF error rc                 = \0", rc as u32);
            break;
        }

        let cn = unsafe { iffparse_current_chunk(iff) };
        if cn.is_null() { continue; }

        let id = unsafe { ((cn as *const u8).add(CN_ID_OFFSET)   as *const u32).read_unaligned() };
        let ty = unsafe { ((cn as *const u8).add(CN_TYPE_OFFSET) as *const u32).read_unaligned() };
        let sz = unsafe { ((cn as *const u8).add(CN_SIZE_OFFSET) as *const u32).read_unaligned() };

        if chunk_count == 0 { top_form_type = ty; }

        let mut id_s   = [0u8; 5];
        let mut type_s = [0u8; 5];
        fourcc(&mut id_s,   id);
        fourcc(&mut type_s, ty);

        say(b"  id=\0");
        unsafe { amiga_debug_str(id_s.as_ptr()); }
        say(b"  type=\0");
        unsafe { amiga_debug_str(type_s.as_ptr()); }
        unsafe { amiga_debug_fmt_u32(b"  size=%lu\n\0".as_ptr(), sz); }

        chunk_count += 1;
    }

    // 5. Tidy up.
    unsafe {
        iffparse_close_iff(iff);
        iffparse_free_iff(iff);
        dos_close(fh);
    }

    say_d(b"\n  Chunks walked                     = \0", chunk_count);
    if chunk_count == 0 {
        say(b"FAIL: no chunks walked\n\0");
        return 4;
    }

    // BricksBlue.brush is a FORM ILBM, so the top-level type should
    // be 'ILBM' = 0x494C424D.
    if top_form_type != 0x494C424D {
        say_d(b"FAIL: expected top FORM type ILBM (0x494C424D), got \0", top_form_type);
        return 5;
    }

    say(b"  => PASS: iff-dump walked FORM ILBM chunks via the binding\n\0");
    say(b"========================================\n\0");
    0
}

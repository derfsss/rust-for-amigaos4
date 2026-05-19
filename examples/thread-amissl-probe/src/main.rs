//
// AmigaOS 4 probe — focused diagnostic for two questions:
//   3) Does amigaos4::thread::spawn need its StdioSwap workaround in
//      all launch contexts, or only when ErrorOutput()=BZERO?
//   4) Can we load and initialise AmiSSL through the
//      amigaos4-sys/amisslmaster wrappers?
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_sys::wrappers::amissl::{
    amissl_init_ami_ssla, amissl_tls_method, amissl_ssl_ctx_new, amissl_ssl_ctx_free,
};
use amigaos4_sys::wrappers::amisslmaster::{
    amisslmaster_init_ami_sslmaster, amisslmaster_open_ami_ssl, amisslmaster_close_ami_ssl,
};
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// Tiny logging helpers wired to amiga_debug_*
// ---------------------------------------------------------------------------

fn say(s: &[u8])                  { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_u(prefix: &[u8], v: u32)   { unsafe { amiga_debug_str(prefix.as_ptr()); amiga_debug_fmt_u32(b"%lx\n\0".as_ptr(), v); } }
fn say_d(prefix: &[u8], v: u32)   { unsafe { amiga_debug_str(prefix.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

// ---------------------------------------------------------------------------
// Probe 3: pthread_create stdio sensitivity
// ---------------------------------------------------------------------------

extern "C" {
    fn pthread_create(
        thread: *mut u32,
        attr: *const u8,
        start: unsafe extern "C" fn(APTR) -> APTR,
        arg: APTR,
    ) -> i32;
    fn pthread_join(thread: u32, retval: *mut APTR) -> i32;
}

unsafe extern "C" fn body(_arg: APTR) -> APTR {
    0xABCDu32 as APTR
}

/// Spawn a child thread bypassing our amigaos4::thread::spawn wrapper.
/// Returns (rc, IoErr).
fn bare_pthread_create() -> (i32, i32) {
    let mut tid: u32 = 0;
    let rc = unsafe { pthread_create(&mut tid, core::ptr::null(), body, core::ptr::null_mut()) };
    let ioerr = unsafe { dos_io_err() };
    if rc == 0 {
        let mut ret: APTR = core::ptr::null_mut();
        unsafe { pthread_join(tid, &mut ret); }
    }
    (rc, ioerr)
}

fn probe_threads() {
    say(b"\n=== Probe 3: pthread_create stdio sensitivity ===\n\0");

    // A. Default SerialShell context: ErrorOutput is BZERO, Input/Output
    //    are non-dupable CON: handles. clib4 pthread_create should fail.
    let i0 = unsafe { dos_input() };
    let o0 = unsafe { dos_output() };
    let e0 = unsafe { dos_error_output() };
    say_u(b"  startup Input    = \0", i0);
    say_u(b"  startup Output   = \0", o0);
    say_u(b"  startup ErrorOut = \0", e0);

    let (rc_a, err_a) = bare_pthread_create();
    say_d(b"  [A] bare pthread_create rc       = \0", rc_a as u32);
    say_d(b"      IoErr after                  = \0", err_a as u32);
    let a_pass = rc_a == 0;
    say(if a_pass { b"      => PASS (no swap needed here)\n\0" } else { b"      => FAIL (this is the SerialShell case)\n\0" });

    // B. Apply our StdioSwap manually — replace all three stdio with NIL:,
    //    pthread_create, then restore. This mimics what
    //    amigaos4::thread::spawn does internally.
    //    MODE_NEWFILE = 1006 (dos/dos.h).
    let n_in  = unsafe { dos_open(b"NIL:\0".as_ptr() as CONST_STRPTR, 1006) };
    let n_out = unsafe { dos_open(b"NIL:\0".as_ptr() as CONST_STRPTR, 1006) };
    let n_err = unsafe { dos_open(b"NIL:\0".as_ptr() as CONST_STRPTR, 1006) };
    let s_in  = unsafe { dos_select_input(n_in) };
    let s_out = unsafe { dos_select_output(n_out) };
    let s_err = unsafe { dos_select_error_output(n_err) };

    let (rc_b, err_b) = bare_pthread_create();
    say_d(b"  [B] swap+pthread_create rc       = \0", rc_b as u32);
    say_d(b"      IoErr after                  = \0", err_b as u32);

    // Restore so subsequent amigaos4 calls keep printing somewhere
    // observable (NIL: would have silently dropped output).
    unsafe {
        dos_select_input(s_in);
        dos_select_output(s_out);
        dos_select_error_output(s_err);
        if n_in  != 0 { dos_close(n_in); }
        if n_out != 0 { dos_close(n_out); }
        if n_err != 0 { dos_close(n_err); }
    }
    let b_pass = rc_b == 0;
    say(if b_pass { b"      => PASS (swap workaround OK)\n\0" } else { b"      => FAIL (swap insufficient!)\n\0" });

    // C. Via amigaos4::thread::spawn — full safe wrapper path. Should be
    //    equivalent to B; sanity-checks that our high-level wrapper still
    //    behaves the same on this image.
    let handle = amigaos4::thread::spawn(|| 0xABCDu32);
    let c_pass = match handle {
        Ok(h) => h.join().map(|v| v == 0xABCD).unwrap_or(false),
        Err(_) => false,
    };
    say(if c_pass { b"  [C] amigaos4::thread::spawn      => PASS\n\0" } else { b"  [C] amigaos4::thread::spawn      => FAIL\n\0" });

    // Conclusion summary.
    say(b"  ---\n\0");
    if !a_pass && b_pass && c_pass {
        say(b"  Verdict: SerialShell launch DOES require StdioSwap (current behaviour).\n\0");
    } else if a_pass && b_pass && c_pass {
        say(b"  Verdict: stdio context here lets bare pthread_create succeed --\n\0");
        say(b"           StdioSwap is harmless overhead in this context.\n\0");
    } else {
        say(b"  Verdict: unexpected -- at least one test that should PASS failed.\n\0");
    }
}

// ---------------------------------------------------------------------------
// Probe 4: AmiSSL load + init + SSL_CTX round-trip
// ---------------------------------------------------------------------------

// AMISSL_V350 from libraries/amisslmaster.h (= AmiSSL v5.20). The walkero
// Docker SDK exposes this as AMISSL_CURRENT_VERSION. amisslmaster falls
// back to the newest installed amissl_*.library, so even though this
// QEMU only has v362 and v097g, V350 is the right ask.
const AMISSL_V350: i32 = 39;

fn probe_amissl() {
    say(b"\n=== Probe 4: AmiSSL load + SSL_CTX round-trip ===\n\0");

    // amissl isn't part of -lauto, so we open + bind interfaces by
    // hand here. amisslmaster_glue.c provides the two static globals
    // (AmiSSLMasterBase / IAmiSSLMaster / AmiSSLBase / IAmiSSL) that
    // the wrappers reference; we have to point them at real objects
    // before any wrapper call.
    extern "C" {
        static mut AmiSSLMasterBase: *mut Library;
        static mut IAmiSSLMaster:    *mut amigaos4_sys::interfaces::amisslmaster::AmiSSLMasterIFace;
        static mut AmiSSLBase:       *mut Library;
        static mut IAmiSSL:          *mut amigaos4_sys::interfaces::amissl::AmiSSLIFace;
    }

    // Step 0: open amisslmaster.library at the version our binding's
    // wrappers expect. We use OpenLibrary directly rather than
    // -lauto since amissl is not in libauto's table.
    let base_master = unsafe {
        exec_open_library(b"amisslmaster.library\0".as_ptr() as CONST_STRPTR, 4)
    };
    say_u(b"  OpenLibrary(amisslmaster.library, 4) = \0", base_master as u32);
    if base_master.is_null() {
        say(b"  => FAIL: amisslmaster.library not available\n\0");
        return;
    }
    let iface_master = unsafe {
        exec_get_interface(base_master, b"main\0".as_ptr() as CONST_STRPTR, 1, core::ptr::null())
    };
    say_u(b"  GetInterface(amisslmaster, main, 1)  = \0", iface_master as u32);
    if iface_master.is_null() {
        say(b"  => FAIL: amisslmaster main IFace missing\n\0");
        unsafe { exec_close_library(base_master); }
        return;
    }
    unsafe {
        AmiSSLMasterBase = base_master;
        IAmiSSLMaster = iface_master as *mut _;
    }

    // Step 1: master init. Returns BOOL (1 = success, per
    // amisslmaster.h: BOOL InitAmiSSLMaster(version, usesOpenSSLStructs)).
    let rc = unsafe { amisslmaster_init_ami_sslmaster(AMISSL_V350, 0) };
    say_d(b"  amisslmaster_init_ami_sslmaster rc   = \0", rc as u32);
    if rc == 0 {
        say(b"  => FAIL: master init refused\n\0");
        unsafe {
            exec_drop_interface(iface_master);
            exec_close_library(base_master);
        }
        return;
    }

    // Step 2: open AmiSSL. Returns a Library pointer on success; the
    // global IAmiSSL static is set up as a side effect (via -lauto).
    let base = unsafe { amisslmaster_open_ami_ssl() };
    say_u(b"  amisslmaster_open_ami_ssl base       = \0", base as u32);
    if base.is_null() {
        say(b"  => FAIL: open_ami_ssl returned NULL\n\0");
        unsafe {
            exec_drop_interface(iface_master);
            exec_close_library(base_master);
        }
        return;
    }

    // amisslmaster_open_ami_ssl gives us AmiSSLBase but doesn't bind
    // IAmiSSL — we still need GetInterface for that.
    let iface_ssl = unsafe {
        exec_get_interface(base, b"main\0".as_ptr() as CONST_STRPTR, 1, core::ptr::null())
    };
    say_u(b"  GetInterface(amissl, main, 1)        = \0", iface_ssl as u32);
    if iface_ssl.is_null() {
        say(b"  => FAIL: amissl main IFace missing\n\0");
        unsafe {
            amisslmaster_close_ami_ssl();
            exec_drop_interface(iface_master);
            exec_close_library(base_master);
        }
        return;
    }
    unsafe {
        AmiSSLBase = base;
        IAmiSSL = iface_ssl as *mut _;
    }

    // Step 3a: per-task AmiSSL init. Without this, the per-thread state
    // (errno, locks, RNG, …) isn't set up and SSL_CTX_new dereferences
    // a NULL/garbage TLS context the moment it tries to record an error.
    let init_rc = unsafe { amissl_init_ami_ssla(core::ptr::null_mut()) };
    say_d(b"  InitAmiSSLA(NULL) rc                 = \0", init_rc as u32);
    if init_rc != 0 {
        say(b"  => FAIL: InitAmiSSLA returned non-zero\n\0");
        unsafe {
            exec_drop_interface(iface_ssl);
            amisslmaster_close_ami_ssl();
            exec_drop_interface(iface_master);
            exec_close_library(base_master);
        }
        return;
    }

    let method = unsafe { amissl_tls_method() };
    say_u(b"  TLS_method()                         = \0", method as u32);
    if method.is_null() {
        say(b"  => FAIL: TLS_method returned NULL\n\0");
        unsafe {
            exec_drop_interface(iface_ssl);
            amisslmaster_close_ami_ssl();
            exec_drop_interface(iface_master);
            exec_close_library(base_master);
        }
        return;
    }

    let ctx = unsafe { amissl_ssl_ctx_new(method) };
    say_u(b"  SSL_CTX_new()                        = \0", ctx as u32);
    if ctx.is_null() {
        say(b"  => FAIL: SSL_CTX_new returned NULL\n\0");
        unsafe {
            exec_drop_interface(iface_ssl);
            amisslmaster_close_ami_ssl();
            exec_drop_interface(iface_master);
            exec_close_library(base_master);
        }
        return;
    }

    // Step 4: free + close. Mostly to confirm the destructor path doesn't
    // crash (it dereferences vtables on the way out).
    unsafe {
        amissl_ssl_ctx_free(ctx);
        exec_drop_interface(iface_ssl);
        amisslmaster_close_ami_ssl();
        exec_drop_interface(iface_master);
        exec_close_library(base_master);
    }
    say(b"  SSL_CTX_free + close_ami_ssl: ok\n\0");
    say(b"  => PASS: AmiSSL initialises, TLS_method available,\n\0");
    say(b"           SSL_CTX_new round-trips through the binding.\n\0");
}

// ---------------------------------------------------------------------------
// Entry
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - Diagnostic Probe\n\0");
    say(b"========================================\n\0");

    probe_threads();
    probe_amissl();

    say(b"\n========================================\n\0");
    0
}

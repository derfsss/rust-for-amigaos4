//! C glue declarations for AmigaOS 4 varargs methods.
//!
//! These `extern "C"` functions are implemented in `glue/amiga_glue.c`,
//! compiled by ppc-amigaos-gcc, and linked into the final binary.
//!
//! Only methods with NO non-varargs alternative need C glue.
//! All other varargs methods have *A / *TagList partners callable
//! directly from Rust via vtable dispatch.

use crate::types::*;

extern "C" {
    // ---- IExec->DebugPrintF wrappers ----

    /// Print a string via DebugPrintF (serial/debug output).
    pub fn amiga_debug_str(s: *const u8);

    /// DebugPrintF with one u32 argument.
    pub fn amiga_debug_fmt_u32(fmt: *const u8, a: u32);

    /// DebugPrintF with two u32 arguments.
    pub fn amiga_debug_fmt_u32x2(fmt: *const u8, a: u32, b: u32);

    /// DebugPrintF with three u32 arguments.
    pub fn amiga_debug_fmt_u32x3(fmt: *const u8, a: u32, b: u32, c: u32);

    /// DebugPrintF with four u32 arguments.
    pub fn amiga_debug_fmt_u32x4(fmt: *const u8, a: u32, b: u32, c: u32, d: u32);

    /// DebugPrintF with one string argument.
    pub fn amiga_debug_fmt_str(fmt: *const u8, s: *const u8);

    /// DebugPrintF with one string and one u32 argument.
    pub fn amiga_debug_fmt_str_u32(fmt: *const u8, s: *const u8, a: u32);

    // ---- IIntuition->IDoMethod wrappers ----

    /// BOOPSI method dispatch with 0 extra args.
    pub fn amiga_do_method_0(obj: APTR, mid: u32) -> u32;

    /// BOOPSI method dispatch with 1 extra arg.
    pub fn amiga_do_method_1(obj: APTR, mid: u32, a1: u32) -> u32;

    /// BOOPSI method dispatch with 2 extra args.
    pub fn amiga_do_method_2(obj: APTR, mid: u32, a1: u32, a2: u32) -> u32;

    /// BOOPSI method dispatch with 3 extra args.
    pub fn amiga_do_method_3(obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32) -> u32;

    /// BOOPSI method dispatch with 4 extra args.
    pub fn amiga_do_method_4(obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32;

    /// BOOPSI method dispatch with 5 extra args.
    pub fn amiga_do_method_5(obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32;

    /// BOOPSI method dispatch with 6 extra args.
    pub fn amiga_do_method_6(obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32;

    /// BOOPSI method dispatch with 7 extra args.
    pub fn amiga_do_method_7(obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32;

    /// BOOPSI method dispatch with 8 extra args.
    pub fn amiga_do_method_8(obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32;

    // ---- IIntuition->DoGadgetMethod wrappers ----

    /// Gadget method dispatch with 0 extra args.
    pub fn amiga_do_gadget_method_0(gad: APTR, win: APTR, req: APTR, mid: u32) -> u32;

    /// Gadget method dispatch with 1 extra arg.
    pub fn amiga_do_gadget_method_1(gad: APTR, win: APTR, req: APTR, mid: u32, a1: u32) -> u32;

    /// Gadget method dispatch with 2 extra args.
    pub fn amiga_do_gadget_method_2(gad: APTR, win: APTR, req: APTR, mid: u32, a1: u32, a2: u32) -> u32;

    /// Gadget method dispatch with 3 extra args.
    pub fn amiga_do_gadget_method_3(gad: APTR, win: APTR, req: APTR, mid: u32, a1: u32, a2: u32, a3: u32) -> u32;

    /// Gadget method dispatch with 4 extra args.
    pub fn amiga_do_gadget_method_4(gad: APTR, win: APTR, req: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32;

    /// Gadget method dispatch with 5 extra args.
    pub fn amiga_do_gadget_method_5(gad: APTR, win: APTR, req: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32;

    /// Gadget method dispatch with 6 extra args.
    pub fn amiga_do_gadget_method_6(gad: APTR, win: APTR, req: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32;

    // ---- IIntuition->IDoSuperMethod wrappers ----

    /// Super class method dispatch with 0 extra args.
    pub fn amiga_do_super_method_0(cl: APTR, obj: APTR, mid: u32) -> u32;

    /// Super class method dispatch with 1 extra arg.
    pub fn amiga_do_super_method_1(cl: APTR, obj: APTR, mid: u32, a1: u32) -> u32;

    /// Super class method dispatch with 2 extra args.
    pub fn amiga_do_super_method_2(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32) -> u32;

    /// Super class method dispatch with 3 extra args.
    pub fn amiga_do_super_method_3(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32) -> u32;

    /// Super class method dispatch with 4 extra args.
    pub fn amiga_do_super_method_4(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32;

    /// Super class method dispatch with 5 extra args.
    pub fn amiga_do_super_method_5(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32;

    /// Super class method dispatch with 6 extra args.
    pub fn amiga_do_super_method_6(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32;

    /// Super class method dispatch with 7 extra args.
    pub fn amiga_do_super_method_7(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32;

    /// Super class method dispatch with 8 extra args.
    pub fn amiga_do_super_method_8(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32;

    // ---- IIntuition->ICoerceMethod wrappers ----

    /// Coerced method dispatch with 0 extra args.
    pub fn amiga_coerce_method_0(cl: APTR, obj: APTR, mid: u32) -> u32;

    /// Coerced method dispatch with 1 extra arg.
    pub fn amiga_coerce_method_1(cl: APTR, obj: APTR, mid: u32, a1: u32) -> u32;

    /// Coerced method dispatch with 2 extra args.
    pub fn amiga_coerce_method_2(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32) -> u32;

    /// Coerced method dispatch with 3 extra args.
    pub fn amiga_coerce_method_3(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32) -> u32;

    /// Coerced method dispatch with 4 extra args.
    pub fn amiga_coerce_method_4(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32;

    /// Coerced method dispatch with 5 extra args.
    pub fn amiga_coerce_method_5(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32;

    /// Coerced method dispatch with 6 extra args.
    pub fn amiga_coerce_method_6(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32;

    /// Coerced method dispatch with 7 extra args.
    pub fn amiga_coerce_method_7(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32;

    /// Coerced method dispatch with 8 extra args.
    pub fn amiga_coerce_method_8(cl: APTR, obj: APTR, mid: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32;
}

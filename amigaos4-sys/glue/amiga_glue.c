/*
 * amiga_glue.c — C wrappers for AmigaOS 4 varargs methods.
 *
 * Only methods with NO non-varargs alternative need C glue.
 * All other varargs methods have *A / *TagList partners callable
 * directly from Rust via vtable dispatch.
 *
 * Compiled by ppc-amigaos-gcc inside Docker with -lauto.
 * Linked into the final binary alongside the Rust staticlib.
 *
 * Methods covered:
 *   IExec->DebugPrintF(fmt, ...)        — printf-style debug output
 *   IIntuition->IDoMethod(obj, ...)     — BOOPSI method dispatch
 *   IIntuition->DoGadgetMethod(...)     — gadget method dispatch
 *   IIntuition->IDoSuperMethod(...)     — super method dispatch
 *   IIntuition->ICoerceMethod(...)      — coerced method dispatch
 */

#include <proto/exec.h>
#include <proto/intuition.h>
#include <intuition/classusr.h>

/* ================================================================
 * IExec->DebugPrintF — printf-style debug output
 *
 * Fixed-arity wrappers for common format patterns.
 * ================================================================ */

void amiga_debug_str(const char *s)
{
    IExec->DebugPrintF("%s", s);
}

void amiga_debug_fmt_u32(const char *fmt, uint32 a)
{
    IExec->DebugPrintF(fmt, a);
}

void amiga_debug_fmt_u32x2(const char *fmt, uint32 a, uint32 b)
{
    IExec->DebugPrintF(fmt, a, b);
}

void amiga_debug_fmt_u32x3(const char *fmt, uint32 a, uint32 b, uint32 c)
{
    IExec->DebugPrintF(fmt, a, b, c);
}

void amiga_debug_fmt_u32x4(const char *fmt, uint32 a, uint32 b, uint32 c, uint32 d)
{
    IExec->DebugPrintF(fmt, a, b, c, d);
}

void amiga_debug_fmt_str(const char *fmt, const char *s)
{
    IExec->DebugPrintF(fmt, s);
}

void amiga_debug_fmt_str_u32(const char *fmt, const char *s, uint32 a)
{
    IExec->DebugPrintF(fmt, s, a);
}

/* ================================================================
 * IIntuition->IDoMethod — BOOPSI method dispatch
 *
 * Dispatches a method to a BOOPSI object. The methodID is the
 * first argument after the object pointer. Subsequent arguments
 * are method-specific.
 *
 * Arity 0-8 covers all standard BOOPSI messages (OM_NEW has tags
 * passed as a pointer, not inline varargs).
 * ================================================================ */

uint32 amiga_do_method_0(Object *obj, uint32 mid)
{
    return IIntuition->IDoMethod(obj, mid);
}

uint32 amiga_do_method_1(Object *obj, uint32 mid, uint32 a1)
{
    return IIntuition->IDoMethod(obj, mid, a1);
}

uint32 amiga_do_method_2(Object *obj, uint32 mid, uint32 a1, uint32 a2)
{
    return IIntuition->IDoMethod(obj, mid, a1, a2);
}

uint32 amiga_do_method_3(Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3)
{
    return IIntuition->IDoMethod(obj, mid, a1, a2, a3);
}

uint32 amiga_do_method_4(Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4)
{
    return IIntuition->IDoMethod(obj, mid, a1, a2, a3, a4);
}

uint32 amiga_do_method_5(Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5)
{
    return IIntuition->IDoMethod(obj, mid, a1, a2, a3, a4, a5);
}

uint32 amiga_do_method_6(Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6)
{
    return IIntuition->IDoMethod(obj, mid, a1, a2, a3, a4, a5, a6);
}

uint32 amiga_do_method_7(Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6, uint32 a7)
{
    return IIntuition->IDoMethod(obj, mid, a1, a2, a3, a4, a5, a6, a7);
}

uint32 amiga_do_method_8(Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6, uint32 a7, uint32 a8)
{
    return IIntuition->IDoMethod(obj, mid, a1, a2, a3, a4, a5, a6, a7, a8);
}

/* ================================================================
 * IIntuition->DoGadgetMethod — gadget method dispatch
 *
 * Like IDoMethod but routes through the gadget's window/requester
 * for proper rendering. The window and requester can be NULL.
 * ================================================================ */

uint32 amiga_do_gadget_method_0(struct Gadget *gad, struct Window *win, struct Requester *req, uint32 mid)
{
    return IIntuition->DoGadgetMethod(gad, win, req, mid);
}

uint32 amiga_do_gadget_method_1(struct Gadget *gad, struct Window *win, struct Requester *req, uint32 mid, uint32 a1)
{
    return IIntuition->DoGadgetMethod(gad, win, req, mid, a1);
}

uint32 amiga_do_gadget_method_2(struct Gadget *gad, struct Window *win, struct Requester *req, uint32 mid, uint32 a1, uint32 a2)
{
    return IIntuition->DoGadgetMethod(gad, win, req, mid, a1, a2);
}

uint32 amiga_do_gadget_method_3(struct Gadget *gad, struct Window *win, struct Requester *req, uint32 mid, uint32 a1, uint32 a2, uint32 a3)
{
    return IIntuition->DoGadgetMethod(gad, win, req, mid, a1, a2, a3);
}

uint32 amiga_do_gadget_method_4(struct Gadget *gad, struct Window *win, struct Requester *req, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4)
{
    return IIntuition->DoGadgetMethod(gad, win, req, mid, a1, a2, a3, a4);
}

uint32 amiga_do_gadget_method_5(struct Gadget *gad, struct Window *win, struct Requester *req, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5)
{
    return IIntuition->DoGadgetMethod(gad, win, req, mid, a1, a2, a3, a4, a5);
}

uint32 amiga_do_gadget_method_6(struct Gadget *gad, struct Window *win, struct Requester *req, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6)
{
    return IIntuition->DoGadgetMethod(gad, win, req, mid, a1, a2, a3, a4, a5, a6);
}

/* ================================================================
 * IIntuition->IDoSuperMethod — super class method dispatch
 *
 * Dispatches a method to an object's superclass. Used inside
 * BOOPSI class implementations.
 * ================================================================ */

uint32 amiga_do_super_method_0(struct IClass *cl, Object *obj, uint32 mid)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid);
}

uint32 amiga_do_super_method_1(struct IClass *cl, Object *obj, uint32 mid, uint32 a1)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1);
}

uint32 amiga_do_super_method_2(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1, a2);
}

uint32 amiga_do_super_method_3(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1, a2, a3);
}

uint32 amiga_do_super_method_4(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1, a2, a3, a4);
}

uint32 amiga_do_super_method_5(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1, a2, a3, a4, a5);
}

uint32 amiga_do_super_method_6(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1, a2, a3, a4, a5, a6);
}

uint32 amiga_do_super_method_7(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6, uint32 a7)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1, a2, a3, a4, a5, a6, a7);
}

uint32 amiga_do_super_method_8(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6, uint32 a7, uint32 a8)
{
    return IIntuition->IDoSuperMethod(cl, obj, mid, a1, a2, a3, a4, a5, a6, a7, a8);
}

/* ================================================================
 * IIntuition->ICoerceMethod — coerced method dispatch
 *
 * Like IDoSuperMethod but dispatches to an arbitrary class,
 * not necessarily the object's superclass.
 * ================================================================ */

uint32 amiga_coerce_method_0(struct IClass *cl, Object *obj, uint32 mid)
{
    return IIntuition->ICoerceMethod(cl, obj, mid);
}

uint32 amiga_coerce_method_1(struct IClass *cl, Object *obj, uint32 mid, uint32 a1)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1);
}

uint32 amiga_coerce_method_2(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1, a2);
}

uint32 amiga_coerce_method_3(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1, a2, a3);
}

uint32 amiga_coerce_method_4(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1, a2, a3, a4);
}

uint32 amiga_coerce_method_5(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1, a2, a3, a4, a5);
}

uint32 amiga_coerce_method_6(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1, a2, a3, a4, a5, a6);
}

uint32 amiga_coerce_method_7(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6, uint32 a7)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1, a2, a3, a4, a5, a6, a7);
}

uint32 amiga_coerce_method_8(struct IClass *cl, Object *obj, uint32 mid, uint32 a1, uint32 a2, uint32 a3, uint32 a4, uint32 a5, uint32 a6, uint32 a7, uint32 a8)
{
    return IIntuition->ICoerceMethod(cl, obj, mid, a1, a2, a3, a4, a5, a6, a7, a8);
}

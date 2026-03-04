/*
 * driver_glue.c -- C startup glue for Rust-based AmigaOS handlers/drivers.
 *
 * Provides the _start() entry point, obtains IExec from the system base,
 * then calls the Rust-side rust_handler_main() function.
 *
 * Build with: ppc-amigaos-gcc -nostartfiles -nodefaultlibs -lgcc
 *             -D__AMIGAOS4__ -U__USE_INLINE__
 *             -fno-tree-loop-distribute-patterns
 *
 * Why C and not Rust for _start():
 *   - The global IExec symbol is declared as extern "C" in amigaos4-sys.
 *     It must be DEFINED in C so the linker resolves it.
 *   - IExec must be set before any Rust code runs (the ExecAllocator
 *     dereferences IExec on every allocation).
 */

#define __NOLIBBASE__
#define __NOGLOBALIFACE__
#include <proto/exec.h>
#include <exec/types.h>

/* Global IExec pointer -- Rust code accesses this via:
 *   extern "C" { pub static IExec: *mut ExecIFace; }
 * declared in amigaos4-sys/src/wrappers/exec.rs */
struct ExecIFace *IExec = NULL;

/* Rust entry point -- implemented in the Rust staticlib */
extern int32 rust_handler_main(struct ExecBase *sysbase);

/* Handler entry -- called directly by AmigaOS kernel */
int32 _start(STRPTR argstring __attribute__((unused)),
             int32 arglen __attribute__((unused)),
             struct ExecBase *sysbase)
{
    /* Set up IExec before anything else */
    IExec = (struct ExecIFace *)sysbase->MainInterface;
    IExec->Obtain();

    /* Call Rust code */
    int32 ret = rust_handler_main(sysbase);

    IExec->Release();
    return ret;
}

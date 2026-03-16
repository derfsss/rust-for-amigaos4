/*
 * library_glue.c -- C glue for Rust-based AmigaOS 4 shared libraries.
 *
 * Provides the Resident structure, library manager interface (Open/Close/
 * Expunge), and main interface vector table that dispatches to Rust
 * functions compiled as a staticlib.
 *
 * This follows the AmigaOS 4 RTF_AUTOINIT pattern used by clib4.library
 * and documented in the SDK (exec/resident.h, exec/exectags.h,
 * exec/interfaces.h).
 *
 * Build with: ppc-amigaos-gcc -nostartfiles -nodefaultlibs -lgcc
 *             -D__AMIGAOS4__ -U__USE_INLINE__
 *
 * How it works:
 *   1. AmigaOS scans for the Resident struct (RTC_MATCHWORD marker).
 *   2. RTF_AUTOINIT tells Exec to read libCreateTags for library setup.
 *   3. Exec allocates the library node (MyLibrary), creates interfaces
 *      from the vector tables, and calls libInit().
 *   4. When a process calls OpenLibrary(), Exec calls libOpen().
 *   5. When a process calls CloseLibrary(), Exec calls libClose().
 *   6. The "main" interface exposes Rust functions to callers.
 */

#define __NOLIBBASE__
#define __NOGLOBALIFACE__
#include <proto/exec.h>
#include <exec/types.h>
#include <exec/resident.h>
#include <exec/libraries.h>

/* --------------------------------------------------------------------------
 * Configuration -- customise these for your library
 * -------------------------------------------------------------------------- */

#define LIBNAME    "PROJNAME.library"
#define LIBVER     1
#define LIBREV     0
#define LIBPRI     0
#define VSTRING    "PROJNAME.library 1.0 (16.03.2026)"

/* --------------------------------------------------------------------------
 * Globals
 * -------------------------------------------------------------------------- */

/* IExec must be defined in C so the linker resolves it for Rust code. */
struct ExecIFace *IExec = NULL;

/* --------------------------------------------------------------------------
 * Library node -- the data Exec allocates for us (CLT_DataSize).
 * -------------------------------------------------------------------------- */

struct MyLibrary {
    struct Library libNode;
    uint16         pad;
    BPTR           segList;
};

/* --------------------------------------------------------------------------
 * Forward declarations
 * -------------------------------------------------------------------------- */

/* Rust entry points (implemented in the Rust staticlib) */
extern int32  rust_lib_init(void);
extern int32  rust_lib_open(void);
extern void   rust_lib_close(void);

/* Rust library functions (main interface) */
extern uint32 rust_lib_add(uint32 a, uint32 b);
extern uint32 rust_lib_version(void);

/* --------------------------------------------------------------------------
 * Manager interface -- Obtain / Release / Open / Close / Expunge
 * -------------------------------------------------------------------------- */

static uint32 _manager_Obtain(struct Interface *self)
{
    /* Atomic increment -- safe for SMP. */
    return self->Data.RefCount++;
}

static uint32 _manager_Release(struct Interface *self)
{
    return self->Data.RefCount--;
}

static struct MyLibrary *libInit(struct MyLibrary *libBase,
                                 BPTR segList,
                                 struct ExecIFace *iexec)
{
    /* Store IExec so Rust code (ExecAllocator) can use it. */
    IExec = iexec;

    libBase->libNode.lib_Node.ln_Type = NT_LIBRARY;
    libBase->libNode.lib_Node.ln_Pri  = LIBPRI;
    libBase->libNode.lib_Node.ln_Name = (STRPTR)LIBNAME;
    libBase->libNode.lib_Flags        = LIBF_SUMUSED | LIBF_CHANGED;
    libBase->libNode.lib_Version      = LIBVER;
    libBase->libNode.lib_Revision     = LIBREV;
    libBase->libNode.lib_IdString     = (STRPTR)VSTRING;
    libBase->segList                  = segList;

    /* Call Rust initialisation (allocator is available here). */
    if (rust_lib_init() != 0) {
        return NULL; /* init failed */
    }

    return libBase;
}

static struct MyLibrary *libOpen(struct Interface *self,
                                 uint32 version __attribute__((unused)))
{
    struct MyLibrary *libBase = (struct MyLibrary *)self->Data.LibBase;
    libBase->libNode.lib_OpenCnt++;
    libBase->libNode.lib_Flags &= ~LIBF_DELEXP;

    if (rust_lib_open() != 0) {
        libBase->libNode.lib_OpenCnt--;
        return NULL;
    }

    return libBase;
}

static BPTR libClose(struct Interface *self)
{
    struct MyLibrary *libBase = (struct MyLibrary *)self->Data.LibBase;

    rust_lib_close();

    libBase->libNode.lib_OpenCnt--;

    if (libBase->libNode.lib_OpenCnt == 0 &&
        (libBase->libNode.lib_Flags & LIBF_DELEXP))
    {
        /* Delayed expunge -- library was asked to unload while still open. */
        return libBase->segList;
    }

    return 0;
}

static BPTR libExpunge(struct Interface *self)
{
    struct MyLibrary *libBase = (struct MyLibrary *)self->Data.LibBase;

    if (libBase->libNode.lib_OpenCnt > 0) {
        /* Still in use -- mark for delayed expunge. */
        libBase->libNode.lib_Flags |= LIBF_DELEXP;
        return 0;
    }

    /* Remove from the system library list. */
    IExec->Remove((struct Node *)libBase);

    BPTR seg = libBase->segList;
    IExec->DeleteLibrary(&libBase->libNode);
    return seg;
}

/* --------------------------------------------------------------------------
 * Manager vector table
 * -------------------------------------------------------------------------- */

static void *libManagerVectors[] = {
    (void *)_manager_Obtain,
    (void *)_manager_Release,
    (void *)0,          /* Reserved */
    (void *)0,          /* Reserved */
    (void *)libOpen,
    (void *)libClose,
    (void *)libExpunge,
    (void *)0,          /* Reserved */
    (void *)(APTR)-1,   /* Sentinel */
};

/* --------------------------------------------------------------------------
 * Main interface vector table -- YOUR LIBRARY'S PUBLIC API
 *
 * The first 4 entries (Obtain, Release, 2x Reserved) are mandatory.
 * After that, add your Rust functions in the order you want them
 * indexed in the interface.
 * -------------------------------------------------------------------------- */

static void *mainVectors[] = {
    (void *)_manager_Obtain,
    (void *)_manager_Release,
    (void *)0,              /* Reserved */
    (void *)0,              /* Reserved */
    /* --- Your library functions below --- */
    (void *)rust_lib_add,
    (void *)rust_lib_version,
    /* --- End of functions --- */
    (void *)(APTR)-1,       /* Sentinel */
};

/* --------------------------------------------------------------------------
 * Interface tag lists
 * -------------------------------------------------------------------------- */

static struct TagItem libManagerTags[] = {
    { MIT_Name,        (uint32)"__library" },
    { MIT_VectorTable, (uint32)libManagerVectors },
    { MIT_Version,     1 },
    { MIT_DataSize,    0 },
    { TAG_DONE,        0 }
};

static struct TagItem mainTags[] = {
    { MIT_Name,        (uint32)"main" },
    { MIT_VectorTable, (uint32)mainVectors },
    { MIT_Version,     1 },
    { MIT_DataSize,    0 },
    { TAG_DONE,        0 }
};

/* Interface array -- manager + main + NULL terminator */
static uint32 libInterfaces[] = {
    (uint32)libManagerTags,
    (uint32)mainTags,
    0
};

/* --------------------------------------------------------------------------
 * Library creation tags (CLT_*)
 * -------------------------------------------------------------------------- */

static struct TagItem libCreateTags[] = {
    { CLT_DataSize,   (uint32)sizeof(struct MyLibrary) },
    { CLT_Interfaces, (uint32)libInterfaces },
    { CLT_InitFunc,   (uint32)libInit },
    { TAG_DONE,       0 }
};

/* --------------------------------------------------------------------------
 * Resident structure -- how AmigaOS discovers this library
 * -------------------------------------------------------------------------- */

static const char libName[]   = LIBNAME;
static const char libVString[] = VSTRING;

const struct Resident __attribute__((used)) RomTag = {
    RTC_MATCHWORD,
    (struct Resident *)&RomTag,
    (struct Resident *)&RomTag + 1,
    RTF_NATIVE | RTF_AUTOINIT,
    LIBVER,
    NT_LIBRARY,
    LIBPRI,
    (STRPTR)libName,
    (STRPTR)libVString,
    (APTR)libCreateTags
};

/* --------------------------------------------------------------------------
 * Dummy _start -- prevents running as a normal executable
 * -------------------------------------------------------------------------- */

void _start(void)
{
    /* This program cannot be run from the shell. */
}

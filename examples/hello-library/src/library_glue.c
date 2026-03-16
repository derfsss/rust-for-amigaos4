/*
 * library_glue.c -- C glue for the hello-library example.
 *
 * This is the AmigaOS 4 shared library boilerplate. It provides:
 *   - A Resident structure so AmigaOS can discover the library
 *   - RTF_AUTOINIT tag list for automatic library initialization
 *   - Manager interface (Open/Close/Expunge lifecycle)
 *   - Main interface vector table dispatching to Rust functions
 *
 * The pattern comes from clib4.library source code and the AmigaOS SDK.
 *
 * Build with: ppc-amigaos-gcc -nostartfiles -nodefaultlibs -lgcc
 */

#define __NOLIBBASE__
#define __NOGLOBALIFACE__
#include <proto/exec.h>
#include <exec/types.h>
#include <exec/resident.h>
#include <exec/libraries.h>

/* --------------------------------------------------------------------------
 * Library identity
 * -------------------------------------------------------------------------- */

#define LIBNAME    "hello.library"
#define LIBVER     1
#define LIBREV     0
#define LIBPRI     0
#define VSTRING    "hello.library 1.0 (16.03.2026)"

/* --------------------------------------------------------------------------
 * Globals
 * -------------------------------------------------------------------------- */

struct ExecIFace *IExec = NULL;

/* --------------------------------------------------------------------------
 * Library node structure
 * -------------------------------------------------------------------------- */

struct MyLibrary {
    struct Library libNode;
    uint16         pad;
    BPTR           segList;
};

/* --------------------------------------------------------------------------
 * Rust function declarations
 * -------------------------------------------------------------------------- */

/* Lifecycle */
extern int32  rust_lib_init(void);
extern int32  rust_lib_open(void);
extern void   rust_lib_close(void);

/* Public API */
extern uint32 rust_lib_add(uint32 a, uint32 b);
extern uint32 rust_lib_multiply(uint32 a, uint32 b);
extern uint32 rust_lib_version(void);

/* --------------------------------------------------------------------------
 * Manager interface
 * -------------------------------------------------------------------------- */

static uint32 _manager_Obtain(struct Interface *self)
{
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
    IExec = iexec;

    libBase->libNode.lib_Node.ln_Type = NT_LIBRARY;
    libBase->libNode.lib_Node.ln_Pri  = LIBPRI;
    libBase->libNode.lib_Node.ln_Name = (STRPTR)LIBNAME;
    libBase->libNode.lib_Flags        = LIBF_SUMUSED | LIBF_CHANGED;
    libBase->libNode.lib_Version      = LIBVER;
    libBase->libNode.lib_Revision     = LIBREV;
    libBase->libNode.lib_IdString     = (STRPTR)VSTRING;
    libBase->segList                  = segList;

    if (rust_lib_init() != 0) {
        return NULL;
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
        return libBase->segList;
    }

    return 0;
}

static BPTR libExpunge(struct Interface *self)
{
    struct MyLibrary *libBase = (struct MyLibrary *)self->Data.LibBase;

    if (libBase->libNode.lib_OpenCnt > 0) {
        libBase->libNode.lib_Flags |= LIBF_DELEXP;
        return 0;
    }

    IExec->Remove((struct Node *)libBase);

    BPTR seg = libBase->segList;
    IExec->DeleteLibrary(&libBase->libNode);
    return seg;
}

/* --------------------------------------------------------------------------
 * Vector tables
 * -------------------------------------------------------------------------- */

static void *libManagerVectors[] = {
    (void *)_manager_Obtain,
    (void *)_manager_Release,
    (void *)0,
    (void *)0,
    (void *)libOpen,
    (void *)libClose,
    (void *)libExpunge,
    (void *)0,
    (void *)(APTR)-1,
};

static void *mainVectors[] = {
    (void *)_manager_Obtain,
    (void *)_manager_Release,
    (void *)0,
    (void *)0,
    /* Library functions */
    (void *)rust_lib_add,       /* index 4 */
    (void *)rust_lib_multiply,  /* index 5 */
    (void *)rust_lib_version,   /* index 6 */
    (void *)(APTR)-1,
};

/* --------------------------------------------------------------------------
 * Interface + library creation tags
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

static uint32 libInterfaces[] = {
    (uint32)libManagerTags,
    (uint32)mainTags,
    0
};

static struct TagItem libCreateTags[] = {
    { CLT_DataSize,   (uint32)sizeof(struct MyLibrary) },
    { CLT_Interfaces, (uint32)libInterfaces },
    { CLT_InitFunc,   (uint32)libInit },
    { TAG_DONE,       0 }
};

/* --------------------------------------------------------------------------
 * Resident structure
 * -------------------------------------------------------------------------- */

static const char libName[]    = LIBNAME;
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

void _start(void)
{
    /* This is a library, not an executable. */
}

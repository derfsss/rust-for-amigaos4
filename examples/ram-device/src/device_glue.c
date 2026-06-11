/*
 * device_glue.c -- C glue for a Rust-based AmigaOS 4 exec device.
 *
 * Provides the Resident structure, the "__device" manager interface
 * (Open/Close/Expunge/BeginIO/AbortIO per struct DeviceManagerInterface
 * in exec/interfaces.h), and dispatches the actual I/O handling to Rust
 * functions compiled as a staticlib.
 *
 * The device implements CMD_READ/CMD_WRITE over a RAM buffer -- a
 * minimal but complete demonstration of the exec device protocol
 * (quick I/O, ReplyMsg, io_Error, io_Actual) with the logic in Rust.
 *
 * Build with: ppc-amigaos-gcc -nostartfiles -nodefaultlibs -lgcc
 *             -D__AMIGAOS4__ -U__USE_INLINE__
 */

#define __NOLIBBASE__
#define __NOGLOBALIFACE__
#include <proto/exec.h>
#include <exec/types.h>
#include <exec/resident.h>
#include <exec/devices.h>
#include <exec/io.h>
#include <exec/errors.h>
#include <dos/dos.h>   /* BPTR */

/* --------------------------------------------------------------------------
 * Configuration
 * -------------------------------------------------------------------------- */

#define DEVNAME    "ram.device"
#define DEVVER     1
#define DEVREV     0
#define DEVPRI     0
#define VSTRING    "ram.device 1.0 (11.06.2026)"

/* --------------------------------------------------------------------------
 * Globals
 * -------------------------------------------------------------------------- */

/* IExec must be defined in C so the linker resolves it for Rust code. */
struct ExecIFace *IExec = NULL;

/* --------------------------------------------------------------------------
 * Device node
 * -------------------------------------------------------------------------- */

struct MyDevice {
    struct Device devNode;
    uint16        pad;
    BPTR          segList;
};

/* --------------------------------------------------------------------------
 * Rust entry points (implemented in the Rust staticlib)
 * -------------------------------------------------------------------------- */

extern int32 rust_dev_init(void);
extern int8  rust_dev_open(struct IORequest *ior, uint32 unit, uint32 flags);
extern void  rust_dev_close(struct IORequest *ior);
extern void  rust_dev_begin_io(struct IORequest *ior);
extern void  rust_dev_abort_io(struct IORequest *ior);

/* --------------------------------------------------------------------------
 * Manager interface ("__device")
 * -------------------------------------------------------------------------- */

static uint32 _manager_Obtain(struct Interface *self)
{
    return self->Data.RefCount++;
}

static uint32 _manager_Release(struct Interface *self)
{
    return self->Data.RefCount--;
}

static struct MyDevice *devInit(struct MyDevice *devBase,
                                BPTR segList,
                                struct ExecIFace *iexec)
{
    IExec = iexec;

    devBase->devNode.dd_Library.lib_Node.ln_Type = NT_DEVICE;
    devBase->devNode.dd_Library.lib_Node.ln_Pri  = DEVPRI;
    devBase->devNode.dd_Library.lib_Node.ln_Name = (STRPTR)DEVNAME;
    devBase->devNode.dd_Library.lib_Flags        = LIBF_SUMUSED | LIBF_CHANGED;
    devBase->devNode.dd_Library.lib_Version      = DEVVER;
    devBase->devNode.dd_Library.lib_Revision     = DEVREV;
    devBase->devNode.dd_Library.lib_IdString     = (STRPTR)VSTRING;
    devBase->segList                             = segList;

    if (rust_dev_init() != 0) {
        return NULL;
    }

    return devBase;
}

/* Per exec/interfaces.h, devOpen's status goes into ior->io_Error. */
static void devOpen(struct Interface *self,
                    struct IORequest *ior, uint32 unit, uint32 flags)
{
    struct MyDevice *devBase = (struct MyDevice *)self->Data.LibBase;

    int8 err = rust_dev_open(ior, unit, flags);
    ior->io_Error = err;

    if (err == 0) {
        devBase->devNode.dd_Library.lib_OpenCnt++;
        devBase->devNode.dd_Library.lib_Flags &= ~LIBF_DELEXP;
        ior->io_Device = (struct Device *)devBase;
        ior->io_Message.mn_Node.ln_Type = NT_REPLYMSG;
    } else {
        ior->io_Device = NULL;
        ior->io_Unit   = NULL;
    }
}

static APTR devClose(struct Interface *self, struct IORequest *ior)
{
    struct MyDevice *devBase = (struct MyDevice *)self->Data.LibBase;

    rust_dev_close(ior);

    ior->io_Device = NULL;
    ior->io_Unit   = NULL;

    devBase->devNode.dd_Library.lib_OpenCnt--;

    if (devBase->devNode.dd_Library.lib_OpenCnt == 0 &&
        (devBase->devNode.dd_Library.lib_Flags & LIBF_DELEXP))
    {
        return (APTR)devBase->segList;
    }

    return NULL;
}

static APTR devExpunge(struct Interface *self)
{
    struct MyDevice *devBase = (struct MyDevice *)self->Data.LibBase;

    if (devBase->devNode.dd_Library.lib_OpenCnt > 0) {
        devBase->devNode.dd_Library.lib_Flags |= LIBF_DELEXP;
        return NULL;
    }

    IExec->Remove((struct Node *)devBase);

    BPTR seg = devBase->segList;
    IExec->DeleteLibrary(&devBase->devNode.dd_Library);
    return (APTR)seg;
}

static void devBeginIO(struct Interface *self __attribute__((unused)),
                       struct IORequest *ior)
{
    rust_dev_begin_io(ior);
}

static void devAbortIO(struct Interface *self __attribute__((unused)),
                       struct IORequest *ior)
{
    rust_dev_abort_io(ior);
}

/* --------------------------------------------------------------------------
 * Manager vector table -- order per struct DeviceManagerInterface
 * (exec/interfaces.h): Obtain, Release, [2 reserved], Open, Close,
 * Expunge, [reserved GetInterface], BeginIO, AbortIO.
 * -------------------------------------------------------------------------- */

static void *devManagerVectors[] = {
    (void *)_manager_Obtain,
    (void *)_manager_Release,
    (void *)0,          /* Reserved (Expunge) */
    (void *)0,          /* Reserved (Clone) */
    (void *)devOpen,
    (void *)devClose,
    (void *)devExpunge,
    (void *)0,          /* Reserved (GetInterface) */
    (void *)devBeginIO,
    (void *)devAbortIO,
    (void *)(APTR)-1,   /* Sentinel */
};

static struct TagItem devManagerTags[] = {
    { MIT_Name,        (uint32)"__device" },
    { MIT_VectorTable, (uint32)devManagerVectors },
    { MIT_Version,     1 },
    { MIT_DataSize,    0 },
    { TAG_DONE,        0 }
};

static uint32 devInterfaces[] = {
    (uint32)devManagerTags,
    0
};

static struct TagItem devCreateTags[] = {
    { CLT_DataSize,   (uint32)sizeof(struct MyDevice) },
    { CLT_Interfaces, (uint32)devInterfaces },
    { CLT_InitFunc,   (uint32)devInit },
    { TAG_DONE,       0 }
};

/* --------------------------------------------------------------------------
 * Resident structure
 * -------------------------------------------------------------------------- */

static const char devName[]    = DEVNAME;
static const char devVString[] = VSTRING;

const struct Resident __attribute__((used)) RomTag = {
    RTC_MATCHWORD,
    (struct Resident *)&RomTag,
    (struct Resident *)&RomTag + 1,
    RTF_NATIVE | RTF_AUTOINIT,
    DEVVER,
    NT_DEVICE,
    DEVPRI,
    (STRPTR)devName,
    (STRPTR)devVString,
    (APTR)devCreateTags
};

/* --------------------------------------------------------------------------
 * Dummy _start -- prevents running as a normal executable
 * -------------------------------------------------------------------------- */

void _start(void)
{
    /* This program cannot be run from the shell. */
}

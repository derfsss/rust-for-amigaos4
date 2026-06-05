//! RAII window management via `OpenWindowTagList`/`CloseWindow`.
//!
//! [`AmigaWindow`] wraps Intuition window creation and disposal with RAII.
//! IDCMP messages are copied into an owned [`IntuiMsg`] value before the
//! underlying `IntuiMessage` is replied, so no dangling pointers can occur.
//!
//! # Example
//! ```ignore
//! use amigaos4::tag::TagListBuilder;
//! use amigaos4::window::*;
//!
//! let tags = TagListBuilder::new()
//!     .tag(WA_LEFT, 50)
//!     .tag(WA_TOP, 50)
//!     .tag(WA_WIDTH, 400)
//!     .tag(WA_HEIGHT, 300)
//!     .tag(WA_TITLE, b"Hello\0".as_ptr() as u32)
//!     .tag(WA_IDCMP, IDCMP_CLOSEWINDOW)
//!     .tag(WA_CLOSE_GADGET, 1)
//!     .tag(WA_DRAG_BAR, 1)
//!     .tag(WA_DEPTH_GADGET, 1)
//!     .tag(WA_ACTIVATE, 1)
//!     .build();
//!
//! let win = AmigaWindow::open(&tags)?;
//! loop {
//!     let msg = win.wait_msg();
//!     if msg.class == IDCMP_CLOSEWINDOW { break; }
//! }
//! ```

use crate::error::{AmigaError, Result};
use amigaos4_sys::{Message, MsgPort, RastPort, TagItem, Window, APTR, TAG_USER};

// ---------------------------------------------------------------------------
// IDCMP flag constants (intuition/intuition.h)
// ---------------------------------------------------------------------------

/// Window was resized.
pub const IDCMP_NEWSIZE: u32 = 0x0000_0002;
/// Window needs refresh (smart/simple refresh).
pub const IDCMP_REFRESHWINDOW: u32 = 0x0000_0004;
/// Mouse button pressed or released.
pub const IDCMP_MOUSEBUTTONS: u32 = 0x0000_0008;
/// Mouse moved within the window.
pub const IDCMP_MOUSEMOVE: u32 = 0x0000_0010;
/// Gadget pressed down.
pub const IDCMP_GADGETDOWN: u32 = 0x0000_0020;
/// Gadget released (verify action).
pub const IDCMP_GADGETUP: u32 = 0x0000_0040;
/// Menu item selected.
pub const IDCMP_MENUPICK: u32 = 0x0000_0100;
/// Close gadget clicked.
pub const IDCMP_CLOSEWINDOW: u32 = 0x0000_0200;
/// Raw keyboard event (uncooked key code).
pub const IDCMP_RAWKEY: u32 = 0x0000_0400;
/// Cooked keyboard event (ASCII character).
pub const IDCMP_VANILLAKEY: u32 = 0x0020_0000;
/// Periodic timer tick (~10 Hz).
pub const IDCMP_INTUITICKS: u32 = 0x0040_0000;

// ---------------------------------------------------------------------------
// Window attribute tags (intuition/intuition.h)
// WA_Dummy = TAG_USER + 99 = 0x8000_0063 (per intuition.h:1387). An
// earlier version of this binding used TAG_USER + 0x10000, which made
// every WA_* tag a different ID than the SDK actually defines — so
// OpenWindowTagList silently ignored size/position/IDCMP and opened a
// VISITOR window at screen size with no UserPort, and any subsequent
// GetMsg call dereferenced a NULL UserPort.
// ---------------------------------------------------------------------------

const WA_DUMMY: u32 = TAG_USER + 99;

/// Window left edge position.
pub const WA_LEFT: u32 = WA_DUMMY + 0x01;
/// Window top edge position.
pub const WA_TOP: u32 = WA_DUMMY + 0x02;
/// Window width in pixels.
pub const WA_WIDTH: u32 = WA_DUMMY + 0x03;
/// Window height in pixels.
pub const WA_HEIGHT: u32 = WA_DUMMY + 0x04;
/// Detail (foreground) pen number.
pub const WA_DETAIL_PEN: u32 = WA_DUMMY + 0x05;
/// Block (background) pen number.
pub const WA_BLOCK_PEN: u32 = WA_DUMMY + 0x06;
/// IDCMP flags for event delivery.
pub const WA_IDCMP: u32 = WA_DUMMY + 0x07;
/// Window flag bits.
pub const WA_FLAGS: u32 = WA_DUMMY + 0x08;
/// Pointer to first gadget in chain.
pub const WA_GADGETS: u32 = WA_DUMMY + 0x09;
/// Checkmark image pointer for menu items.
pub const WA_CHECKMARK: u32 = WA_DUMMY + 0x0A;
/// Window title string (null-terminated).
pub const WA_TITLE: u32 = WA_DUMMY + 0x0B;
/// Screen title displayed when window is active.
pub const WA_SCREEN_TITLE: u32 = WA_DUMMY + 0x0C;
/// Open on a custom screen (pointer).
pub const WA_CUSTOM_SCREEN: u32 = WA_DUMMY + 0x0D;
/// Super bitmap pointer for WFLG_SUPER_BITMAP windows.
pub const WA_SUPER_BITMAP: u32 = WA_DUMMY + 0x0E;
/// Minimum window width.
pub const WA_MIN_WIDTH: u32 = WA_DUMMY + 0x0F;
/// Minimum window height.
pub const WA_MIN_HEIGHT: u32 = WA_DUMMY + 0x10;
/// Maximum window width.
pub const WA_MAX_WIDTH: u32 = WA_DUMMY + 0x11;
/// Maximum window height.
pub const WA_MAX_HEIGHT: u32 = WA_DUMMY + 0x12;
/// Inner (client area) width in pixels.
pub const WA_INNER_WIDTH: u32 = WA_DUMMY + 0x13;
/// Inner (client area) height in pixels.
pub const WA_INNER_HEIGHT: u32 = WA_DUMMY + 0x14;
/// Public screen name string (null-terminated).
pub const WA_PUB_SCREEN_NAME: u32 = WA_DUMMY + 0x15;
/// Open on a public screen (pointer).
pub const WA_PUB_SCREEN: u32 = WA_DUMMY + 0x16;
/// Fall back to default screen if named screen unavailable.
pub const WA_PUB_SCREEN_FALLBACK: u32 = WA_DUMMY + 0x17;
/// Enable the sizing gadget. Alias for [`WA_SIZE_GADGET`].
pub const WA_SIZING: u32 = WA_DUMMY + 0x1E;
/// Enable the sizing gadget.
pub const WA_SIZE_GADGET: u32 = WA_DUMMY + 0x1E;
/// Enable the drag bar (title bar).
pub const WA_DRAG_BAR: u32 = WA_DUMMY + 0x1F;
/// Enable the depth (front/back) gadget.
pub const WA_DEPTH_GADGET: u32 = WA_DUMMY + 0x20;
/// Enable the close gadget.
pub const WA_CLOSE_GADGET: u32 = WA_DUMMY + 0x21;
/// Open as a backdrop window (behind all others).
pub const WA_BACKDROP: u32 = WA_DUMMY + 0x22;
/// Open without window borders.
pub const WA_BORDERLESS: u32 = WA_DUMMY + 0x25;
/// Activate the window immediately on open.
pub const WA_ACTIVATE: u32 = WA_DUMMY + 0x26;
/// Report pointer movement over the window (IDCMP_MOUSEMOVE without needing a
/// button held). Boolean tag: set to 1 to enable. Value `WA_Dummy + 0x23` per
/// the OS4 SDK `intuition/intuition.h`.
pub const WA_REPORT_MOUSE: u32 = WA_DUMMY + 0x23;
/// Trap right mouse button events (suppress menu).
pub const WA_RMBTRAP: u32 = WA_DUMMY + 0x27;
/// Use simple refresh mode.
pub const WA_SIMPLE_REFRESH: u32 = WA_DUMMY + 0x29;
/// Use smart refresh mode.
pub const WA_SMART_REFRESH: u32 = WA_DUMMY + 0x2A;
/// Request a GimmeZeroZero inner region.
pub const WA_GIMME_ZERO_ZERO: u32 = WA_DUMMY + 0x2E;

// ---------------------------------------------------------------------------
// Raw IntuiMessage partial layout for reading IDCMP fields.
// ---------------------------------------------------------------------------

/// Partial layout of `struct IntuiMessage` used to extract IDCMP fields.
///
/// On AmigaOS 4 (PPC, 32-bit), the layout is:
/// ```text
/// struct IntuiMessage {
///     struct Message ExecMessage;  // 20 bytes
///     uint32 Class;                // offset 20
///     uint16 Code;                 // offset 24
///     uint16 Qualifier;            // offset 26
///     APTR   IAddress;             // offset 28
///     int16  MouseX;               // offset 32
///     int16  MouseY;               // offset 34
///     ...
/// };
/// ```
#[repr(C)]
struct RawIntuiMessage {
    _exec_message: [u8; 20],
    class: u32,
    code: u16,
    qualifier: u16,
    _iaddress: APTR,
    mouse_x: i16,
    mouse_y: i16,
}

/// Owned copy of IDCMP message fields. The underlying `IntuiMessage` has
/// already been replied by the time this value is returned.
#[derive(Debug, Clone, Copy)]
pub struct IntuiMsg {
    /// IDCMP class (e.g. `IDCMP_CLOSEWINDOW`).
    pub class: u32,
    /// Event code (key code, menu number, etc.).
    pub code: u16,
    /// Input qualifier flags (shift, alt, etc.).
    pub qualifier: u16,
    /// Mouse X position relative to the window.
    pub mouse_x: i16,
    /// Mouse Y position relative to the window.
    pub mouse_y: i16,
}

// ---------------------------------------------------------------------------
// Window UserPort offset
// ---------------------------------------------------------------------------

/// Byte offset of `Window.UserPort` in the AmigaOS 4 Window struct.
///
/// This is a well-known, stable offset derived from the OS4 SDK:
/// ```text
/// Window {
///   NextWindow: *mut Window,       //  0  (4)
///   LeftEdge, TopEdge: i16,        //  4  (4)
///   Width, Height: i16,            //  8  (4)
///   MouseY, MouseX: i16,           // 12  (4)
///   MinWidth, MinHeight: i16,      // 16  (4)
///   MaxWidth, MaxHeight: u16,      // 20  (4)
///   Flags: u32,                    // 24  (4)
///   MenuStrip: *mut Menu,          // 28  (4)
///   Title: *const i8,              // 32  (4)
///   FirstRequest: *mut Requester,  // 36  (4)
///   DMRequest: *mut Requester,     // 40  (4)
///   ReqCount: i16,                 // 44  (2)
///   WScreen: *mut Screen,          // 46  (4)
///   RPort: *mut RastPort,          // 50  (4)
///   BorderLeft, BorderTop: i8,     // 54  (2)
///   BorderRight, BorderBottom: i8, // 56  (2)
///   BorderRPort: *mut RastPort,    // 58  (4)
///   FirstGadget: *mut Gadget,      // 62  (4)
///   Parent, Descendant: *mut Window, // 66, 70  (8)
///   Pointer: *mut u16,             // 74  (4)
///   PtrHeight, PtrWidth: i8,       // 78  (2)
///   XOffset, YOffset: i8,          // 80  (2)
///   IDCMPFlags: u32,               // 82  (4)
///   UserPort: *mut MsgPort,        // 86  (4)
/// }
/// ```
const WINDOW_USER_PORT_OFFSET: usize = 86;

/// Byte offset of `Window.RPort` in the AmigaOS 4 Window struct.
const WINDOW_RPORT_OFFSET: usize = 50;

/// Extract the UserPort from a Window pointer.
///
/// # Safety
/// `win` must point to a valid, open AmigaOS Window.
#[inline]
unsafe fn window_user_port(win: *mut Window) -> *mut MsgPort {
    *((win as *const u8).add(WINDOW_USER_PORT_OFFSET) as *const *mut MsgPort)
}

/// Extract the RastPort from a Window pointer.
///
/// # Safety
/// `win` must point to a valid, open AmigaOS Window.
#[inline]
unsafe fn window_rast_port(win: *mut Window) -> *mut RastPort {
    *((win as *const u8).add(WINDOW_RPORT_OFFSET) as *const *mut RastPort)
}

// ---------------------------------------------------------------------------
// AmigaWindow
// ---------------------------------------------------------------------------

/// RAII wrapper around `OpenWindowTagList` / `CloseWindow`.
///
/// Automatically closes the window on drop.
pub struct AmigaWindow {
    ptr: *mut Window,
}

impl AmigaWindow {
    /// Open a window using a TAG_DONE-terminated tag list.
    ///
    /// Pass `core::ptr::null()` for `NewWindow` (the first parameter) and
    /// configure entirely via tags. Use [`TagListBuilder`](crate::tag::TagListBuilder)
    /// to construct the tag list.
    ///
    /// Returns `Err(AllocationFailed)` if the OS could not open the window.
    pub fn open(tags: &[TagItem]) -> Result<Self> {
        let ptr = unsafe {
            amigaos4_sys::intuition_open_window_tag_list(core::ptr::null(), tags.as_ptr())
        };
        if ptr.is_null() {
            Err(AmigaError::AllocationFailed)
        } else {
            Ok(Self { ptr })
        }
    }

    /// Block until an IDCMP message arrives, then copy its fields and reply.
    ///
    /// This calls `WaitPort` followed by `GetMsg`. The message is replied
    /// immediately after copying, so the returned [`IntuiMsg`] is fully owned.
    pub fn wait_msg(&self) -> IntuiMsg {
        unsafe {
            let port = window_user_port(self.ptr);
            loop {
                amigaos4_sys::exec_wait_port(port);
                let msg = amigaos4_sys::exec_get_msg(port);
                if !msg.is_null() {
                    let imsg = copy_intui_message(msg);
                    amigaos4_sys::exec_reply_msg(msg);
                    return imsg;
                }
                // Spurious wake — wait again
            }
        }
    }

    /// Non-blocking check for a pending IDCMP message.
    ///
    /// Returns `Some(msg)` if a message was waiting, `None` otherwise.
    /// The message is replied immediately after copying.
    pub fn get_msg(&self) -> Option<IntuiMsg> {
        unsafe {
            let port = window_user_port(self.ptr);
            let msg = amigaos4_sys::exec_get_msg(port);
            if msg.is_null() {
                None
            } else {
                let imsg = copy_intui_message(msg);
                amigaos4_sys::exec_reply_msg(msg);
                Some(imsg)
            }
        }
    }

    /// Get the window's RastPort for drawing operations.
    ///
    /// The returned pointer is valid for as long as this window is open.
    /// Use it with [`DrawContext::from_window`](crate::gfx::DrawContext) for
    /// safe drawing.
    #[inline]
    pub fn rast_port(&self) -> *mut RastPort {
        unsafe { window_rast_port(self.ptr) }
    }

    /// Get the raw `Window` pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut Window {
        self.ptr
    }

    /// Consume the wrapper and return the raw pointer without closing
    /// the window. The caller assumes responsibility for calling
    /// `intuition_close_window`.
    #[inline]
    pub fn into_raw(self) -> *mut Window {
        let ptr = self.ptr;
        core::mem::forget(self);
        ptr
    }
}

impl Drop for AmigaWindow {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { amigaos4_sys::intuition_close_window(self.ptr) }
        }
    }
}

/// Copy IDCMP fields from a raw IntuiMessage pointer into an owned value.
///
/// # Safety
/// `msg` must be a non-null pointer to a valid `IntuiMessage`.
unsafe fn copy_intui_message(msg: *mut Message) -> IntuiMsg {
    let raw = msg as *const RawIntuiMessage;
    IntuiMsg {
        class: (*raw).class,
        code: (*raw).code,
        qualifier: (*raw).qualifier,
        mouse_x: (*raw).mouse_x,
        mouse_y: (*raw).mouse_y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;
    extern crate alloc;
    use alloc::format;

    #[test]
    fn raw_intui_message_size() {
        // On PPC (32-bit target): 36 bytes (no alignment padding).
        // On x86_64 (64-bit host): 48 bytes (APTR is 8 bytes + alignment padding).
        if cfg!(target_pointer_width = "32") {
            assert_eq!(mem::size_of::<RawIntuiMessage>(), 36);
        } else {
            assert_eq!(mem::size_of::<RawIntuiMessage>(), 48);
        }
    }

    #[test]
    fn window_user_port_offset_value() {
        assert_eq!(WINDOW_USER_PORT_OFFSET, 86);
    }

    #[test]
    fn window_rport_offset_value() {
        assert_eq!(WINDOW_RPORT_OFFSET, 50);
    }

    #[test]
    fn intui_msg_is_copy_clone_debug() {
        let msg = IntuiMsg {
            class: 0,
            code: 0,
            qualifier: 0,
            mouse_x: 0,
            mouse_y: 0,
        };
        let _copy = msg;
        let _clone = msg.clone();
        let _debug = format!("{:?}", msg);
    }

    #[test]
    fn intui_msg_fields_roundtrip() {
        let msg = IntuiMsg {
            class: 0x200,
            code: 42,
            qualifier: 0x8000,
            mouse_x: -10,
            mouse_y: 100,
        };
        assert_eq!(msg.class, 0x200);
        assert_eq!(msg.code, 42);
        assert_eq!(msg.qualifier, 0x8000);
        assert_eq!(msg.mouse_x, -10);
        assert_eq!(msg.mouse_y, 100);
    }

    #[test]
    fn idcmp_closewindow_bit() {
        assert_eq!(IDCMP_CLOSEWINDOW, 0x200);
    }

    #[test]
    fn idcmp_gadgetup_bit() {
        assert_eq!(IDCMP_GADGETUP, 0x40);
    }

    #[test]
    fn idcmp_vanillakey_bit() {
        assert_eq!(IDCMP_VANILLAKEY, 0x200000);
    }

    #[test]
    fn wa_dummy_base() {
        assert_eq!(WA_DUMMY, TAG_USER + 99);
        assert_eq!(WA_DUMMY, 0x8000_0063);
    }

    #[test]
    fn wa_left_offset() {
        assert_eq!(WA_LEFT, WA_DUMMY + 1);
    }

    #[test]
    fn wa_sizing_equals_size_gadget() {
        assert_eq!(WA_SIZING, WA_SIZE_GADGET);
    }
}

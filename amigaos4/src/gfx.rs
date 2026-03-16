//! Graphics drawing operations on a RastPort.
//!
//! [`DrawContext`] borrows a `RastPort` and provides safe wrappers around
//! the AmigaOS Graphics library drawing primitives.
//!
//! # Example
//! ```ignore
//! use amigaos4::gfx::DrawContext;
//!
//! let win = AmigaWindow::open(&tags)?;
//! let mut dc = unsafe { DrawContext::from_ptr(win.rast_port()) };
//! dc.set_pen(1);
//! dc.move_to(10, 10);
//! dc.draw_to(100, 100);
//! dc.fill_rect(20, 20, 80, 80);
//! dc.draw_text(b"Hello, Amiga!");
//! ```

use amigaos4_sys::{RastPort, CONST_STRPTR};
use core::marker::PhantomData;

/// A borrowed RastPort handle for drawing operations.
///
/// The lifetime parameter ties this context to the owner of the RastPort
/// (typically an [`AmigaWindow`](crate::window::AmigaWindow) or a
/// [`PubScreen`](crate::screen::PubScreen)), preventing use-after-free.
pub struct DrawContext<'a> {
    rp: *mut RastPort,
    _lifetime: PhantomData<&'a mut ()>,
}

impl<'a> DrawContext<'a> {
    /// Wrap a raw RastPort pointer in a `DrawContext`.
    ///
    /// # Safety
    /// The caller must ensure that `rp` points to a valid `RastPort` and
    /// remains valid for the lifetime `'a`.
    #[inline]
    pub unsafe fn from_ptr(rp: *mut RastPort) -> Self {
        Self {
            rp,
            _lifetime: PhantomData,
        }
    }

    /// Set the foreground drawing pen (A-pen).
    #[inline]
    pub fn set_pen(&mut self, pen: u8) {
        unsafe { amigaos4_sys::graphics_set_apen(self.rp, pen) }
    }

    /// Set the background drawing pen (B-pen).
    #[inline]
    pub fn set_bg_pen(&mut self, pen: u8) {
        unsafe { amigaos4_sys::graphics_set_bpen(self.rp, pen) }
    }

    /// Move the drawing position without drawing.
    #[inline]
    pub fn move_to(&mut self, x: i16, y: i16) {
        unsafe { amigaos4_sys::graphics_move(self.rp, x, y) }
    }

    /// Draw a line from the current position to (`x`, `y`).
    #[inline]
    pub fn draw_to(&mut self, x: i16, y: i16) {
        unsafe { amigaos4_sys::graphics_draw(self.rp, x, y) }
    }

    /// Fill a rectangle with the current A-pen colour.
    ///
    /// Coordinates are inclusive: the rectangle covers pixels from
    /// (`x1`, `y1`) to (`x2`, `y2`).
    #[inline]
    pub fn fill_rect(&mut self, x1: i16, y1: i16, x2: i16, y2: i16) {
        unsafe { amigaos4_sys::graphics_rect_fill(self.rp, x1, y1, x2, y2) }
    }

    /// Draw text at the current drawing position.
    ///
    /// `text` is a raw byte slice (no null terminator required — the length
    /// is passed explicitly). The text is rendered using the RastPort's
    /// current font.
    #[inline]
    pub fn draw_text(&mut self, text: &[u8]) {
        let len = text.len().min(u16::MAX as usize) as u16;
        unsafe { amigaos4_sys::graphics_text(self.rp, text.as_ptr() as CONST_STRPTR, len) }
    }

    /// Measure the pixel width of `text` in the RastPort's current font.
    #[inline]
    pub fn text_width(&self, text: &[u8]) -> i16 {
        let len = text.len().min(u16::MAX as usize) as u16;
        unsafe { amigaos4_sys::graphics_text_length(self.rp, text.as_ptr() as CONST_STRPTR, len) }
    }

    /// Get the raw RastPort pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut RastPort {
        self.rp
    }
}

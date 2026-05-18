//! Thread spawning via clib4 pthreads (application mode only).
//!
//! [`spawn`] creates a new thread from a closure and returns a
//! [`JoinHandle`] to retrieve the result. The closure and result
//! are transferred via heap-allocated boxes.
//!
//! Requires the `thread` feature (enabled by default via `app`).

use alloc::boxed::Box;
use core::marker::PhantomData;
use crate::error::{AmigaError, Result};
use amigaos4_sys::APTR;

extern "C" {
    fn pthread_create(
        thread: *mut u32,
        attr: *const u8,
        start_routine: unsafe extern "C" fn(APTR) -> APTR,
        arg: APTR,
    ) -> i32;
    fn pthread_join(thread: u32, retval: *mut APTR) -> i32;
}

/// Swap parent's DOS Input/Output/ErrorOutput to fresh `NIL:` handles
/// for the duration of pthread_create.
///
/// clib4's pthread_create internally calls `DupFileHandle()` on each
/// of the parent's three DOS stdio handles. Two failure modes show up
/// in practice:
///   - SerialShell launches with `ErrorOutput()` == BZERO → DupFileHandle
///     returns BZERO → pthread_create returns EAGAIN (DOS IoErr=211).
///   - Even with all three non-BZERO, a SerialShell CON: handle cannot
///     be duped (DOS IoErr=202 ERROR_OBJECT_IN_USE) → EAGAIN.
///
/// Both go away if we point the parent at three throwaway `NIL:` opens
/// for the brief window where pthread_create captures stdio. The child
/// task inherits the (NIL:) dupes; we restore the parent's originals
/// immediately so parent-side stdout/serial print continues to work.
struct StdioSwap {
    saved_in: u32,
    saved_out: u32,
    saved_err: u32,
    nil_in: u32,
    nil_out: u32,
    nil_err: u32,
}

impl StdioSwap {
    fn install() -> Self {
        unsafe {
            // MODE_NEWFILE = 1006 (dos/dos.h)
            let nil_in  = amigaos4_sys::dos_open(b"NIL:\0".as_ptr() as amigaos4_sys::CONST_STRPTR, 1006);
            let nil_out = amigaos4_sys::dos_open(b"NIL:\0".as_ptr() as amigaos4_sys::CONST_STRPTR, 1006);
            let nil_err = amigaos4_sys::dos_open(b"NIL:\0".as_ptr() as amigaos4_sys::CONST_STRPTR, 1006);
            let saved_in  = amigaos4_sys::dos_select_input(nil_in);
            let saved_out = amigaos4_sys::dos_select_output(nil_out);
            let saved_err = amigaos4_sys::dos_select_error_output(nil_err);
            StdioSwap { saved_in, saved_out, saved_err, nil_in, nil_out, nil_err }
        }
    }
}

impl Drop for StdioSwap {
    fn drop(&mut self) {
        unsafe {
            amigaos4_sys::dos_select_input(self.saved_in);
            amigaos4_sys::dos_select_output(self.saved_out);
            amigaos4_sys::dos_select_error_output(self.saved_err);
            if self.nil_in  != 0 { amigaos4_sys::dos_close(self.nil_in); }
            if self.nil_out != 0 { amigaos4_sys::dos_close(self.nil_out); }
            if self.nil_err != 0 { amigaos4_sys::dos_close(self.nil_err); }
        }
    }
}

/// Handle returned by `spawn()`. Call `join()` to wait for the thread result.
pub struct JoinHandle<T> {
    thread_id: u32,
    _marker: PhantomData<T>,
}

impl<T> JoinHandle<T> {
    /// Block until the thread completes and return its result.
    pub fn join(self) -> Result<T> {
        let mut retval: APTR = core::ptr::null_mut();
        let rc = unsafe { pthread_join(self.thread_id, &mut retval) };
        if rc != 0 {
            return Err(AmigaError::IoError(rc));
        }
        if retval.is_null() {
            return Err(AmigaError::NullPointer);
        }
        // Reconstruct the Box<T> from the returned pointer
        let boxed = unsafe { Box::from_raw(retval as *mut T) };
        Ok(*boxed)
    }
}

/// Trampoline function called by pthread_create.
/// `arg` is a Box<Box<dyn FnOnce() -> T>> cast to APTR.
unsafe extern "C" fn thread_trampoline<T>(arg: APTR) -> APTR {
    let closure: Box<Box<dyn FnOnce() -> T>> = Box::from_raw(arg as *mut Box<dyn FnOnce() -> T>);
    let result = closure();
    // Box the result and leak it — join() will reclaim it
    let boxed = Box::new(result);
    Box::into_raw(boxed) as APTR
}

/// Spawn a new thread running `f`. Returns a `JoinHandle` to retrieve the result.
///
/// # Example
/// ```ignore
/// let handle = amigaos4::thread::spawn(|| {
///     42u32
/// }).unwrap();
/// let result = handle.join().unwrap();
/// assert_eq!(result, 42);
/// ```
pub fn spawn<F, T>(f: F) -> Result<JoinHandle<T>>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    // Double-box: outer Box for the trait object, inner Box for the fat pointer
    let closure: Box<Box<dyn FnOnce() -> T>> = Box::new(Box::new(f));
    let arg = Box::into_raw(closure) as APTR;

    // Hold StdioSwap across pthread_create so the child task inherits
    // dupable NIL: stdio; Drop restores the parent's stdio immediately.
    let _swap = StdioSwap::install();

    let mut thread_id: u32 = 0;
    let rc = unsafe {
        pthread_create(
            &mut thread_id,
            core::ptr::null(),
            thread_trampoline::<T>,
            arg,
        )
    };
    if rc != 0 {
        // Reclaim the closure to avoid a leak
        unsafe { drop(Box::from_raw(arg as *mut Box<dyn FnOnce() -> T>)); }
        Err(AmigaError::IoError(rc))
    } else {
        Ok(JoinHandle {
            thread_id,
            _marker: PhantomData,
        })
    }
}

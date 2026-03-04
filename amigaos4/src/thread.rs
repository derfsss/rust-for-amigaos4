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

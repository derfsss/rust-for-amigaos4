//! Cooperative async executor for AmigaOS 4.
//!
//! This module provides a single-threaded async runtime that uses AmigaOS
//! Exec signals for efficient waking. No threads are used; all futures run
//! cooperatively on the calling task.
//!
//! # Design
//!
//! The executor allocates one Exec signal bit at creation time. All wakers
//! share that signal bit: when any waker fires, the executor's task is
//! signaled and all ready tasks are re-polled. This keeps the implementation
//! simple while remaining efficient for typical single-threaded workloads.
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::async_rt::Executor;
//!
//! let mut exec = Executor::new().unwrap();
//! exec.spawn(async {
//!     amigaos4::serial_println!("Hello from async!");
//! });
//! exec.run();
//! ```
//!
//! ## `block_on`
//!
//! ```ignore
//! let mut exec = Executor::new().unwrap();
//! let answer: u32 = exec.block_on(async { 42 });
//! assert_eq!(answer, 42);
//! ```

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::error::{AmigaError, Result};

// ---------------------------------------------------------------------------
// Task wrapper
// ---------------------------------------------------------------------------

/// A cooperatively scheduled async task.
struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    /// `true` when the task has been woken and should be polled.
    ready: bool,
}

// ---------------------------------------------------------------------------
// Executor
// ---------------------------------------------------------------------------

/// Single-threaded cooperative executor backed by Exec signals.
///
/// The executor owns a [`VecDeque`] of tasks and an Exec signal bit.  Call
/// [`spawn`](Executor::spawn) to enqueue futures, then [`run`](Executor::run)
/// to drive them all to completion.  When no task is ready the executor
/// blocks on [`exec_wait`](amigaos4_sys::exec_wait) instead of busy-looping.
pub struct Executor {
    tasks: VecDeque<Task>,
    /// The signal bit allocated for this executor (0..31), or -1 if invalid.
    signal_bit: i8,
    /// Pre-computed signal mask: `1 << signal_bit`.
    signal_mask: u32,
    /// Pointer to the owning AmigaOS task (i.e. ourselves).
    self_task: *mut amigaos4_sys::Task,
}

impl Executor {
    /// Create a new executor.
    ///
    /// Allocates an Exec signal bit for waking.  Returns
    /// [`AmigaError::NullPointer`] if the current task cannot be found, or
    /// [`AmigaError::AllocationFailed`] if no signal bits are available.
    pub fn new() -> Result<Self> {
        // SAFETY: passing null to exec_find_task returns the current task.
        let self_task = unsafe { amigaos4_sys::exec_find_task(core::ptr::null()) };
        if self_task.is_null() {
            return Err(AmigaError::NullPointer);
        }

        // SAFETY: -1 requests any free signal bit.
        let signal_bit = unsafe { amigaos4_sys::exec_alloc_signal(-1) };
        if signal_bit < 0 {
            return Err(AmigaError::AllocationFailed);
        }

        let signal_mask = 1u32 << (signal_bit as u32);

        Ok(Self {
            tasks: VecDeque::new(),
            signal_bit,
            signal_mask,
            self_task,
        })
    }

    /// Add an async task to the executor's run queue.
    ///
    /// The future will be polled for the first time during the next call to
    /// [`run`](Executor::run) or [`block_on`](Executor::block_on).
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.tasks.push_back(Task {
            future: Box::pin(future),
            ready: true, // poll once immediately
        });
    }

    /// Run all spawned tasks to completion.
    ///
    /// Blocks until every task has returned [`Poll::Ready`].  Between poll
    /// rounds the executor waits for its Exec signal, avoiding busy-loops.
    pub fn run(&mut self) {
        while !self.tasks.is_empty() {
            let made_progress = self.poll_ready_tasks();

            if !made_progress && !self.tasks.is_empty() {
                // Nothing completed and tasks remain. Peek at our signal
                // state via SetSignal(0, 0): if a Pending future already
                // called waker.wake() during this poll round, the bit
                // is set and we should re-poll immediately instead of
                // sleeping. This is what makes self-waking futures (e.g.
                // busy-poll async I/O) actually progress under this
                // executor — otherwise the bit gets cleared the moment
                // before we'd wait on it.
                let current = unsafe { amigaos4_sys::exec_set_signal(0, 0) };
                if current & self.signal_mask == 0 {
                    unsafe { amigaos4_sys::exec_wait(self.signal_mask); }
                }
                // Either path lands us here: clear the wake bit and
                // mark every pending task ready for the next round.
                unsafe { amigaos4_sys::exec_set_signal(0, self.signal_mask); }
                for task in &mut self.tasks {
                    task.ready = true;
                }
            }
        }
    }

    /// Run a single future to completion, returning its output.
    ///
    /// Any previously [`spawn`](Executor::spawn)ed tasks are also driven
    /// forward as a side effect.
    ///
    /// # Panics
    ///
    /// Panics (via `expect`) if the future does not produce a value —
    /// this should not happen in practice since `run` only returns once
    /// all tasks are complete.
    pub fn block_on<F, T>(&mut self, future: F) -> T
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        let result_cell: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
        let rc = result_cell.clone();

        let wrapper = async move {
            let val = future.await;
            *rc.borrow_mut() = Some(val);
        };

        self.spawn(wrapper);
        self.run();

        let result = result_cell.borrow_mut().take();
        result.expect("block_on: future did not complete")
    }

    // ---- internal helpers ------------------------------------------------

    /// Poll every task that is marked ready.
    ///
    /// Returns `true` if at least one task completed (was removed).
    fn poll_ready_tasks(&mut self) -> bool {
        let mut made_progress = false;
        let mut i = 0;

        while i < self.tasks.len() {
            if !self.tasks[i].ready {
                i += 1;
                continue;
            }

            self.tasks[i].ready = false;

            let waker = self.make_waker();
            let mut cx = Context::from_waker(&waker);

            let poll = self.tasks[i].future.as_mut().poll(&mut cx);

            match poll {
                Poll::Ready(()) => {
                    self.tasks.remove(i);
                    made_progress = true;
                    // Don't increment — the next element shifted into slot i.
                }
                Poll::Pending => {
                    i += 1;
                }
            }
        }

        made_progress
    }

    /// Build a [`Waker`] that signals this executor's task via Exec.
    fn make_waker(&self) -> Waker {
        let data = Box::new(WakerData {
            task: self.self_task,
            mask: self.signal_mask,
        });
        let raw = Box::into_raw(data) as *const ();

        // SAFETY: the vtable functions correctly manage the WakerData
        // allocation that `raw` points to.
        unsafe { Waker::from_raw(RawWaker::new(raw, &WAKER_VTABLE)) }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        if self.signal_bit >= 0 {
            // SAFETY: we allocated this signal bit in `new`.
            unsafe { amigaos4_sys::exec_free_signal(self.signal_bit) }
        }
    }
}

// ---------------------------------------------------------------------------
// Waker vtable (RawWaker implementation)
// ---------------------------------------------------------------------------

/// Heap-allocated data carried inside every [`Waker`] created by this
/// executor.
struct WakerData {
    /// The AmigaOS task to signal.
    task: *mut amigaos4_sys::Task,
    /// Signal mask (a single bit) to use.
    mask: u32,
}

/// Vtable for the raw waker.  All four callbacks manipulate `WakerData`
/// heap allocations.
static WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);

/// Clone: allocate a new `WakerData` with the same contents.
unsafe fn waker_clone(data: *const ()) -> RawWaker {
    let orig = &*(data as *const WakerData);
    let cloned = Box::new(WakerData {
        task: orig.task,
        mask: orig.mask,
    });
    RawWaker::new(Box::into_raw(cloned) as *const (), &WAKER_VTABLE)
}

/// Wake (by value): signal the task and free the `WakerData`.
unsafe fn waker_wake(data: *const ()) {
    let data = Box::from_raw(data as *mut WakerData);
    if !data.task.is_null() {
        amigaos4_sys::exec_signal(data.task, data.mask);
    }
    // `data` is dropped here, freeing the allocation.
}

/// Wake by reference: signal the task without consuming the waker.
unsafe fn waker_wake_by_ref(data: *const ()) {
    let data = &*(data as *const WakerData);
    if !data.task.is_null() {
        amigaos4_sys::exec_signal(data.task, data.mask);
    }
}

/// Drop: free the `WakerData` allocation.
unsafe fn waker_drop(data: *const ()) {
    let _ = Box::from_raw(data as *mut WakerData);
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn waker_data_size() {
        // WakerData: *mut Task (pointer) + u32 mask (with alignment padding)
        let size = mem::size_of::<WakerData>();
        assert!(size >= mem::size_of::<*mut amigaos4_sys::Task>() + mem::size_of::<u32>());
    }

    #[test]
    fn executor_struct_non_zero() {
        assert!(mem::size_of::<Executor>() > 0);
    }
}

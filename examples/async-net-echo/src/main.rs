//
// AmigaOS 4 async-net-echo - drives an HTTP/1.1 round-trip through
// amigaos4::net::AsyncTcpStream futures running on the
// amigaos4::async_rt::Executor.
//
// Demonstrates: the new async-TCP plumbing (set_nonblocking +
// AsyncTcpStream + ReadFut / WriteAllFut) running cooperatively
// under the no_std Executor. A second future runs alongside to
// prove the executor really does interleave work.
//
// Connects to 1.1.1.1:80 (same target as http-client), so the
// example is self-contained as long as bsdsocket has internet egress.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::rc::Rc;
use core::cell::RefCell;
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::async_rt::Executor;
use amigaos4::net::{SocketAddr, TcpStream, AsyncTcpStream};

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])           { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - async-net-echo\n\0");
    say(b"========================================\n\0");

    // 1. Connect blocking, then promote to non-blocking + async.
    let addr = SocketAddr::new([1, 1, 1, 1], 80);
    let raw = match TcpStream::connect(&addr) {
        Ok(s) => s,
        Err(_) => { say(b"FAIL: TCP connect refused\n\0"); return 1; }
    };
    let stream = match AsyncTcpStream::new(raw) {
        Ok(s) => s,
        Err(_) => { say(b"FAIL: set_nonblocking refused\n\0"); return 2; }
    };
    say_d(b"  Connected, async fd               = \0", stream.as_raw_fd() as u32);

    // 2. Build the executor. Two tasks: an HTTP round-trip on the
    //    socket future, and a counter that prints progress to show
    //    the executor is actually interleaving them.
    let mut exec = match Executor::new() {
        Ok(e) => e,
        Err(_) => { say(b"FAIL: Executor::new\n\0"); return 3; }
    };

    let stream = Rc::new(stream);
    let result_status = Rc::new(RefCell::new([0u8; 64]));
    let counter = Rc::new(RefCell::new(0u32));

    // ---- The "real work" future: write GET, read response head ----
    let s = stream.clone();
    let out = result_status.clone();
    exec.spawn(async move {
        let req: &[u8] = b"GET / HTTP/1.1\r\nHost: 1.1.1.1\r\nConnection: close\r\nUser-Agent: rust-for-amigaos4\r\n\r\n";
        if let Err(_) = s.write_all(req).await {
            say(b"  [http] write_all error\n\0");
            return;
        }
        say(b"  [http] wrote request\n\0");

        let mut buf = [0u8; 64];
        match s.read(&mut buf).await {
            Ok(0)  => say(b"  [http] EOF before any bytes\n\0"),
            Ok(n)  => {
                let end = buf[..n].iter().position(|&b| b == b'\r' || b == b'\n').unwrap_or(n);
                let dst_len = end.min(out.borrow().len() - 1);
                out.borrow_mut()[..dst_len].copy_from_slice(&buf[..dst_len]);
                out.borrow_mut()[dst_len] = 0;
            }
            Err(_) => say(b"  [http] read error\n\0"),
        }
        say(b"  [http] done\n\0");
    });

    // ---- The "I am interleaved" future: bump a counter a few times
    //      while the HTTP future is waiting on the network. ----
    let c = counter.clone();
    exec.spawn(async move {
        for _ in 0..3 {
            *c.borrow_mut() += 1;
            // Yield to give the network future a chance.
            YieldOnce { yielded: false }.await;
        }
    });

    // 3. Run the executor to completion.
    exec.run();

    // 4. Report results.
    let final_count = *counter.borrow();
    say_d(b"  Counter ticks while waiting       = \0", final_count);
    say(b"  HTTP status line: \0");
    unsafe { amiga_debug_str(result_status.borrow().as_ptr()); }
    say(b"\n\0");

    if final_count == 0 {
        say(b"FAIL: counter future never ran\n\0");
        return 4;
    }
    if result_status.borrow()[0] == 0 {
        say(b"FAIL: HTTP future produced no status line\n\0");
        return 5;
    }

    say(b"\n  => PASS: async executor interleaved a network future and a counter future\n\0");
    say(b"========================================\n\0");
    0
}

// One-shot yield-now future. Returns Pending once (after registering a
// self-wake) and Ready on the second poll. Useful for forcing the
// executor to consider another task in the middle of a closure.
struct YieldOnce { yielded: bool }
impl core::future::Future for YieldOnce {
    type Output = ();
    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<()> {
        if self.yielded {
            core::task::Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            core::task::Poll::Pending
        }
    }
}

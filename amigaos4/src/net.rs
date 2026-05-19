//! TCP networking via clib4 POSIX sockets (application mode only).
//!
//! Provides [`TcpStream`] and [`TcpListener`] with RAII cleanup, plus
//! [`SocketAddr`] for IPv4 address/port pairs. All socket operations go
//! through clib4's BSD-socket layer.
//!
//! Requires the `net` feature (included in `app` by default).

use crate::error::{AmigaError, Result};
use crate::io;

// ---------------------------------------------------------------------------
// clib4 POSIX FFI
// ---------------------------------------------------------------------------

extern "C" {
    fn socket(domain: i32, socket_type: i32, protocol: i32) -> i32;
    fn connect(socket: i32, address: *const SockAddrIn, address_len: u32) -> i32;
    fn bind(socket: i32, address: *const SockAddrIn, address_len: u32) -> i32;
    fn listen(socket: i32, backlog: i32) -> i32;
    fn accept(socket: i32, address: *mut SockAddrIn, address_len: *mut u32) -> i32;
    fn send(socket: i32, buffer: *const u8, length: u32, flags: i32) -> i32;
    fn recv(socket: i32, buffer: *mut u8, length: u32, flags: i32) -> i32;
    fn close(fd: i32) -> i32;
    fn __errno() -> *mut i32;
    fn setsockopt(
        socket: i32,
        level: i32,
        option_name: i32,
        option_value: *const u8,
        option_len: u32,
    ) -> i32;
    fn shutdown(socket: i32, how: i32) -> i32;
    fn fcntl(fd: i32, cmd: i32, arg: i32) -> i32;
}

// clib4 errno values; EAGAIN/EWOULDBLOCK collapse to the same value.
const EAGAIN: i32       = 35;
const EWOULDBLOCK: i32  = EAGAIN;
const EINPROGRESS: i32  = 36;

// clib4 fcntl.h
const F_GETFL: i32 = 3;
const F_SETFL: i32 = 4;
const O_NONBLOCK: i32 = 0x0004;

fn errno() -> i32 {
    unsafe { *__errno() }
}

// ---------------------------------------------------------------------------
// Constants (from clib4 headers)
// ---------------------------------------------------------------------------

const AF_INET: i32 = 2;
const SOCK_STREAM: i32 = 1;
const IPPROTO_TCP: i32 = 6;
const SOL_SOCKET: i32 = 0xFFFF;
const SO_REUSEADDR: i32 = 0x0004;
const SHUT_RDWR: i32 = 2;

// PPC AmigaOS is big-endian, so byte-order conversions are identity.
#[inline]
fn htons(v: u16) -> u16 {
    v
}

#[inline]
fn htonl(v: u32) -> u32 {
    v
}

#[inline]
fn ntohs(v: u16) -> u16 {
    v
}

#[inline]
fn ntohl(v: u32) -> u32 {
    v
}

// ---------------------------------------------------------------------------
// SockAddrIn — clib4 BSD-style sockaddr_in (packed(2) on PPC)
// ---------------------------------------------------------------------------

/// Raw C `sockaddr_in` for clib4 (BSD layout with `sin_len`).
#[repr(C, packed(2))]
struct SockAddrIn {
    sin_len: u8,
    sin_family: u8,
    sin_port: u16,
    sin_addr: u32,
    sin_zero: [u8; 8],
}

impl SockAddrIn {
    fn new(addr: &SocketAddr) -> Self {
        let ip_u32 = u32::from_be_bytes(addr.ip);
        Self {
            sin_len: core::mem::size_of::<SockAddrIn>() as u8,
            sin_family: AF_INET as u8,
            sin_port: htons(addr.port),
            sin_addr: htonl(ip_u32),
            sin_zero: [0u8; 8],
        }
    }

    fn to_socket_addr(&self) -> SocketAddr {
        let raw = ntohl(self.sin_addr);
        let ip = raw.to_be_bytes();
        let port = ntohs(self.sin_port);
        SocketAddr { ip, port }
    }
}

// ---------------------------------------------------------------------------
// SocketAddr
// ---------------------------------------------------------------------------

/// IPv4 socket address (IP + port).
#[derive(Debug, Clone, Copy)]
pub struct SocketAddr {
    /// IPv4 address octets, e.g. `[127, 0, 0, 1]`.
    pub ip: [u8; 4],
    /// Port number in host byte order.
    pub port: u16,
}

impl SocketAddr {
    /// Create a new socket address.
    pub fn new(ip: [u8; 4], port: u16) -> Self {
        Self { ip, port }
    }

    /// Create a socket address bound to all interfaces (`0.0.0.0`) on the
    /// given port.
    pub fn any(port: u16) -> Self {
        Self {
            ip: [0, 0, 0, 0],
            port,
        }
    }

    /// Parse from `"1.2.3.4:80"` format.
    ///
    /// The input slice may or may not be null-terminated; the parser does not
    /// require it. Returns [`AmigaError::IoError(0)`] on malformed input.
    pub fn parse(addr: &[u8]) -> Result<Self> {
        // Strip optional null terminator for parsing.
        let data = if addr.last() == Some(&0) {
            &addr[..addr.len() - 1]
        } else {
            addr
        };

        // Find the colon separating IP and port.
        let colon_pos = find_byte(data, b':').ok_or(AmigaError::IoError(0))?;
        let ip_part = &data[..colon_pos];
        let port_part = &data[colon_pos + 1..];

        // Parse four octets separated by dots.
        let ip = parse_ipv4(ip_part)?;

        // Parse port number.
        let port = parse_u16(port_part)?;

        Ok(Self { ip, port })
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

fn find_byte(data: &[u8], needle: u8) -> Option<usize> {
    let mut i = 0;
    while i < data.len() {
        if data[i] == needle {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn parse_ipv4(data: &[u8]) -> Result<[u8; 4]> {
    let mut octets = [0u8; 4];
    let mut octet_idx = 0usize;
    let mut acc: u16 = 0;
    let mut digits = 0u8;

    for &b in data {
        if b == b'.' {
            if digits == 0 || octet_idx >= 3 || acc > 255 {
                return Err(AmigaError::IoError(0));
            }
            octets[octet_idx] = acc as u8;
            octet_idx += 1;
            acc = 0;
            digits = 0;
        } else if b >= b'0' && b <= b'9' {
            acc = acc * 10 + (b - b'0') as u16;
            digits += 1;
            if digits > 3 || acc > 255 {
                return Err(AmigaError::IoError(0));
            }
        } else {
            return Err(AmigaError::IoError(0));
        }
    }

    // Final octet
    if digits == 0 || octet_idx != 3 || acc > 255 {
        return Err(AmigaError::IoError(0));
    }
    octets[3] = acc as u8;

    Ok(octets)
}

fn parse_u16(data: &[u8]) -> Result<u16> {
    if data.is_empty() {
        return Err(AmigaError::IoError(0));
    }
    let mut acc: u32 = 0;
    for &b in data {
        if b >= b'0' && b <= b'9' {
            acc = acc * 10 + (b - b'0') as u32;
            if acc > 65535 {
                return Err(AmigaError::IoError(0));
            }
        } else {
            return Err(AmigaError::IoError(0));
        }
    }
    Ok(acc as u16)
}

// ---------------------------------------------------------------------------
// TcpStream
// ---------------------------------------------------------------------------

/// A connected TCP socket with RAII cleanup.
///
/// Implements [`io::Read`] and [`io::Write`]. The underlying file descriptor
/// is closed on drop.
pub struct TcpStream {
    fd: i32,
}

impl TcpStream {
    /// Connect to `addr`.
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        let fd = unsafe { socket(AF_INET, SOCK_STREAM, IPPROTO_TCP) };
        if fd < 0 {
            return Err(AmigaError::IoError(errno()));
        }

        let sa = SockAddrIn::new(addr);
        let rc = unsafe {
            connect(
                fd,
                &sa as *const SockAddrIn,
                core::mem::size_of::<SockAddrIn>() as u32,
            )
        };
        if rc < 0 {
            let e = errno();
            unsafe { close(fd); }
            return Err(AmigaError::IoError(e));
        }

        Ok(Self { fd })
    }

    /// Wrap an already-connected file descriptor (e.g. from `TcpListener::accept`).
    ///
    /// # Safety contract
    ///
    /// The caller must ensure `fd` is a valid, connected socket descriptor.
    /// Ownership transfers to this struct; it will be closed on drop.
    pub fn from_fd(fd: i32) -> Self {
        Self { fd }
    }

    /// Return the raw file descriptor.
    #[inline]
    pub fn as_raw_fd(&self) -> i32 {
        self.fd
    }

    /// Switch the socket to non-blocking mode. After this, read/write
    /// calls return immediately; if no data is available recv() returns
    /// -1 with errno=EAGAIN/EWOULDBLOCK, send() can return a short
    /// count, and `try_read`/`try_write` (or [`AsyncTcpStream`]) become
    /// usable.
    pub fn set_nonblocking(&self, on: bool) -> Result<()> {
        let flags = unsafe { fcntl(self.fd, F_GETFL, 0) };
        if flags < 0 {
            return Err(AmigaError::IoError(errno()));
        }
        let newflags = if on { flags | O_NONBLOCK } else { flags & !O_NONBLOCK };
        let rc = unsafe { fcntl(self.fd, F_SETFL, newflags) };
        if rc < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(())
        }
    }

    /// Non-blocking recv. Returns `Ok(Some(n))` if `n>0` bytes were
    /// read (`n==0` means orderly EOF), `Ok(None)` if the socket would
    /// have blocked, or `Err` on a hard failure. Requires the socket
    /// to have been put into non-blocking mode first.
    pub fn try_read(&self, buf: &mut [u8]) -> Result<Option<usize>> {
        let n = unsafe { recv(self.fd, buf.as_mut_ptr(), buf.len() as u32, 0) };
        if n >= 0 {
            return Ok(Some(n as usize));
        }
        let e = errno();
        if e == EAGAIN || e == EWOULDBLOCK {
            Ok(None)
        } else {
            Err(AmigaError::IoError(e))
        }
    }

    /// Non-blocking send.  Returns `Ok(Some(n))` for the bytes sent
    /// (possibly short), `Ok(None)` if the socket would have blocked,
    /// or `Err` on a hard failure.
    pub fn try_write(&self, buf: &[u8]) -> Result<Option<usize>> {
        let n = unsafe { send(self.fd, buf.as_ptr(), buf.len() as u32, 0) };
        if n >= 0 {
            return Ok(Some(n as usize));
        }
        let e = errno();
        if e == EAGAIN || e == EWOULDBLOCK {
            Ok(None)
        } else {
            Err(AmigaError::IoError(e))
        }
    }

    /// Shut down both halves of the connection.
    pub fn shutdown(&self) -> Result<()> {
        let rc = unsafe { shutdown(self.fd, SHUT_RDWR) };
        if rc < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(())
        }
    }
}

impl io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = unsafe { recv(self.fd, buf.as_mut_ptr(), buf.len() as u32, 0) };
        if n < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(n as usize)
        }
    }
}

impl io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = unsafe { send(self.fd, buf.as_ptr(), buf.len() as u32, 0) };
        if n < 0 {
            Err(AmigaError::IoError(errno()))
        } else {
            Ok(n as usize)
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(()) // TCP has no user-space buffer to flush here
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        if self.fd >= 0 {
            unsafe {
                close(self.fd);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AsyncTcpStream — minimal Future-based wrapper
// ---------------------------------------------------------------------------
//
// This is a thin shim on top of a non-blocking TcpStream. Each read or
// write call is exposed as a future that polls the underlying syscall
// once per turn; on EAGAIN it re-wakes itself via cx.waker().wake_by_ref
// so the executor re-polls without sleeping.
//
// That makes this a busy-poll cooperative loop, not a proper readiness
// scheduler. It works fine for examples that want to interleave a
// handful of socket-driven tasks with other futures, but it does spin
// at 100% CPU while idle — the AmigaOS-native way to fix that would be
// to integrate `WaitSelect()` into the executor's wait point. That
// belongs in a follow-up patch; for now this is enough to demonstrate
// the API.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/// Future-based wrapper over a non-blocking [`TcpStream`].
///
/// Created by [`TcpStream::into_async`]. Read/write operations return
/// futures that yield via `cx.waker().wake_by_ref()` on EAGAIN.
pub struct AsyncTcpStream {
    inner: TcpStream,
}

impl AsyncTcpStream {
    /// Wrap an existing [`TcpStream`], putting it in non-blocking mode.
    pub fn new(stream: TcpStream) -> Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self { inner: stream })
    }

    /// Return the raw file descriptor.
    #[inline]
    pub fn as_raw_fd(&self) -> i32 {
        self.inner.as_raw_fd()
    }

    /// Convenience: connect blocking, switch to non-blocking, return.
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        let s = TcpStream::connect(addr)?;
        Self::new(s)
    }

    /// Future that resolves to either `Ok(n)` bytes read (0 = EOF) or
    /// `Err`. Calls into `recv()` on each poll.
    pub fn read<'a>(&'a self, buf: &'a mut [u8]) -> ReadFut<'a> {
        ReadFut { stream: self, buf }
    }

    /// Future that writes the full buffer, looping internally over
    /// short writes. Resolves to `Ok(())` on success.
    pub fn write_all<'a>(&'a self, buf: &'a [u8]) -> WriteAllFut<'a> {
        WriteAllFut { stream: self, buf, sent: 0 }
    }
}

/// Borrowing read future returned by [`AsyncTcpStream::read`].
pub struct ReadFut<'a> {
    stream: &'a AsyncTcpStream,
    buf: &'a mut [u8],
}

impl<'a> Future for ReadFut<'a> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: we don't move anything from `this`; we only re-borrow
        // the underlying buf slice.
        let this = unsafe { self.get_unchecked_mut() };
        match this.stream.inner.try_read(&mut *this.buf) {
            Ok(Some(n)) => Poll::Ready(Ok(n)),
            Ok(None) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

/// Borrowing write-all future returned by [`AsyncTcpStream::write_all`].
pub struct WriteAllFut<'a> {
    stream: &'a AsyncTcpStream,
    buf: &'a [u8],
    sent: usize,
}

impl<'a> Future for WriteAllFut<'a> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        while this.sent < this.buf.len() {
            let remaining = &this.buf[this.sent..];
            match this.stream.inner.try_write(remaining) {
                Ok(Some(0)) => {
                    return Poll::Ready(Err(AmigaError::IoError(0)));
                }
                Ok(Some(n)) => {
                    this.sent += n;
                }
                Ok(None) => {
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                }
                Err(e) => return Poll::Ready(Err(e)),
            }
        }
        Poll::Ready(Ok(()))
    }
}

// ---------------------------------------------------------------------------
// TcpListener
// ---------------------------------------------------------------------------

/// A listening TCP socket with RAII cleanup.
///
/// Call [`accept`](TcpListener::accept) in a loop to handle incoming
/// connections.
pub struct TcpListener {
    fd: i32,
}

impl TcpListener {
    /// Bind to `addr` and start listening with the given `backlog`.
    pub fn bind(addr: &SocketAddr, backlog: i32) -> Result<Self> {
        let fd = unsafe { socket(AF_INET, SOCK_STREAM, IPPROTO_TCP) };
        if fd < 0 {
            return Err(AmigaError::IoError(errno()));
        }

        // Set SO_REUSEADDR so we can rebind quickly after a restart.
        let one: i32 = 1;
        let rc = unsafe {
            setsockopt(
                fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                &one as *const i32 as *const u8,
                core::mem::size_of::<i32>() as u32,
            )
        };
        if rc < 0 {
            let e = errno();
            unsafe { close(fd); }
            return Err(AmigaError::IoError(e));
        }

        let sa = SockAddrIn::new(addr);
        let rc = unsafe {
            bind(
                fd,
                &sa as *const SockAddrIn,
                core::mem::size_of::<SockAddrIn>() as u32,
            )
        };
        if rc < 0 {
            let e = errno();
            unsafe { close(fd); }
            return Err(AmigaError::IoError(e));
        }

        let rc = unsafe { listen(fd, backlog) };
        if rc < 0 {
            let e = errno();
            unsafe { close(fd); }
            return Err(AmigaError::IoError(e));
        }

        Ok(Self { fd })
    }

    /// Accept a connection, returning the stream and client address.
    ///
    /// Blocks until a client connects.
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        let mut client_addr: SockAddrIn = unsafe { core::mem::zeroed() };
        let mut addr_len = core::mem::size_of::<SockAddrIn>() as u32;

        let client_fd = unsafe {
            accept(
                self.fd,
                &mut client_addr as *mut SockAddrIn,
                &mut addr_len as *mut u32,
            )
        };
        if client_fd < 0 {
            return Err(AmigaError::IoError(errno()));
        }

        let addr = client_addr.to_socket_addr();
        Ok((TcpStream::from_fd(client_fd), addr))
    }

    /// Return the raw file descriptor.
    #[inline]
    pub fn as_raw_fd(&self) -> i32 {
        self.fd
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        if self.fd >= 0 {
            unsafe {
                close(self.fd);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_socket_addr() {
        let addr = SocketAddr::parse(b"127.0.0.1:8080").unwrap();
        assert_eq!(addr.ip, [127, 0, 0, 1]);
        assert_eq!(addr.port, 8080);
    }

    #[test]
    fn parse_socket_addr_nul_terminated() {
        let addr = SocketAddr::parse(b"10.0.0.1:80\0").unwrap();
        assert_eq!(addr.ip, [10, 0, 0, 1]);
        assert_eq!(addr.port, 80);
    }

    #[test]
    fn parse_socket_addr_max_octets() {
        let addr = SocketAddr::parse(b"255.255.255.255:65535").unwrap();
        assert_eq!(addr.ip, [255, 255, 255, 255]);
        assert_eq!(addr.port, 65535);
    }

    #[test]
    fn parse_socket_addr_zero() {
        let addr = SocketAddr::parse(b"0.0.0.0:0").unwrap();
        assert_eq!(addr.ip, [0, 0, 0, 0]);
        assert_eq!(addr.port, 0);
    }

    #[test]
    fn parse_socket_addr_no_colon() {
        assert!(SocketAddr::parse(b"127.0.0.1").is_err());
    }

    #[test]
    fn parse_socket_addr_invalid_octet() {
        assert!(SocketAddr::parse(b"256.0.0.1:80").is_err());
    }

    #[test]
    fn parse_socket_addr_port_overflow() {
        assert!(SocketAddr::parse(b"1.2.3.4:99999").is_err());
    }

    #[test]
    fn parse_socket_addr_empty() {
        assert!(SocketAddr::parse(b"").is_err());
    }

    #[test]
    fn socket_addr_any() {
        let a = SocketAddr::any(1234);
        assert_eq!(a.ip, [0, 0, 0, 0]);
        assert_eq!(a.port, 1234);
    }

    // ---- IPv4 parsing edge cases ----

    #[test]
    fn parse_ipv4_one_octet() {
        assert!(parse_ipv4(b"1").is_err());
    }

    #[test]
    fn parse_ipv4_three_octets() {
        assert!(parse_ipv4(b"1.2.3").is_err());
    }

    #[test]
    fn parse_ipv4_five_octets() {
        assert!(parse_ipv4(b"1.2.3.4.5").is_err());
    }

    #[test]
    fn parse_ipv4_leading_dot() {
        assert!(parse_ipv4(b".1.2.3").is_err());
    }

    #[test]
    fn parse_ipv4_trailing_dot() {
        assert!(parse_ipv4(b"1.2.3.").is_err());
    }

    #[test]
    fn parse_ipv4_double_dot() {
        assert!(parse_ipv4(b"1..2.3").is_err());
    }

    #[test]
    fn parse_ipv4_non_digit() {
        assert!(parse_ipv4(b"1.2.a.3").is_err());
    }

    // ---- u16 parsing edge cases ----

    #[test]
    fn parse_u16_empty() {
        assert!(parse_u16(b"").is_err());
    }

    #[test]
    fn parse_u16_zero() {
        assert_eq!(parse_u16(b"0").unwrap(), 0);
    }

    #[test]
    fn parse_u16_max() {
        assert_eq!(parse_u16(b"65535").unwrap(), 65535);
    }

    #[test]
    fn parse_u16_overflow() {
        assert!(parse_u16(b"65536").is_err());
    }

    #[test]
    fn parse_u16_non_digit() {
        assert!(parse_u16(b"80ab").is_err());
    }

    // ---- SockAddrIn layout ----

    #[test]
    fn sock_addr_in_size() {
        assert_eq!(core::mem::size_of::<SockAddrIn>(), 16);
    }

    #[test]
    fn sock_addr_in_roundtrip() {
        let addr = SocketAddr::new([192, 168, 1, 100], 8080);
        let sa = SockAddrIn::new(&addr);
        let back = sa.to_socket_addr();
        assert_eq!(back.ip, addr.ip);
        assert_eq!(back.port, addr.port);
    }

    // ---- SocketAddr constructors ----

    #[test]
    fn socket_addr_new_fields() {
        let a = SocketAddr::new([1, 2, 3, 4], 80);
        assert_eq!(a.ip, [1, 2, 3, 4]);
        assert_eq!(a.port, 80);
    }

    #[test]
    fn socket_addr_debug_display() {
        extern crate alloc;
        let a = SocketAddr::new([10, 0, 0, 1], 443);
        let dbg = alloc::format!("{:?}", a);
        assert!(!dbg.is_empty());
    }
}

//! DNS resolution via clib4 gethostbyname (application mode only).
//!
//! Provides [`resolve`] to look up a hostname and return one or more IPv4
//! addresses. The result can be fed directly to
//! [`SocketAddr::new`](crate::net::SocketAddr::new) and then to
//! [`TcpStream::connect`](crate::net::TcpStream::connect).
//!
//! Requires the `net` feature.

use alloc::vec::Vec;
use crate::error::{AmigaError, Result};

// ---------------------------------------------------------------------------
// clib4 POSIX FFI
// ---------------------------------------------------------------------------

/// Layout matches clib4's `struct hostent` (packed(2) on PPC).
#[repr(C, packed(2))]
struct HostEnt {
    h_name: *const u8,
    h_aliases: *const *const u8,
    h_addrtype: i32,
    h_length: i32,
    h_addr_list: *const *const u8,
}

extern "C" {
    fn gethostbyname(name: *const u8) -> *const HostEnt;
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Resolve a hostname to one or more IPv4 addresses.
///
/// `hostname` must be a null-terminated byte slice (e.g. `b"example.com\0"`).
///
/// Returns a `Vec` of `[u8; 4]` IPv4 address octets. The first entry is
/// typically the preferred address.
///
/// # Errors
///
/// Returns [`AmigaError::NotNulTerminated`] if `hostname` is missing its
/// `\0`, and [`AmigaError::HostNotFound`] if the name cannot be resolved
/// or the result contains no IPv4 addresses.
pub fn resolve(hostname: &[u8]) -> Result<Vec<[u8; 4]>> {
    let name_ptr = crate::cstr::require_nul(hostname)?;
    let he = unsafe { gethostbyname(name_ptr) };
    if he.is_null() {
        return Err(AmigaError::HostNotFound);
    }

    let mut addrs = Vec::new();
    unsafe {
        // Only IPv4 results are supported — each address must be 4 octets,
        // otherwise the fixed-size reads below would over- or under-read.
        if (*he).h_length != 4 {
            return Err(AmigaError::HostNotFound);
        }
        let list = (*he).h_addr_list;
        if list.is_null() {
            return Err(AmigaError::HostNotFound);
        }
        let mut i = 0usize;
        while !(*list.add(i)).is_null() {
            let addr_ptr = *list.add(i);
            let ip = [
                *addr_ptr.add(0),
                *addr_ptr.add(1),
                *addr_ptr.add(2),
                *addr_ptr.add(3),
            ];
            addrs.push(ip);
            i += 1;
            // Safety cap — a single hostname should never resolve to
            // hundreds of addresses; bail out to prevent runaway reads.
            if i >= 32 {
                break;
            }
        }
    }

    if addrs.is_empty() {
        Err(AmigaError::HostNotFound)
    } else {
        Ok(addrs)
    }
}

# Phase 5 — Networking

Started: 2026-03-15
Completed: 2026-03-15

## Goal

Enable network-capable applications.

## Tasks

### 1. Socket wrappers (net module)
- **Status:** Complete
- **Deliverable:** `amigaos4/src/net.rs` (480+ lines)
- **Contents:**
  - `SocketAddr` — IPv4 address + port, with `new()`, `any()`, `parse("ip:port")`
  - `TcpStream` — RAII connected socket with `connect()`, `shutdown()`, `from_fd()`, implements `io::Read` + `io::Write`
  - `TcpListener` — RAII listening socket with `bind()` + `accept()`, SO_REUSEADDR
  - `SockAddrIn` — repr(C) internal struct matching clib4's sockaddr_in
  - htons/htonl as identity functions (PPC is big-endian)
  - IPv4 parser with full validation (octet range, port range)
  - 9 host-side unit tests for SocketAddr parsing

### 2. DNS resolution
- **Status:** Complete
- **Deliverable:** `amigaos4/src/dns.rs` (83 lines)
- **Contents:**
  - `resolve(hostname)` — wraps gethostbyname, returns `Vec<[u8; 4]>`
  - Safety cap at 32 addresses
  - Null checks on hostent and addr_list

### 3. HTTP client
- **Status:** Complete
- **Deliverable:** `amigaos4/src/http.rs` (180+ lines)
- **Contents:**
  - `HttpResponse` — status_code + body
  - `get(host, port, path)` — HTTP/1.1 GET with Connection: close
  - Internal helpers: parse_status_code, find_body_start, strip_nul
  - 8 host-side unit tests for parsing

### 4. TLS (AmiSSL)
- **Status:** Deferred
- **Notes:** AmiSSL requires complex library initialization. Deferred to a dedicated phase.

### 5. Network example
- **Status:** Complete
- **Deliverable:** `examples/net-demo/` (7 files)
- **Contents:** DNS resolution, TCP connection, HTTP GET, all with graceful error handling

---

## Issues Log

No issues found during implementation or code review.

---

## Changes from Original Plan

1. **TLS deferred** — AmiSSL integration too complex for this phase
2. **Added SocketAddr::parse()** — Not in original plan, but useful for config parsing
3. **Added host-side tests** — 17 unit tests for parsing logic in net.rs and http.rs
4. **htons/htonl as Rust identity functions** — PPC is big-endian, no byte swap needed. Avoids FFI call overhead.

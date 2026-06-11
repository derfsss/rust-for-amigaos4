//! Host-side integration tests of the amigaos4 crate (migrated from
//! amigaos4/tests/host_tests.rs).
//!
//! Exercises pure-Rust logic that has no AmigaOS runtime dependency:
//!   - error: AmigaError Display, Copy, Result alias
//!   - time: Duration constructors and arithmetic (NOT Instant)
//!   - tag: TagListBuilder, conditional tag_if
//!   - io: Read/Write traits via mock implementations
//!
//! Gated behind the `link-amigaos4` Cargo feature because importing the
//! amigaos4 crate causes the host link step to fail on Windows
//! (unresolved IExec/IDOS/IIntuition globals — Linux ld resolves these
//! lazily, Windows MSVC linker does not).
//!
//! Run on Linux: `cargo test --features link-amigaos4`
//! Run on Windows: not currently possible without gating the AmigaOS
//! wrappers behind `#[cfg(target_os = "amigaos")]` upstream.

#![cfg(feature = "link-amigaos4")]

use amigaos4::error::AmigaError;
use amigaos4::tag::TagListBuilder;
use amigaos4_sys::{TAG_DONE, TAG_IGNORE, TAG_USER};

// ============================================================
// error module
// ============================================================

#[test]
fn error_display_all_variants() {
    assert_eq!(format!("{}", AmigaError::NullPointer), "null pointer");
    assert_eq!(format!("{}", AmigaError::AllocationFailed), "allocation failed");
    assert_eq!(format!("{}", AmigaError::IoError(13)), "I/O error (errno=13)");
    assert_eq!(format!("{}", AmigaError::DosError(205)), "DOS error (205)");
}

#[test]
fn error_is_copy_and_eq() {
    let a = AmigaError::IoError(5);
    let b = a; // Copy
    assert_eq!(a, b);
}

#[test]
fn result_alias_works() {
    let ok: amigaos4::error::Result<i32> = Ok(42);
    assert_eq!(ok.unwrap(), 42);

    let err: amigaos4::error::Result<i32> = Err(AmigaError::NullPointer);
    assert!(err.is_err());
}

// ============================================================
// time module
// ============================================================

mod time_tests {
    use amigaos4::time::Duration;

    #[test]
    fn duration_from_secs_and_back() {
        let d = Duration::from_secs(7);
        assert_eq!(d.as_secs(), 7);
        assert_eq!(d.subsec_nanos(), 0);
    }

    #[test]
    fn duration_from_millis_split() {
        let d = Duration::from_millis(2_345);
        assert_eq!(d.as_secs(), 2);
        assert_eq!(d.as_millis(), 2_345);
    }

    #[test]
    fn duration_add_with_carry() {
        let a = Duration::from_nanos(999_999_999);
        let b = Duration::from_nanos(2);
        let c = a + b;
        assert_eq!(c.as_secs(), 1);
        assert_eq!(c.subsec_nanos(), 1);
    }

    #[test]
    fn duration_sub_saturates() {
        let small = Duration::from_millis(1);
        let big = Duration::from_secs(1);
        let result = small - big;
        assert_eq!(result, Duration::ZERO);
    }
}

// ============================================================
// tag module
// ============================================================

#[test]
fn tag_builder_empty() {
    let tags = TagListBuilder::new().build();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].ti_Tag, TAG_DONE);
}

#[test]
fn tag_builder_conditional() {
    let tags = TagListBuilder::new()
        .tag_if(true, TAG_USER + 10, 0xAA)
        .tag_if(false, TAG_USER + 20, 0xBB)
        .build();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0].ti_Tag, TAG_USER + 10);
    assert_eq!(tags[0].ti_Data, 0xAA);
    assert_eq!(tags[1].ti_Tag, TAG_IGNORE);
    assert_eq!(tags[2].ti_Tag, TAG_DONE);
}

// ============================================================
// io module (via mock implementations)
// ============================================================

mod io_tests {
    use amigaos4::error::{AmigaError, Result};
    use amigaos4::io::{Read, Write};

    struct Cursor {
        data: Vec<u8>,
        pos: usize,
    }

    impl Cursor {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
    }

    impl Read for Cursor {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let remaining = &self.data[self.pos..];
            let n = buf.len().min(remaining.len());
            buf[..n].copy_from_slice(&remaining[..n]);
            self.pos += n;
            Ok(n)
        }
    }

    struct Sink {
        written: Vec<u8>,
    }

    impl Sink {
        fn new() -> Self {
            Self { written: Vec::new() }
        }
    }

    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    #[test]
    fn read_exact_success() {
        let mut c = Cursor::new(b"abcdef".to_vec());
        let mut buf = [0u8; 6];
        c.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"abcdef");
    }

    #[test]
    fn read_exact_eof_error() {
        let mut c = Cursor::new(b"ab".to_vec());
        let mut buf = [0u8; 5];
        let err = c.read_exact(&mut buf).unwrap_err();
        assert_eq!(err, AmigaError::UnexpectedEof);
    }

    #[test]
    fn write_all_success() {
        let mut s = Sink::new();
        s.write_all(b"hello world").unwrap();
        assert_eq!(&s.written, b"hello world");
    }

    #[test]
    fn flush_default_is_ok() {
        let mut s = Sink::new();
        s.flush().unwrap();
    }
}

// ============================================================
// timer module — TimerVal (pure data arithmetic)
// ============================================================

mod timer_tests {
    use amigaos4::timer::TimerVal;

    #[test]
    fn zero_is_zero() {
        assert_eq!(TimerVal::ZERO.secs, 0);
        assert_eq!(TimerVal::ZERO.micros, 0);
    }

    #[test]
    fn new_round_trip() {
        let t = TimerVal::new(3, 500_000);
        assert_eq!(t.secs, 3);
        assert_eq!(t.micros, 500_000);
    }

    #[test]
    fn from_millis_splits_seconds_and_micros() {
        let t = TimerVal::from_millis(2_500);
        assert_eq!(t.secs, 2);
        assert_eq!(t.micros, 500_000);
    }

    #[test]
    fn from_millis_under_a_second() {
        let t = TimerVal::from_millis(750);
        assert_eq!(t.secs, 0);
        assert_eq!(t.micros, 750_000);
    }

    #[test]
    fn as_millis_round_trip() {
        assert_eq!(TimerVal::from_millis(1_234).as_millis(), 1_234);
        assert_eq!(TimerVal::from_millis(60_000).as_millis(), 60_000);
    }

    #[test]
    fn is_copy_and_eq() {
        let a = TimerVal::new(5, 100);
        let b = a; // Copy
        assert_eq!(a, b);
    }
}

// ============================================================
// window module — IDCMP/WA constants and IntuiMsg shape
// ============================================================

mod window_tests {
    use amigaos4::window::{
        IntuiMsg, IDCMP_CLOSEWINDOW, IDCMP_GADGETDOWN, IDCMP_GADGETUP,
        IDCMP_INTUITICKS, IDCMP_MENUPICK, IDCMP_MOUSEBUTTONS, IDCMP_MOUSEMOVE,
        IDCMP_NEWSIZE, IDCMP_RAWKEY, IDCMP_REFRESHWINDOW, IDCMP_VANILLAKEY,
        WA_HEIGHT, WA_LEFT, WA_TOP, WA_WIDTH,
    };

    #[test]
    fn idcmp_close_window_matches_sdk_value() {
        // Documented value from intuition/intuition.h.
        assert_eq!(IDCMP_CLOSEWINDOW, 0x0000_0200);
    }

    #[test]
    fn idcmp_flags_are_distinct() {
        let flags = [
            IDCMP_NEWSIZE, IDCMP_REFRESHWINDOW, IDCMP_MOUSEBUTTONS,
            IDCMP_MOUSEMOVE, IDCMP_GADGETDOWN, IDCMP_GADGETUP,
            IDCMP_MENUPICK, IDCMP_CLOSEWINDOW, IDCMP_RAWKEY,
            IDCMP_VANILLAKEY, IDCMP_INTUITICKS,
        ];
        for (i, a) in flags.iter().enumerate() {
            for b in &flags[i + 1..] {
                assert_ne!(*a, *b, "duplicate IDCMP flag bit");
            }
        }
    }

    #[test]
    fn wa_tag_ids_strictly_increasing() {
        assert!(WA_LEFT < WA_TOP);
        assert!(WA_TOP < WA_WIDTH);
        assert!(WA_WIDTH < WA_HEIGHT);
    }

    #[test]
    fn intui_msg_is_copy() {
        let m = IntuiMsg {
            class: IDCMP_CLOSEWINDOW,
            code: 0,
            qualifier: 0,
            mouse_x: 10,
            mouse_y: 20,
            gadget_id: 0,
        };
        let c = m; // Copy
        assert_eq!(c.class, m.class);
        assert_eq!(c.mouse_x, 10);
        assert_eq!(c.mouse_y, 20);
    }
}

// ============================================================
// fmt module — Serial type shape and entry point
// ============================================================

mod fmt_tests {
    use amigaos4::fmt::{serial, Serial};
    use core::mem::size_of;

    #[test]
    fn serial_is_zero_sized() {
        assert_eq!(size_of::<Serial>(), 0);
    }

    #[test]
    fn serial_fn_returns_serial() {
        let _s: Serial = serial();
    }
}

// ============================================================
// Type-shape sanity checks for RAII wrappers
// ============================================================
//
// These don't construct the wrappers (constructors call FFI), but
// they pin down the in-memory layout — guarding against accidental
// field-list bloat that would change the ABI.

mod shape_tests {
    use core::mem::size_of;

    #[test]
    fn amiga_msg_port_is_pointer_sized() {
        assert_eq!(
            size_of::<amigaos4::port::AmigaMsgPort>(),
            size_of::<*mut amigaos4_sys::MsgPort>()
        );
    }

    #[test]
    fn amiga_vec_is_ptr_plus_usize() {
        assert_eq!(
            size_of::<amigaos4::mem::AmigaVec>(),
            size_of::<*mut u8>() + size_of::<usize>()
        );
    }

    #[test]
    fn amiga_catalog_is_pointer_sized() {
        assert_eq!(
            size_of::<amigaos4::locale::AmigaCatalog>(),
            size_of::<*mut amigaos4_sys::Catalog>()
        );
    }
}

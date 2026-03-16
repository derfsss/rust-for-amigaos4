//! Monotonic time measurement via clib4 `clock_gettime` (application mode only).
//!
//! [`Duration`] represents a span of time (seconds + nanoseconds).
//! [`Instant`] captures a monotonic clock snapshot for measuring elapsed time.
//!
//! Requires the `time` feature (enabled by default via `app`).

extern "C" {
    fn clock_gettime(clk_id: u32, tp: *mut Timespec) -> i32;
}

const CLOCK_MONOTONIC: u32 = 1;

/// clib4 timespec layout (PPC: long = 32-bit).
#[repr(C)]
struct Timespec {
    tv_sec: i32,
    tv_nsec: i32,
}

/// A duration of time (seconds + nanoseconds).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    secs: u64,
    nanos: u32,
}

impl Duration {
    /// A duration of zero length.
    pub const ZERO: Duration = Duration { secs: 0, nanos: 0 };

    /// Create a duration from whole seconds.
    #[inline]
    pub fn from_secs(secs: u64) -> Self {
        Self { secs, nanos: 0 }
    }

    /// Create a duration from milliseconds.
    #[inline]
    pub fn from_millis(millis: u64) -> Self {
        Self {
            secs: millis / 1_000,
            nanos: ((millis % 1_000) * 1_000_000) as u32,
        }
    }

    /// Create a duration from nanoseconds.
    #[inline]
    pub fn from_nanos(nanos: u64) -> Self {
        Self {
            secs: nanos / 1_000_000_000,
            nanos: (nanos % 1_000_000_000) as u32,
        }
    }

    /// Return the whole seconds component.
    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.secs
    }

    /// Return the total duration in milliseconds.
    #[inline]
    pub fn as_millis(&self) -> u64 {
        self.secs * 1_000 + self.nanos as u64 / 1_000_000
    }

    /// Return the total duration in nanoseconds.
    #[inline]
    pub fn as_nanos(&self) -> u64 {
        self.secs * 1_000_000_000 + self.nanos as u64
    }

    /// Return the nanosecond fractional part (0..999_999_999).
    #[inline]
    pub fn subsec_nanos(&self) -> u32 {
        self.nanos
    }
}

impl core::ops::Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        let total_nanos_lhs = self.secs * 1_000_000_000 + self.nanos as u64;
        let total_nanos_rhs = rhs.secs * 1_000_000_000 + rhs.nanos as u64;
        let diff = total_nanos_lhs.saturating_sub(total_nanos_rhs);
        Duration::from_nanos(diff)
    }
}

impl core::ops::Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        let total_nanos = self.as_nanos() + rhs.as_nanos();
        Duration::from_nanos(total_nanos)
    }
}

/// A monotonic clock instant.
#[derive(Debug, Clone, Copy)]
pub struct Instant {
    secs: u64,
    nanos: u32,
}

impl Instant {
    /// Capture the current monotonic time.
    ///
    /// Returns `Duration::ZERO`-based instant if `clock_gettime` fails
    /// (should not happen on a working system).
    pub fn now() -> Self {
        unsafe {
            let mut ts: Timespec = core::mem::zeroed();
            let rc = clock_gettime(CLOCK_MONOTONIC, &mut ts);
            if rc != 0 || ts.tv_sec < 0 || ts.tv_nsec < 0 || ts.tv_nsec >= 1_000_000_000 {
                // clock_gettime failed or returned invalid values — return epoch as fallback
                return Self { secs: 0, nanos: 0 };
            }
            Self {
                secs: ts.tv_sec as u64,
                nanos: ts.tv_nsec as u32,
            }
        }
    }

    /// Duration elapsed since this instant.
    pub fn elapsed(&self) -> Duration {
        let now = Self::now();
        let now_nanos = now.secs * 1_000_000_000 + now.nanos as u64;
        let self_nanos = self.secs * 1_000_000_000 + self.nanos as u64;
        Duration::from_nanos(now_nanos.saturating_sub(self_nanos))
    }
}

impl core::ops::Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        let lhs_nanos = self.secs * 1_000_000_000 + self.nanos as u64;
        let rhs_nanos = rhs.secs * 1_000_000_000 + rhs.nanos as u64;
        Duration::from_nanos(lhs_nanos.saturating_sub(rhs_nanos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Duration constructors ----

    #[test]
    fn from_secs() {
        let d = Duration::from_secs(5);
        assert_eq!(d.as_secs(), 5);
        assert_eq!(d.subsec_nanos(), 0);
    }

    #[test]
    fn from_secs_zero() {
        let d = Duration::from_secs(0);
        assert_eq!(d, Duration::ZERO);
    }

    #[test]
    fn from_millis() {
        let d = Duration::from_millis(1_500);
        assert_eq!(d.as_secs(), 1);
        assert_eq!(d.subsec_nanos(), 500_000_000);
    }

    #[test]
    fn from_millis_exact_second() {
        let d = Duration::from_millis(3_000);
        assert_eq!(d.as_secs(), 3);
        assert_eq!(d.subsec_nanos(), 0);
    }

    #[test]
    fn from_millis_sub_second() {
        let d = Duration::from_millis(250);
        assert_eq!(d.as_secs(), 0);
        assert_eq!(d.subsec_nanos(), 250_000_000);
    }

    #[test]
    fn from_nanos() {
        let d = Duration::from_nanos(2_500_000_000);
        assert_eq!(d.as_secs(), 2);
        assert_eq!(d.subsec_nanos(), 500_000_000);
    }

    #[test]
    fn from_nanos_sub_second() {
        let d = Duration::from_nanos(999_999_999);
        assert_eq!(d.as_secs(), 0);
        assert_eq!(d.subsec_nanos(), 999_999_999);
    }

    #[test]
    fn from_nanos_exact_second() {
        let d = Duration::from_nanos(1_000_000_000);
        assert_eq!(d.as_secs(), 1);
        assert_eq!(d.subsec_nanos(), 0);
    }

    // ---- Duration conversions ----

    #[test]
    fn as_millis() {
        let d = Duration::from_millis(1_234);
        assert_eq!(d.as_millis(), 1_234);
    }

    #[test]
    fn as_millis_from_nanos() {
        // 1.5 seconds = 1500 ms
        let d = Duration::from_nanos(1_500_000_000);
        assert_eq!(d.as_millis(), 1_500);
    }

    #[test]
    fn as_nanos() {
        let d = Duration::from_secs(1);
        assert_eq!(d.as_nanos(), 1_000_000_000);
    }

    #[test]
    fn as_nanos_from_millis() {
        let d = Duration::from_millis(42);
        assert_eq!(d.as_nanos(), 42_000_000);
    }

    // ---- Duration::ZERO ----

    #[test]
    fn zero_constant() {
        assert_eq!(Duration::ZERO.as_secs(), 0);
        assert_eq!(Duration::ZERO.subsec_nanos(), 0);
        assert_eq!(Duration::ZERO.as_millis(), 0);
        assert_eq!(Duration::ZERO.as_nanos(), 0);
    }

    // ---- Duration arithmetic ----

    #[test]
    fn add_durations() {
        let a = Duration::from_millis(500);
        let b = Duration::from_millis(700);
        let c = a + b;
        assert_eq!(c.as_secs(), 1);
        assert_eq!(c.subsec_nanos(), 200_000_000);
    }

    #[test]
    fn add_durations_no_carry() {
        let a = Duration::from_millis(100);
        let b = Duration::from_millis(200);
        let c = a + b;
        assert_eq!(c.as_millis(), 300);
    }

    #[test]
    fn add_zero() {
        let a = Duration::from_secs(5);
        let c = a + Duration::ZERO;
        assert_eq!(c, a);
    }

    #[test]
    fn sub_durations() {
        let a = Duration::from_millis(1_500);
        let b = Duration::from_millis(700);
        let c = a - b;
        assert_eq!(c.as_millis(), 800);
    }

    #[test]
    fn sub_durations_exact() {
        let a = Duration::from_secs(3);
        let b = Duration::from_secs(1);
        let c = a - b;
        assert_eq!(c.as_secs(), 2);
        assert_eq!(c.subsec_nanos(), 0);
    }

    #[test]
    fn sub_saturates_at_zero() {
        let a = Duration::from_millis(100);
        let b = Duration::from_millis(500);
        let c = a - b;
        assert_eq!(c, Duration::ZERO);
    }

    #[test]
    fn sub_self_is_zero() {
        let a = Duration::from_millis(1_234);
        let c = a - a;
        assert_eq!(c, Duration::ZERO);
    }

    // ---- Duration ordering ----

    #[test]
    fn ordering() {
        let a = Duration::from_millis(100);
        let b = Duration::from_millis(200);
        assert!(a < b);
        assert!(b > a);
        assert!(a <= a);
        assert!(a >= a);
    }

    #[test]
    fn ordering_sub_nano() {
        let a = Duration::from_nanos(1);
        let b = Duration::from_nanos(2);
        assert!(a < b);
    }

    // ---- Roundtrip consistency ----

    #[test]
    fn millis_roundtrip() {
        for ms in [0, 1, 999, 1000, 1001, 59_999, 60_000, 123_456] {
            let d = Duration::from_millis(ms);
            assert_eq!(d.as_millis(), ms, "roundtrip failed for {}ms", ms);
        }
    }

    #[test]
    fn nanos_roundtrip() {
        for ns in [0, 1, 999_999_999, 1_000_000_000, 1_000_000_001, 5_500_000_000] {
            let d = Duration::from_nanos(ns);
            assert_eq!(d.as_nanos(), ns, "roundtrip failed for {}ns", ns);
        }
    }

    // ---- Large values ----

    #[test]
    fn large_secs() {
        let d = Duration::from_secs(u64::MAX);
        assert_eq!(d.as_secs(), u64::MAX);
        assert_eq!(d.subsec_nanos(), 0);
    }

    #[test]
    fn add_nanos_with_carry() {
        let a = Duration::from_nanos(999_999_999);
        let b = Duration::from_nanos(1);
        let c = a + b;
        assert_eq!(c.as_secs(), 1);
        assert_eq!(c.subsec_nanos(), 0);
    }

    // ---- Sprint 3A: additional Duration tests ----

    #[test]
    fn duration_from_millis_zero() {
        assert_eq!(Duration::from_millis(0), Duration::ZERO);
    }

    #[test]
    fn duration_from_nanos_zero() {
        assert_eq!(Duration::from_nanos(0), Duration::ZERO);
    }

    #[test]
    fn duration_add_commutative() {
        let a = Duration::from_millis(300);
        let b = Duration::from_millis(700);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn duration_sub_borrow_from_secs() {
        let a = Duration { secs: 2, nanos: 100 };
        let b = Duration { secs: 1, nanos: 999_999_999 };
        let c = a - b;
        // 2_000_000_100 - 1_999_999_999 = 101 nanos
        assert_eq!(c.as_nanos(), 101);
    }

    #[test]
    fn duration_debug_formatting() {
        extern crate alloc;
        let d = Duration::from_millis(1_500);
        let dbg = alloc::format!("{:?}", d);
        assert!(!dbg.is_empty());
    }
}

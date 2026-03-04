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
    pub const ZERO: Duration = Duration { secs: 0, nanos: 0 };

    #[inline]
    pub fn from_secs(secs: u64) -> Self {
        Self { secs, nanos: 0 }
    }

    #[inline]
    pub fn from_millis(millis: u64) -> Self {
        Self {
            secs: millis / 1_000,
            nanos: ((millis % 1_000) * 1_000_000) as u32,
        }
    }

    #[inline]
    pub fn from_nanos(nanos: u64) -> Self {
        Self {
            secs: nanos / 1_000_000_000,
            nanos: (nanos % 1_000_000_000) as u32,
        }
    }

    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.secs
    }

    #[inline]
    pub fn as_millis(&self) -> u64 {
        self.secs * 1_000 + self.nanos as u64 / 1_000_000
    }

    #[inline]
    pub fn as_nanos(&self) -> u64 {
        self.secs * 1_000_000_000 + self.nanos as u64
    }

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
    pub fn now() -> Self {
        unsafe {
            let mut ts: Timespec = core::mem::zeroed();
            clock_gettime(CLOCK_MONOTONIC, &mut ts);
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

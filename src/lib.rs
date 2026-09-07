//! Fast seconds → hour/minute/second conversion.
//!
//! The public [`hms`] entry point is Joffe's V3 64-bit algorithm
//! ("base-64 clock trick"): <https://www.benjoffe.com/fast-time-of-day>
//!
//! Valid for `seconds` in `0..=`[`MAX_SECONDS`] (max result `626:59:58`).
//!
//! Alternate implementations live in [`algorithms`] for differential tests.

pub mod algorithms;

pub use algorithms::{MAX_SECONDS_NERI, MAX_SECONDS_V2, MAX_SECONDS_V3, SECONDS_PER_DAY};

/// Inclusive maximum input for [`hms`] (V3 64-bit).
pub const MAX_SECONDS: u32 = MAX_SECONDS_V3;

/// Hour, minute, and second decomposed from a seconds-of-day timestamp.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Hms {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Hms {
    /// Reconstruct total seconds: `hour * 3600 + minute * 60 + second`.
    #[inline]
    #[must_use]
    pub const fn to_seconds(self) -> u64 {
        self.hour as u64 * 3600 + self.minute as u64 * 60 + self.second as u64
    }
}

/// Convert `seconds` in `[0, MAX_SECONDS]` to hour, minute, and second.
#[inline(always)]
#[must_use]
pub fn hms(seconds: u32) -> Hms {
    algorithms::v3_64(seconds)
}

#[cfg(test)]
mod assurance;

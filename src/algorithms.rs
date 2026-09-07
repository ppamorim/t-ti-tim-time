//! Independent time-of-day decompositions used by the library and by tests.
//!
//! `reference` is deliberately the naive `/` `%` form. The others follow
//! Joffe and Neri so they can be differenced against that oracle.

use crate::Hms;

/// `(((1 << 32) / 60) + 1) << 32`
pub const M_MUL: u64 = ((1u64 << 32) / 60 + 1) << 32;
/// `(((1 << 32) / 3600) + 1) << 32`
pub const H_MUL: u64 = ((1u64 << 32) / 3600 + 1) << 32;

/// 32-bit reciprocal of 60: `(1 << 32) / 60 + 1`
pub const RECIP_60: u64 = (1u64 << 32) / 60 + 1;
/// 32-bit reciprocal of 3600: `(1 << 32) / 3600 + 1`
pub const RECIP_3600: u64 = (1u64 << 32) / 3600 + 1;

/// Inclusive maximum `seconds` for V1-fixed-point, V3, and V3-64.
pub const MAX_SECONDS_V3: u32 = 2_257_198;
/// Inclusive maximum `seconds` for V2 and V2-64.
pub const MAX_SECONDS_V2: u32 = 2_255_818;
/// Inclusive maximum `seconds` for Neri 2020.
pub const MAX_SECONDS_NERI: u32 = 97_612_918;

/// Seconds in a civil day (Unix time; no leap second).
pub const SECONDS_PER_DAY: u32 = 86_400;

/// Naive division/modulo. Valid for every `u32`.
#[inline]
#[must_use]
pub fn reference(time: u32) -> Hms {
    Hms {
        hour: time / 3600,
        minute: (time / 60) % 60,
        second: time % 60,
    }
}

/// Neri 2020 high/low-bit remainder. Valid for `0..=MAX_SECONDS_NERI`.
#[inline]
#[must_use]
pub fn neri(time: u32) -> Hms {
    let p1 = u64::from(time) * RECIP_60;
    let tmin = (p1 >> 32) as u32;
    let second = (p1 as u32) / RECIP_60 as u32;
    let p2 = u64::from(tmin) * RECIP_60;
    let hour = (p2 >> 32) as u32;
    let minute = (p2 as u32) / RECIP_60 as u32;
    Hms {
        hour,
        minute,
        second,
    }
}

/// Joffe V1: parallel `/ 60` and `/ 3600`. Full `u32` range.
#[inline]
#[must_use]
pub fn v1(time: u32) -> Hms {
    let tmin = time / 60;
    let hour = time / 3600;
    let second = time - tmin * 60;
    let minute = tmin - hour * 60;
    Hms {
        hour,
        minute,
        second,
    }
}

/// Joffe V1 with 32-bit fixed-point quotients. Valid for `0..=MAX_SECONDS_V3`.
#[inline]
#[must_use]
pub fn v1_fp(time: u32) -> Hms {
    let tmin = ((u64::from(time) * RECIP_60) >> 32) as u32;
    let hour = ((u64::from(time) * RECIP_3600) >> 32) as u32;
    let second = time - tmin * 60;
    let minute = tmin - hour * 60;
    Hms {
        hour,
        minute,
        second,
    }
}

/// Joffe V2: fixed-point high/low bits. Valid for `0..=MAX_SECONDS_V2`.
#[inline]
#[must_use]
pub fn v2(time: u32) -> Hms {
    let hprd = u64::from(time) * RECIP_3600;
    let mlow = time.wrapping_mul(RECIP_60 as u32);
    let hour = (hprd >> 32) as u32;
    let hlow = hprd as u32;
    let minute = ((u64::from(hlow) * 60) >> 32) as u32;
    let second = ((u64::from(mlow) * 60) >> 32) as u32;
    Hms {
        hour,
        minute,
        second,
    }
}

/// Joffe V2 widened to 64×64→128. Valid for `0..=MAX_SECONDS_V2`.
#[inline]
#[must_use]
pub fn v2_64(time: u32) -> Hms {
    let hprd = u128::from(time) * u128::from(H_MUL);
    let mlow = u64::from(time).wrapping_mul(M_MUL);
    let hour = (hprd >> 64) as u32;
    let hlow = hprd as u64;
    let minute = ((u128::from(hlow) * 60) >> 64) as u32;
    let second = ((u128::from(mlow) * 60) >> 64) as u32;
    Hms {
        hour,
        minute,
        second,
    }
}

/// Joffe V3: base-64 clock trick. Valid for `0..=MAX_SECONDS_V3`.
#[inline]
#[must_use]
pub fn v3(time: u32) -> Hms {
    let tmin = ((u64::from(time) * RECIP_60) >> 32) as u32;
    let hour = ((u64::from(time) * RECIP_3600) >> 32) as u32;
    let second = time.wrapping_add(tmin << 2) & 63;
    let minute = tmin.wrapping_add(hour << 2) & 63;
    Hms {
        hour,
        minute,
        second,
    }
}

/// Joffe V3 using 64-bit products. Valid for `0..=MAX_SECONDS_V3`.
#[inline(always)]
#[must_use]
pub fn v3_64(time: u32) -> Hms {
    // M_MUL has 32 trailing zero bits, so cancel that shift:
    // (time * (reciprocal << 32)) >> 64 == (time * reciprocal) >> 32.
    let tmin = ((u64::from(time) * RECIP_60) >> 32) as u32;

    // AArch64 lowers this to UMULH, avoiding a dependent right shift.
    // Keep the minute product narrow: widening both products can trigger
    // slower SIMD code for the scalar Hms result (see the benchmarks).
    #[cfg(target_arch = "aarch64")]
    let hour = ((u128::from(time) * u128::from(H_MUL)) >> 64) as u32;
    #[cfg(not(target_arch = "aarch64"))]
    let hour = ((u64::from(time) * RECIP_3600) >> 32) as u32;

    // Only the low six bits matter; keep the clock arithmetic in u32.
    let second = time.wrapping_add(tmin << 2) & 63;
    let minute = tmin.wrapping_add(hour << 2) & 63;
    Hms {
        hour,
        minute,
        second,
    }
}

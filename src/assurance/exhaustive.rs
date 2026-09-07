//! Exhaustive reduced-domain tests. The full `u32` sweep is ignored (nightly).

use super::support::assert_all_in_range;
use crate::algorithms::{reference, v1, v3_64, MAX_SECONDS_V3};
use crate::{hms, MAX_SECONDS};

#[test]
fn exhaustive_u8_domain() {
    for time in 0..=u32::from(u8::MAX) {
        assert_all_in_range(time);
    }
}

#[test]
fn exhaustive_u16_domain() {
    for time in 0..=u32::from(u16::MAX) {
        assert_eq!(hms(time), reference(time), "time={time}");
        assert_eq!(v1(time), reference(time), "v1 time={time}");
    }
}

/// 24-bit values that still sit inside the V3 validity range.
#[test]
fn exhaustive_u24_intersecting_v3_range() {
    let last = (1u32 << 24).min(MAX_SECONDS);
    for time in 0..=last {
        assert_eq!(hms(time), reference(time), "time={time}");
    }
}

/// Every input in the documented V3 range. Skipped under cargo-mutants.
#[cfg(not(mutants))]
#[test]
fn exhaustive_v3_range_against_reference() {
    for time in 0..=MAX_SECONDS {
        assert_eq!(hms(time), reference(time), "time={time}");
        assert_eq!(v3_64(time), reference(time), "v3_64 time={time}");
    }
}

#[test]
fn v3_breaks_at_the_documented_limit() {
    assert_eq!(hms(MAX_SECONDS_V3), reference(MAX_SECONDS_V3));
    let first_bad =
        (MAX_SECONDS_V3 + 1..=MAX_SECONDS_V3 + 64).find(|&time| hms(time) != reference(time));
    assert_eq!(
        first_bad,
        Some(MAX_SECONDS_V3 + 1),
        "V3 should fail immediately after the documented maximum"
    );
}

/// 4.29 billion inputs. Run with `cargo test --release -- --ignored`.
#[test]
#[ignore = "nightly/release verification: exhaustive u32 v1 vs reference"]
fn exhaustive_u32_v1_matches_reference() {
    for time in 0..=u32::MAX {
        if v1(time) != reference(time) {
            panic!("v1 diverged from reference at {time}");
        }
    }
}

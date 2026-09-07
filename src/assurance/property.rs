//! Property tests over generated `u64` (kept when they fit the valid range).

use crate::algorithms::{neri, reference, v1, v2, MAX_SECONDS_V2};
use crate::{hms, Hms, MAX_SECONDS};
use proptest::prelude::*;

fn cases() -> u32 {
    if cfg!(mutants) {
        32
    } else {
        10_000
    }
}

fn contract_holds(time: u32, got: Hms) -> Result<(), TestCaseError> {
    prop_assert!(got.second < 60);
    prop_assert!(got.minute < 60);
    prop_assert_eq!(got.to_seconds(), u64::from(time));
    prop_assert_eq!(got, reference(time));
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(cases()))]

    #[test]
    fn hms_matches_reference_for_random_u64(
        time in prop_oneof![
            0u64..=u64::from(MAX_SECONDS),
            any::<u64>(),
        ]
    ) {
        if time > u64::from(MAX_SECONDS) {
            return Ok(());
        }
        let time = time as u32;
        let got = hms(time);
        contract_holds(time, got)?;
        prop_assert_eq!(v1(time), got);
        prop_assert_eq!(neri(time), got);
        if time <= MAX_SECONDS_V2 {
            prop_assert_eq!(v2(time), got);
        }
    }

    #[test]
    fn hms_matches_reference_in_closed_v3_range(time in 0u32..=MAX_SECONDS) {
        contract_holds(time, hms(time))?;
    }

    #[test]
    fn v1_matches_reference_for_any_u32(time in any::<u32>()) {
        prop_assert_eq!(v1(time), reference(time));
        prop_assert_eq!(v1(time).to_seconds(), u64::from(time));
    }

    #[test]
    fn day_shift_preserves_clock_fields(time in 0u32..=(MAX_SECONDS - 86_400)) {
        let before = hms(time);
        let after = hms(time + 86_400);
        prop_assert_eq!(after.hour, before.hour + 24);
        prop_assert_eq!(after.minute, before.minute);
        prop_assert_eq!(after.second, before.second);
    }
}

//! Multiple independent implementations must agree with naive `/` `%`.

use super::support::{
    assert_all_in_range, assert_matches_reference, datealgo_hms, edge_corpus, lcg_times,
};
use crate::algorithms::{
    neri, reference, v1, v1_fp, v2, v2_64, v3, MAX_SECONDS_NERI, MAX_SECONDS_V2, MAX_SECONDS_V3,
};
use crate::{hms, MAX_SECONDS};

#[test]
fn all_algorithms_match_reference_on_corpus() {
    for time in edge_corpus() {
        assert_all_in_range(time);
    }
}

#[test]
fn all_algorithms_match_reference_on_random_u32_in_range() {
    for time in lcg_times(50_000, MAX_SECONDS) {
        assert_all_in_range(time);
    }
}

#[test]
fn generated_u64_values_match_when_inside_v3_range() {
    let mut state = 0xC0FFEE_u64;
    for _ in 0..20_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        if state > u64::from(MAX_SECONDS) {
            continue;
        }
        let time = state as u32;
        assert_all_in_range(time);
    }
}

#[test]
fn v1_matches_reference_across_full_u32_sample() {
    for time in [0, 1, u32::MAX / 2, u32::MAX - 1, u32::MAX] {
        assert_eq!(v1(time), reference(time));
        assert_matches_reference(time, v1(time));
    }
    for time in lcg_times(20_000, u32::MAX) {
        assert_eq!(v1(time), reference(time));
    }
}

#[test]
fn neri_matches_reference_up_to_its_range() {
    for time in lcg_times(20_000, MAX_SECONDS_NERI) {
        assert_eq!(neri(time), reference(time));
    }
    assert_eq!(neri(MAX_SECONDS_NERI), reference(MAX_SECONDS_NERI));
}

#[test]
fn v2_family_matches_reference_up_to_its_range() {
    for time in lcg_times(20_000, MAX_SECONDS_V2) {
        assert_eq!(v2(time), reference(time));
        assert_eq!(v2_64(time), reference(time));
    }
}

#[test]
fn v3_family_matches_reference_up_to_its_range() {
    for time in lcg_times(20_000, MAX_SECONDS_V3) {
        assert_eq!(v1_fp(time), reference(time));
        assert_eq!(v3(time), reference(time));
        assert_eq!(hms(time), reference(time));
    }
}

#[test]
fn datealgo_matches_reference_for_full_u32_sample() {
    for time in lcg_times(10_000, u32::MAX) {
        assert_eq!(datealgo_hms(time), reference(time));
    }
}

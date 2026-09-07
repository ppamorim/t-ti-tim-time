//! Permanent edge-value corpus, including power-of-two neighborhoods.

use super::support::{assert_all_in_range, edge_corpus, power_of_two_neighborhood};
use crate::algorithms::{reference, v1};
use crate::{hms, MAX_SECONDS};

const FIXED_CORPUS: &[u32] = &[
    0,
    1,
    2,
    59,
    60,
    61,
    3599,
    3600,
    3601,
    86_399,
    86_400,
    86_401,
    (1 << 8) - 1,
    1 << 8,
    (1 << 8) + 1,
    (1 << 16) - 1,
    1 << 16,
    (1 << 16) + 1,
    (1 << 24) - 1,
    1 << 24,
    (1 << 24) + 1,
];

#[test]
fn fixed_edge_corpus_matches_reference() {
    for &time in FIXED_CORPUS {
        if time <= MAX_SECONDS {
            assert_all_in_range(time);
        } else {
            assert_eq!(v1(time), reference(time), "v1 time={time}");
        }
    }
}

#[test]
fn power_of_two_neighborhoods_in_v3_range() {
    for time in power_of_two_neighborhood(MAX_SECONDS) {
        assert_all_in_range(time);
    }
}

#[test]
fn combined_edge_corpus_matches_reference() {
    let corpus = edge_corpus();
    assert!(corpus.contains(&0));
    assert!(corpus.contains(&86_399));
    assert!(corpus.contains(&MAX_SECONDS));
    for time in corpus {
        assert_eq!(hms(time), reference(time));
    }
}

#[test]
fn wide_width_sentinels_do_not_panic() {
    for time in [
        u8::MAX as u32,
        u16::MAX as u32,
        (1u32 << 24) - 1,
        u32::MAX - 1,
        u32::MAX,
    ] {
        let _ = hms(time);
        assert_eq!(v1(time), reference(time));
    }
}

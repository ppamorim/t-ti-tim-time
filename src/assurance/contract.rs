//! Formal specification: each field is in range and the triple reconstructs.

use super::support::{assert_invariants, edge_corpus};
use crate::algorithms::reference;
use crate::{hms, MAX_SECONDS};

#[test]
fn specification_holds_on_corpus() {
    for time in edge_corpus() {
        let got = hms(time);
        assert!(got.second < 60, "second invariant time={time} {got:?}");
        assert!(got.minute < 60, "minute invariant time={time} {got:?}");
        assert_eq!(
            got.hour * 3600 + got.minute * 60 + got.second,
            time,
            "hour*3600 + minute*60 + second == input, time={time} {got:?}"
        );
        assert_eq!(got.to_seconds(), u64::from(time));
        assert_invariants(time, got);
    }
}

#[test]
fn specification_holds_for_every_second_of_a_day() {
    for time in 0..=86_399 {
        assert_invariants(time, hms(time));
        assert_eq!(hms(time), reference(time));
    }
}

#[test]
fn documented_range_endpoint_satisfies_contract() {
    assert_invariants(MAX_SECONDS, hms(MAX_SECONDS));
    assert_eq!(hms(MAX_SECONDS), reference(MAX_SECONDS));
}

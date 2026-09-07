//! Values immediately around every 60- and 3600-second discontinuity.

use super::support::{around, assert_all_in_range, neighborhoods_of_multiples};
use crate::algorithms::{MAX_SECONDS_V2, MAX_SECONDS_V3};
use crate::{hms, MAX_SECONDS};

#[test]
fn neighborhoods_of_every_minute() {
    for time in neighborhoods_of_multiples(60, MAX_SECONDS) {
        assert_all_in_range(time);
        assert_eq!(hms(time).second, time % 60);
    }
}

#[test]
fn neighborhoods_of_every_hour() {
    for time in neighborhoods_of_multiples(3600, MAX_SECONDS) {
        assert_all_in_range(time);
        assert_eq!(hms(time).minute, (time / 60) % 60);
        assert_eq!(hms(time).hour, time / 3600);
    }
}

#[test]
fn neighborhoods_of_algorithm_range_limits() {
    for max in [MAX_SECONDS_V2, MAX_SECONDS_V3, MAX_SECONDS] {
        for time in around(u64::from(max), MAX_SECONDS) {
            assert_all_in_range(time);
        }
    }
}

#[test]
fn neighborhoods_of_u32_limits_do_not_panic() {
    for time in around(u64::from(u32::MAX), u32::MAX) {
        let _ = hms(time);
        let _ = crate::algorithms::v1(time);
        let _ = crate::algorithms::reference(time);
    }
}

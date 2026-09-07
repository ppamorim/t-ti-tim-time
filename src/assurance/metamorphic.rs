//! Relationships between inputs, not individual expected triples.

use super::support::lcg_times;
use crate::algorithms::{reference, SECONDS_PER_DAY};
use crate::{hms, MAX_SECONDS};

#[test]
fn adding_a_day_shifts_hour_by_24() {
    let limit = MAX_SECONDS - SECONDS_PER_DAY;
    for time in lcg_times(8_000, limit) {
        let before = hms(time);
        let after = hms(time + SECONDS_PER_DAY);
        assert_eq!(after.hour, before.hour + 24, "time={time}");
        assert_eq!(after.minute, before.minute, "time={time}");
        assert_eq!(after.second, before.second, "time={time}");
        assert_eq!(after, reference(time + SECONDS_PER_DAY));
    }
}

#[test]
fn minute_boundaries_reset_the_second() {
    let last_k = MAX_SECONDS / 60;
    for k in 0..=last_k {
        let base = k * 60;
        assert_eq!(hms(base).second, 0, "60*{k}");
        if base + 59 <= MAX_SECONDS {
            assert_eq!(hms(base + 59).second, 59, "60*{k}+59");
        }
        if base + 60 <= MAX_SECONDS {
            assert_eq!(hms(base + 60).second, 0, "60*{k}+60");
        }
    }
}

#[test]
fn hour_boundaries_reset_the_minute() {
    let last_k = MAX_SECONDS / 3600;
    for k in 1..=last_k {
        let hour = k * 3600;
        assert_eq!(hms(hour - 1).minute, 59, "3600*{k}-1");
        assert_eq!(hms(hour).minute, 0, "3600*{k}");
        assert_eq!(hms(hour).second, 0, "3600*{k} second");
        if hour < MAX_SECONDS {
            assert_eq!(hms(hour + 1).minute, 0, "3600*{k}+1");
            assert_eq!(hms(hour + 1).second, 1, "3600*{k}+1 second");
        }
    }
}

#[test]
fn increment_walks_hms_like_a_clock() {
    for time in lcg_times(12_000, MAX_SECONDS.saturating_sub(1)) {
        let now = hms(time);
        let next = hms(time + 1);
        if now.second < 59 {
            assert_eq!(next.second, now.second + 1, "time={time}");
            assert_eq!(next.minute, now.minute, "time={time}");
            assert_eq!(next.hour, now.hour, "time={time}");
        } else if now.minute < 59 {
            assert_eq!(next.second, 0, "time={time}");
            assert_eq!(next.minute, now.minute + 1, "time={time}");
            assert_eq!(next.hour, now.hour, "time={time}");
        } else {
            assert_eq!(next.second, 0, "time={time}");
            assert_eq!(next.minute, 0, "time={time}");
            assert_eq!(next.hour, now.hour + 1, "time={time}");
        }
    }
}

#[test]
fn plus_sixty_seconds_increments_minute_or_hour() {
    let limit = MAX_SECONDS - 60;
    for time in lcg_times(8_000, limit) {
        let before = hms(time);
        let after = hms(time + 60);
        assert_eq!(after.second, before.second, "time={time}");
        if before.minute < 59 {
            assert_eq!(after.minute, before.minute + 1, "time={time}");
            assert_eq!(after.hour, before.hour, "time={time}");
        } else {
            assert_eq!(after.minute, 0, "time={time}");
            assert_eq!(after.hour, before.hour + 1, "time={time}");
        }
    }
}

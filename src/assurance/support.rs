use crate::algorithms::{
    neri, reference, v1, v1_fp, v2, v2_64, v3, v3_64, MAX_SECONDS_NERI, MAX_SECONDS_V2,
    MAX_SECONDS_V3,
};
use crate::{hms, Hms, MAX_SECONDS};

pub fn datealgo_hms(time: u32) -> Hms {
    let (days, hour, minute, second) = datealgo::secs_to_dhms(i64::from(time));
    debug_assert!(days >= 0);
    Hms {
        hour: days as u32 * 24 + u32::from(hour),
        minute: u32::from(minute),
        second: u32::from(second),
    }
}

pub fn assert_invariants(time: u32, got: Hms) {
    assert!(got.minute < 60, "minute out of range for {time}: {got:?}");
    assert!(got.second < 60, "second out of range for {time}: {got:?}");
    assert_eq!(
        got.to_seconds(),
        u64::from(time),
        "reconstruction failed for {time}: {got:?}"
    );
}

pub fn assert_matches_reference(time: u32, got: Hms) {
    let expected = reference(time);
    assert_eq!(got, expected, "time={time}");
    assert_invariants(time, got);
}

pub fn assert_all_in_range(time: u32) {
    let expected = reference(time);
    assert_eq!(v1(time), expected, "v1 time={time}");
    assert_eq!(datealgo_hms(time), expected, "datealgo time={time}");

    if time <= MAX_SECONDS_NERI {
        assert_eq!(neri(time), expected, "neri time={time}");
    }
    if time <= MAX_SECONDS_V3 {
        assert_eq!(v1_fp(time), expected, "v1_fp time={time}");
        assert_eq!(v3(time), expected, "v3 time={time}");
        assert_eq!(v3_64(time), expected, "v3_64 time={time}");
        assert_eq!(hms(time), expected, "hms time={time}");
        assert_invariants(time, hms(time));
    }
    if time <= MAX_SECONDS_V2 {
        assert_eq!(v2(time), expected, "v2 time={time}");
        assert_eq!(v2_64(time), expected, "v2_64 time={time}");
    }
}

pub fn around(base: u64, max: u32) -> impl Iterator<Item = u32> {
    [-2i64, -1, 0, 1, 2].into_iter().filter_map(move |delta| {
        let value = base as i128 + i128::from(delta);
        (value >= 0 && value <= i128::from(max)).then_some(value as u32)
    })
}

pub fn neighborhoods_of_multiples(divisor: u32, max: u32) -> Vec<u32> {
    let last = max / divisor;
    let mut out = Vec::with_capacity((last as usize + 1) * 5);
    for n in 0..=last {
        out.extend(around(u64::from(n) * u64::from(divisor), max));
    }
    out
}

pub fn power_of_two_neighborhood(max: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for shift in 0..=32u32 {
        let pow = if shift == 32 {
            1u64 << 32
        } else {
            1u64 << shift
        };
        out.extend(around(pow, max));
        if pow > 0 {
            out.extend(around(pow - 1, max));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

pub fn edge_corpus() -> Vec<u32> {
    let mut values = vec![
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
        u8::MAX as u32,
        u16::MAX as u32,
        (1u32 << 24) - 1,
        MAX_SECONDS_V2,
        MAX_SECONDS_V2.saturating_sub(1),
        MAX_SECONDS_V3,
        MAX_SECONDS_V3.saturating_sub(1),
        MAX_SECONDS,
    ];
    values.extend(power_of_two_neighborhood(MAX_SECONDS));
    values.sort_unstable();
    values.dedup();
    values.retain(|&time| time <= MAX_SECONDS);
    values
}

pub fn lcg_times(count: usize, max: u32) -> Vec<u32> {
    let mut state = 0xA5A5_1234_u64;
    let span = u64::from(max) + 1;
    (0..count)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state % span) as u32
        })
        .collect()
}

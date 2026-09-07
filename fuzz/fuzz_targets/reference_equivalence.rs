#![no_main]

use t_ti_tim_time::algorithms::{neri, reference, v1, v2};
use t_ti_tim_time::{hms, MAX_SECONDS};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|time: u64| {
    if time > u64::from(MAX_SECONDS) {
        return;
    }
    let time = time as u32;
    let expected = reference(time);
    assert_eq!(hms(time), expected);
    assert_eq!(v1(time), expected);
    assert_eq!(v2(time), expected);
    assert_eq!(neri(time), expected);
});

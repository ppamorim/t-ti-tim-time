#![no_main]

use t_ti_tim_time::{hms, MAX_SECONDS};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|time: u64| {
    if time > u64::from(MAX_SECONDS) {
        return;
    }
    let time = time as u32;
    let got = hms(time);
    assert!(got.second < 60);
    assert!(got.minute < 60);
    assert_eq!(
        got.hour as u64 * 3600 + got.minute as u64 * 60 + got.second as u64,
        u64::from(time)
    );
});

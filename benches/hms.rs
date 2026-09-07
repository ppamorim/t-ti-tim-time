use criterion::{black_box, criterion_group, criterion_main, Criterion};
use t_ti_tim_time::algorithms::{neri, reference, v1, v2, v2_64, v3_64};
use t_ti_tim_time::{hms, Hms, MAX_SECONDS};

fn lcg_times(count: usize) -> Vec<u32> {
    let mut state = 0xA5A5_1234_u64;
    let span = u64::from(MAX_SECONDS) + 1;
    (0..count)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state % span) as u32
        })
        .collect()
}

fn sequential_times(count: usize) -> Vec<u32> {
    (0..count as u32).map(|i| i % (MAX_SECONDS + 1)).collect()
}

fn boundary_times() -> Vec<u32> {
    let mut out = Vec::new();
    let last = MAX_SECONDS / 60;
    for n in 0..=last {
        let base = n * 60;
        for delta in [0u32, 1, 59] {
            let time = base.saturating_add(delta);
            if time <= MAX_SECONDS {
                out.push(time);
            }
        }
    }
    out
}

fn fold(times: &[u32], f: fn(u32) -> Hms) -> u32 {
    let mut acc = 0u32;
    for &time in times {
        let got = f(black_box(time));
        acc ^= got.hour ^ got.minute ^ got.second;
    }
    acc
}

fn latency(times: &[u32], f: fn(u32) -> Hms) -> u32 {
    let mut acc = 0u32;
    for &time in times {
        let got = f(black_box(time ^ acc));
        acc = got.hour ^ got.minute ^ got.second;
    }
    acc
}

fn bench_set(c: &mut Criterion, label: &str, times: &[u32]) {
    let mut group = c.benchmark_group(label);
    group.bench_function("reference", |b| b.iter(|| fold(times, reference)));
    group.bench_function("neri", |b| b.iter(|| fold(times, neri)));
    group.bench_function("v1", |b| b.iter(|| fold(times, v1)));
    group.bench_function("v2", |b| b.iter(|| fold(times, v2)));
    group.bench_function("v2_64", |b| b.iter(|| fold(times, v2_64)));
    group.bench_function("v3_64", |b| b.iter(|| fold(times, v3_64)));
    group.bench_function("v3_narrow_baseline", |b| {
        b.iter(|| fold(times, v3_narrow_baseline))
    });
    group.bench_function("v3_128_baseline", |b| {
        b.iter(|| fold(times, v3_128_baseline))
    });
    group.bench_function("hms", |b| b.iter(|| fold(times, hms)));
    group.finish();
}

// Previous implementation, retained for a same-binary comparison.
#[inline(always)]
fn v3_narrow_baseline(time: u32) -> Hms {
    use t_ti_tim_time::algorithms::{RECIP_3600, RECIP_60};

    let tmin = ((u64::from(time) * RECIP_60) >> 32) as u32;
    let hour = ((u64::from(time) * RECIP_3600) >> 32) as u32;
    let second = time.wrapping_add(tmin << 2) & 63;
    let minute = tmin.wrapping_add(hour << 2) & 63;
    Hms {
        hour,
        minute,
        second,
    }
}

// Original wide arithmetic, retained to measure the effect of narrowing products.
#[inline(always)]
fn v3_128_baseline(time: u32) -> Hms {
    use t_ti_tim_time::algorithms::{H_MUL, M_MUL};

    let tmin = ((u128::from(time) * u128::from(M_MUL)) >> 64) as u64;
    let hour = ((u128::from(time) * u128::from(H_MUL)) >> 64) as u32;
    let second = (u64::from(time).wrapping_add(tmin << 2) as u32) & 63;
    let minute = (tmin.wrapping_add(u64::from(hour) << 2) as u32) & 63;
    Hms {
        hour,
        minute,
        second,
    }
}

fn benches(c: &mut Criterion) {
    let random = lcg_times(16_384);
    let sequential = sequential_times(16_384);
    let boundary = boundary_times();
    let constant = vec![43_200; 16_384];

    bench_set(c, "throughput/random", &random);
    bench_set(c, "throughput/sequential", &sequential);
    bench_set(c, "throughput/boundary", &boundary);
    bench_set(c, "throughput/constant", &constant);

    let mut latency_group = c.benchmark_group("latency/random");
    latency_group.bench_function("reference", |b| b.iter(|| latency(&random, reference)));
    latency_group.bench_function("v1", |b| b.iter(|| latency(&random, v1)));
    latency_group.bench_function("v2", |b| b.iter(|| latency(&random, v2)));
    latency_group.bench_function("v3_narrow_baseline", |b| {
        b.iter(|| latency(&random, v3_narrow_baseline))
    });
    latency_group.bench_function("v3_128_baseline", |b| {
        b.iter(|| latency(&random, v3_128_baseline))
    });
    latency_group.bench_function("hms", |b| b.iter(|| latency(&random, hms)));
    latency_group.finish();

    // Keep the function pointer opaque to measure callers that cannot inline hms.
    let mut indirect = c.benchmark_group("indirect/random");
    for (name, f) in [
        ("v3_narrow_baseline", v3_narrow_baseline as fn(u32) -> Hms),
        ("v3_128_baseline", v3_128_baseline as fn(u32) -> Hms),
        ("hms", hms as fn(u32) -> Hms),
    ] {
        indirect.bench_function(format!("throughput/{name}"), |b| {
            b.iter(|| fold(&random, black_box(f)))
        });
        indirect.bench_function(format!("latency/{name}"), |b| {
            b.iter(|| latency(&random, black_box(f)))
        });
    }
    indirect.finish();
}

criterion_group!(hms_benches, benches);
criterion_main!(hms_benches);

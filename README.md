# t-ti-tim-time

Rust library that converts a seconds-of-day timestamp into hour, minute, and second.

The public `hms` entry point is Ben Joffe's V3 64-bit algorithm (the base-64 clock trick):

<https://www.benjoffe.com/fast-time-of-day>

Valid for `seconds` in `0..=2_257_198` (`MAX_SECONDS`). That covers a full civil day (`0..=86_399`) and continues up to `626:59:58`. Unix time only — no leap seconds.

Alternate implementations (naive `/` `%`, Neri 2020, Joffe V1/V2/V3) live in `t_ti_tim_time::algorithms` for comparison and tests.

## Use

Add the crate as a path dependency:

```toml
[dependencies]
t-ti-tim-time = { path = "path/to/crate" }
```

```rust
use t_ti_tim_time::hms;

let t = hms(3661);
assert_eq!((t.hour, t.minute, t.second), (1, 1, 1));
assert_eq!(t.to_seconds(), 3661);
```

## Test

From this directory:

```bash
# library unit tests (includes the exhaustive V3 sweep)
cargo test --lib

# release
cargo test --lib --release

# nightly 4.29B V1 vs reference sweep
cargo test --lib --release -- --ignored
```

### Assurance script

`scripts/assurance.sh` reruns the library tests across compiler opt levels:

- debug (`opt-level = 0`)
- `test-o1`, `test-o2`, `test-o3`
- release (`opt-level = 3`)

```bash
./scripts/assurance.sh
```

AddressSanitizer and UndefinedBehaviorSanitizer (nightly):

```bash
ASSURANCE_SANITIZERS=1 ./scripts/assurance.sh
```

### Mutation, fuzz, benches

```bash
# cargo-mutants: catch deliberate breaks in the arithmetic
cargo mutants --file src/lib.rs --file src/algorithms.rs

# cargo-fuzz targets
cargo +nightly fuzz run reference_equivalence
cargo +nightly fuzz run invariants

# criterion (random / sequential / boundary / constant, plus latency)
cargo bench
```

## License

MIT. See [LICENSE](LICENSE).

#!/usr/bin/env bash
# Compiler-opt and sanitizer differential runs for the time-of-day crate.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

run_tests() {
  local label="$1"
  shift
  echo "==> $label"
  cargo test --offline --lib "$@"
}

run_tests "debug / opt-level 0" 
run_tests "release / opt-level 3" --release
run_tests "test-o1" --profile test-o1
run_tests "test-o2" --profile test-o2
run_tests "test-o3" --profile test-o3

if [[ "${ASSURANCE_SANITIZERS:-}" == "1" ]]; then
  TARGET="$(rustc -vV | awk '/^host:/{print $2}')"
  echo "==> AddressSanitizer ($TARGET)"
  RUSTFLAGS="-Z sanitizer=address" cargo +nightly test --offline --lib --target "$TARGET"
  echo "==> UndefinedBehaviorSanitizer ($TARGET)"
  RUSTFLAGS="-Z sanitizer=undefined" cargo +nightly test --offline --lib --target "$TARGET"
fi

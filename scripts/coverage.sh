#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report nextest --workspace
NEXTEST_PROFILE=db scripts/with-db.sh cargo llvm-cov --no-report nextest --workspace
# ponytail: one cell, so ignoring kernel and app makes the total the loads floor; loop per cell when a second cell lands.
# ponytail: inline #[cfg(test)] fakes, builders, and contract stay in the denominator; #[coverage(off)] is unstable on 1.98.
cargo llvm-cov report --fail-under-lines 80 \
    --ignore-filename-regex 'crates/(kernel|app)/|/test_db\.rs$'

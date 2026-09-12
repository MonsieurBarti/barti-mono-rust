#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
if [[ -f .env ]]; then
    set -a
    # shellcheck disable=SC1091
    source .env
    set +a
fi
export LOADS_DATABASE_URL="${LOADS_TEST_DATABASE_URL:-postgres://loads:loads@127.0.0.1:5432/hive_test}"
export LOADS_MIGRATOR_DATABASE_URL="${LOADS_TEST_MIGRATOR_DATABASE_URL:-postgres://loads_migrator:loads_migrator@127.0.0.1:5432/hive_test}"
exec cargo nextest run --workspace --profile integration "$@"

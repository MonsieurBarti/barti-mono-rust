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
export GRANT_PROOF_DATABASE_URL="${GRANT_PROOF_DATABASE_URL:-postgres://grant_proof:grant_proof@127.0.0.1:5432/hive_test}"
host_port="${LOADS_DATABASE_URL#*@}"
host_port="${host_port%%/*}"
host="${host_port%:*}"
port="${host_port##*:}"
# OrbStack keeps 5432 open after compose stop; /dev/tcp then lies. pg_isready
# speaks the protocol. CI runners without it fall back to the port check.
if command -v pg_isready >/dev/null 2>&1; then
    if ! pg_isready -h "$host" -p "$port" -q; then
        echo "Postgres is unreachable at ${host_port}. Run: docker compose up -d --wait" >&2
        exit 1
    fi
elif ! (exec 3<>"/dev/tcp/${host}/${port}") 2>/dev/null; then
    echo "Postgres is unreachable at ${host_port}. Run: docker compose up -d --wait" >&2
    exit 1
fi
exec "$@"

#!/bin/bash
set -euo pipefail
# One database per nextest db slot; matches test-groups.db.max-threads in .config/nextest.toml.
for slot in 0 1 2 3; do
    psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
        -c "CREATE DATABASE hive_test_${slot};"
    psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "hive_test_${slot}" \
        -f /docker-entrypoint-initdb.d/02-cell.sql
done

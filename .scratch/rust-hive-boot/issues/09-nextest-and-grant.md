# Prove nextest profiles and GRANT

Type: task
Label: wayfinder:task
Status: resolved

Blocked by: 06, 07, 08

## Question

Make `cargo nextest run` the gate this destination names.

Unit profile needs no DSN. It excludes `integration::` and the `tests/` target. Db profile runs loads integration, loads e2e, and app GRANT, and requires worker databases. Invocation is `cargo nextest run`. `cargo test` is a local escape hatch.

GRANT proof lives in `crates/app/tests`. Open the loads cell-role pool and a pool that can see the planted second schema. A planted cross-schema `query` from the loads role expects SQLSTATE `42501`. Not a product E2E. Not a coverage cell.

Green means unit, integration, and the create-load e2e. 80% line on `loads` is CI law via cargo-llvm-cov; kernel and `app` have no floor. Use tdd only if a profile or GRANT test is missing.

Do not add a second product cell. Do not boot `app` from a cell test.

## Answer

`cargo nextest run` is the gate. Default profile is unit: `not (test(/::integration::/) or kind(test))`, no DSN. Profile `db` is `test(/::integration::/) or kind(test)`: loads integration, loads e2e, app GRANT. Db tests join nextest group `db` (`max-threads = 4`) and map the lane DSN onto `hive_test_<NEXTEST_TEST_GROUP_SLOT>`. Compose init creates `hive_test_0` through `hive_test_3`, each with the loads schema and the planted `grant_proof.probe`.

`scripts/with-db.sh` exports the lane DSNs, fails fast on a closed Postgres port, and runs its arguments. `scripts/test-db.sh` runs the db profile. `scripts/coverage.sh` merges unit and db under cargo-llvm-cov and fails `loads` under 80% lines; kernel and `app` are ignored in the report. `scripts/test-integration.sh` and `scripts/test-e2e.sh` are gone.

GRANT proof is `crates/app/tests/grant.rs`: the `grant_proof` owner reads `grant_proof.probe`; the `loads` role gets SQLSTATE `42501` on the same `SELECT`. It does not boot `app`.

CI runs `docker compose up -d --wait`, `scripts/test-db.sh`, and `scripts/coverage.sh` after the four commands. `cargo test` stays the local escape hatch: `scripts/with-db.sh cargo test --workspace -- --test-threads=1`. No second cell.

Proof: unit 94 passed; db 9 passed; `loads` line coverage 94.74%. Gate exit 0 on fmt, clippy, deny, nextest, test-db, coverage.


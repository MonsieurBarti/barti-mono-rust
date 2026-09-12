# Prove nextest profiles and GRANT

Type: task
Label: wayfinder:task
Blocked by: 06, 07, 08

## Question

Make `cargo nextest run` the gate this destination names.

Unit profile needs no DSN. It excludes `integration::` and the `tests/` target. Db profile runs loads integration, loads e2e, and app GRANT, and requires worker databases. Invocation is `cargo nextest run`. `cargo test` is a local escape hatch.

GRANT proof lives in `crates/app/tests`. Open the loads cell-role pool and a pool that can see the planted second schema. A planted cross-schema `query` from the loads role expects SQLSTATE `42501`. Not a product E2E. Not a coverage cell.

Green means unit, integration, and the create-load e2e. 80% line on `loads` is CI law via cargo-llvm-cov; kernel and `app` have no floor. Use tdd only if a profile or GRANT test is missing.

Do not add a second product cell. Do not boot `app` from a cell test.

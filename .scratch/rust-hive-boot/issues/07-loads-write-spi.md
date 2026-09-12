# Persist a Load through the write SPI

Type: task
Status: resolved

Label: wayfinder:task
Blocked by: 01, 03, 04, 05

## Question

Give `loads` a schema and a write SPI that can save a Load.

Migrations live in the cell crate and change only `loads` tables and indexes. State table, not a stream. Include `idempotency_key` unique on `(actor_id, key)`. Stops are exclusive tables in this schema. Columns follow [Which Load fields does the first POST require?](03-create-load-fields.md) and chapter 14 (UUID, `TIMESTAMPTZ`, `DATE`, never `NUMERIC` for money).

Write SPI is `get_by_id` / `save`. `save` takes `Option` of the idempotency record. Replay is not this ticket. sqlx adapter holds the named pool newtype. Cell `new` never takes `PgPool`. `query!` compiles against the cell-role DSN.

Handwritten fake next to the SPI. Contract behind feature `contract`. Integration constructs the use-case or adapter with real sqlx on the cell-role pool. Migrate in tests with the migrator DSN; do not boot `app`. Use tdd.

Do not add HTTP. Do not add Quote HTTP. Do not `CREATE ROLE` or `GRANT` in cell SQL.

## Answer

`loads` owns a state-table write model in its schema. Migration `20260912120000_load_write_model` creates `loads.load`, `loads.stop`, and `loads.idempotency_key`. Ids are `UUID`, `created_at` is `TIMESTAMPTZ`, stop `date` is `DATE`. Unique `(actor_id, key)` on `idempotency_key`. Unique `(load_id, kind)` on `stop`. No `CREATE ROLE` or `GRANT`. No money column.

Write SPI is `get_by_id` / `save`. `save` takes `Option<IdempotencyRecord>` and writes that row on the same transaction when `Some`. Replay is not implemented. `SqlxLoadStore` holds `LoadsPool`. Cell `new` is still absent, so nothing takes `PgPool`. `query!` compiles against `LOADS_DATABASE_URL`. Integration migrates with `LOADS_MIGRATOR_DATABASE_URL` and never boots `app`.

Kernel `Instant` gained `from_unix_timestamp_millis` / `unix_timestamp_millis` so the adapter round-trips millisecond `created_at` without a `time` dep in `loads`.

Handwritten `FakeLoadStore` sits next to the SPI. `load_store_contract` is `pub` under `cfg(any(test, feature = "contract"))`. Both adapters invoke it. Mapper unit tests round-trip optional pickup `name` / `line2`.

No HTTP. No Quote. `loads::new` waits on ticket 08.

Proof: `cargo test --workspace` 69 green including loads mapper, fake contract, and sqlx adapter against local hive. `cargo clippy --workspace --all-targets -D warnings` and `cargo deny check bans` clean under `SQLX_OFFLINE=true`.


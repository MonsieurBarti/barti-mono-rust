# Persist a Load through the write SPI

Type: task
Label: wayfinder:task
Blocked by: 01, 03, 04, 05

## Question

Give `loads` a schema and a write SPI that can save a Load.

Migrations live in the cell crate and change only `loads` tables and indexes. State table, not a stream. Include `idempotency_key` unique on `(actor_id, key)`. Stops are exclusive tables in this schema. Columns follow [Which Load fields does the first POST require?](03-create-load-fields.md) and chapter 14 (UUID, `TIMESTAMPTZ`, `DATE`, never `NUMERIC` for money).

Write SPI is `get_by_id` / `save`. `save` takes `Option` of the idempotency record. Replay is not this ticket. sqlx adapter holds the named pool newtype. Cell `new` never takes `PgPool`. `query!` compiles against the cell-role DSN.

Handwritten fake next to the SPI. Contract behind feature `contract`. Integration constructs the use-case or adapter with real sqlx on the cell-role pool. Migrate in tests with the migrator DSN; do not boot `app`. Use tdd.

Do not add HTTP. Do not add Quote HTTP. Do not `CREATE ROLE` or `GRANT` in cell SQL.

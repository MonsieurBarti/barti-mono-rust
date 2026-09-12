# POST a Load with Idempotency-Key

Type: task
Status: resolved

Label: wayfinder:task
Blocked by: 03, 06, 07

## Question

Expose one human POST that creates a Load.

`POST /loads` maps to a create-load API port. Body is the PL from [Which Load fields does the first POST require?](03-create-load-fields.md). Header `Idempotency-Key` is required. Missing, empty, or over-length is `VALIDATION_FAILED` in presentation and never enters the cell. Port takes `actor_id` and `idempotency_key` as strings. Decode is the first act. Success is 201 with PL JSON. Handler chooses no business logic.

Idempotency SPI `get` is a committed read. Matching fingerprint replays the stored `Result`. Mismatch is `VALIDATION_FAILED`. Miss runs the command. `save` writes the row on the same transaction. Store success and domain 4xx. Do not store 5xx. `23505` on the unique is `*_CONFLICT` → 409.

Create-load may stay `pub(crate)` if no other cell should call it. `lib.rs` still exports `new` and `router`. Envelope is `{ type, context }` at the port; public document is problem+json.

Unit: codec and Load invariants. Integration: happy path, error, replay, `23505`; real sqlx; fake leaving SPIs. E2E: `router` with tower; missing header; one happy POST. Never boot `app` from this cell. Use tdd.

Do not add GET or list. Do not add Quote.

## Answer

`POST /loads` is the create-load driving adapter. Create-load is a `pub(crate)` API-port trait implemented by `CreateLoadCommand`. `lib.rs` exports `new` and `router`. `new` takes `LoadsPool`; `app` wraps the named `DatabaseConnection`. `new` binds `Clock`, `Logger`, and `Metrics`. `app serve` nests `loads::router`.

Body is ticket 03 PL. Codec `decode` is the first act. Header `Idempotency-Key` is required in presentation: missing, empty, or over-length is `VALIDATION_FAILED` and never enters the cell. Port takes `actor_id` and `idempotency_key` as strings. Success is 201 Load JSON. Handler has no business logic.

Idempotency SPI `get` is a committed read. Matching fingerprint replays the stored `Result`. Mismatch is `VALIDATION_FAILED`. Miss runs the command. `save` writes the idempotency row on the same transaction as the Load. Store success. Do not store 5xx or garde failures. `23505` is `LOAD_CONFLICT` → 409. Envelope is `CreateLoadError` at that port. Public document is problem+json.

Proof: sea-orm adapter (`SeaOrmLoadStore`, handwritten `Model`/`ActiveModel`, `schema_name = "loads"`, `23505` via `DbErr::sql_err()`), `sea-orm-migration` `Migrator` exported from the cell and run by `app migrate` with `search_path = loads`, sqlx gone as a direct dependency. Default nextest excludes `integration::` and the loads `tests/` target (no Postgres in CI). Cell tests never boot `app`.


# Harden the compile wall

Type: task
Label: wayfinder:task
Status: resolved

Blocked by:

## Question

Which hive forbiddens can rustc, clippy, and cargo-deny enforce today? Land them, so the rules cover only what the compiler cannot see.

Chapter 3 names `unreachable_pub` as part of the import wall. It is not configured. `Cargo.toml` has no `[workspace.lints]`. There is no `clippy.toml`. `deny.toml` walls `tracing` and OpenTelemetry out of cells and walls `loads` behind `app`; no other cell line exists.

Candidates: `[workspace.lints.rust] unreachable_pub = "deny"` with `[lints] workspace = true` in every crate; `clippy.toml` `disallowed-methods` for `std::time::SystemTime::now`, `time::OffsetDateTime::now_utc`, and `time::OffsetDateTime::now_local`, with kernel `SystemClock` carrying `#[expect(clippy::disallowed_methods)]`; `disallowed-types` where a type is forbidden workspace-wide; a `deny.toml` line per cell with a comment that says how a new cell adds its own.

Layer-internal forbiddens are module-grain and clippy is crate-grain: serde or garde outside `domain/api`, sqlx in domain or application, `Arc<dyn`, `async_trait`, `PgPool` on cell `new`, runtime `query()`. Do not fake them with a script. List them for the rules.

Acceptance: CI's four commands are green; the answer lists what the compiler now catches and what stays prose. Fix any lint the new config surfaces in existing code.

## Answer

`unreachable_pub` is deny at workspace grain. Kernel, loads, and app set `[lints] workspace = true`.

Clippy bans `std::time::SystemTime::now`, `time::OffsetDateTime::now_utc`, and `time::OffsetDateTime::now_local`. `SystemClock::now` carries `#[expect(clippy::disallowed_methods)]`. `now_local` uses `allow-invalid` because `time` does not enable `local-offset`.

`deny.toml` already walls `loads` behind `app` and walls tracing plus OpenTelemetry out of cells. A comment records the line a new cell adds.

No `disallowed-types` list. No type is forbidden workspace-wide.

The wall surfaced intra-crate `pub` in `app` and `loads::load_store_contract`. Those are now `pub(crate)`.

Stays prose, module-grain: serde or garde outside `domain/api`; sqlx in domain or application; `Arc<dyn`; `async_trait`; `PgPool` on cell `new`; runtime `query()`.

Gate: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans`, `cargo nextest run --workspace` (67 passed, 1 skipped). Compose Postgres was down. Db lane skipped.

# Write the TTSR interrupts

Type: task
Label: wayfinder:task
Status: resolved

Blocked by: 02

## Question

At most six `.omp/rules/ttsr-<forbidden>.md` files that interrupt an edit while it streams. Only forbiddens the compiler does not catch after [Harden the compile wall](02-harden-the-compile-wall.md).

Candidates: `use serde` or `use garde` under `src/domain/` outside `src/domain/api/`; `sqlx` under `src/domain/` or `src/application/`; `tracing` in any cell crate; `Arc<dyn`; `async_trait`; `SystemTime::now` or `now_utc` in a cell; runtime `sqlx::query(` or `query_as(`. Choose six or fewer by false-positive risk.

Each file: `condition` regex, `globs` as the path gate, `scope` limited to `tool:edit(<glob>)` and `tool:write(<glob>)`, `interruptMode`, and a body of one to three lines naming the ADR and the fix. See `omp://rulebook-matching-pipeline.md` §6 for field semantics. `domain/api` must not trigger the serde or garde rule. Express the exclusion in the glob (brace groups over the sibling folders) or the regex, and prove it.

Acceptance: a throwaway script runs each regex against one positive and one negative snippet, including the `domain/api` negative; the answer records the names, the patterns, and any candidate dropped and why. Delete the script.

## Answer

Four TTSR files in `.omp/rules/`. Each has `condition`, `globs`, `scope` on `tool:edit`/`tool:write`, and `interruptMode: tool-only`.

- `ttsr-serde-garde` — `use\s+(serde|garde)(\b|::)`. Globs: `crates/cells/**/src/domain/{entities,events,errors,spi}/**` and `crates/cells/**/src/domain/*.rs`.
- `ttsr-sqlx` — `use\s+(sqlx|sea_orm)\b|(sqlx|sea_orm)::`. Globs: domain and application.
- `ttsr-arc-dyn` — `Arc\s*<\s*dyn\b`. Glob: `crates/cells/**`.
- `ttsr-async-trait` — `\basync_trait\b`. Glob: `crates/cells/**`.

Dropped as compiler: `tracing` in a cell (`deny.toml`); `SystemTime::now` / `now_utc` (clippy `disallowed-methods`).
Dropped `sqlx::query(` / `query_as(`: the interrupt would teach `query!`, and [Persistence](../../../docs/adr/0007-persistence.md) now forbids `query!`.

Throwaway script: one positive and one negative per regex; `use serde::Deserialize` still matches the serde regex in `domain/api`, and the brace glob misses `crates/cells/freight/loads/src/domain/api/create_load/mod.rs`. Script deleted.

Gate: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans`, `cargo nextest run --workspace` (67 passed, 1 skipped). Compose Postgres was down. Db lane skipped.

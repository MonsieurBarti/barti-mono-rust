# Write the TTSR interrupts

Type: task
Label: wayfinder:task
Blocked by: 02

## Question

At most six `.omp/rules/ttsr-<forbidden>.md` files that interrupt an edit while it streams. Only forbiddens the compiler does not catch after [Harden the compile wall](02-harden-the-compile-wall.md).

Candidates: `use serde` or `use garde` under `src/domain/` outside `src/domain/api/`; `sqlx` under `src/domain/` or `src/application/`; `tracing` in any cell crate; `Arc<dyn`; `async_trait`; `SystemTime::now` or `now_utc` in a cell; runtime `sqlx::query(` or `query_as(`. Choose six or fewer by false-positive risk.

Each file: `condition` regex, `globs` as the path gate, `scope` limited to `tool:edit(<glob>)` and `tool:write(<glob>)`, `interruptMode`, and a body of one to three lines naming the ADR and the fix. See `omp://rulebook-matching-pipeline.md` §6 for field semantics. `domain/api` must not trigger the serde or garde rule. Express the exclusion in the glob (brace groups over the sibling folders) or the regex, and prove it.

Acceptance: a throwaway script runs each regex against one positive and one negative snippet, including the `domain/api` negative; the answer records the names, the patterns, and any candidate dropped and why. Delete the script.

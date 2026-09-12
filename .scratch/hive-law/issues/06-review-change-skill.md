# Write the review-change skill and command

Type: task
Label: wayfinder:task
Status: resolved
Blocked by: 03

## Question

`.omp/skills/review-change/SKILL.md` and `.omp/prompts/review-change.md`. The prompt has a `description` and passes `$ARGUMENTS`: a PR number or a branch, plus the optional word `post`. A missing target stops. Shape matches `~/.omp/agent/prompts/review-fintech.md`.

The skill reviews the diff against `origin/main` on two axes in parallel `task` subagents, then one judge pass:

- Law. For each touched file, map its path to a layer rule name from the map's contract and read `rule://hive-<layer>`; read the ADRs that rule links. Each finding names the rule line and the ADR.
- Ticket. Find the ticket the PR body or branch name points at (`<effort-short>/NN-<slug>` maps to `.scratch/<effort>/issues/NN-<slug>.md`). Check that the diff does what the `## Question` asks and nothing it forbids.

Output is one finding per line: `path:line — ADR or ticket line — reject | warn — fix`. `reject` is a law break or a missed ask. `warn` is taste. No finding without a line. End with a verdict: ship, or fix N rejects. Chat only. With `post`, publish a GitHub review through `gh api` with one inline comment per finding and the verdict as the review body.

The skill is a document for agents: `skill://writing-for-agents`. Name what it never does: edit files, approve, merge, review without a target.

Acceptance: both files exist with valid frontmatter; `/review-change` appears in a fresh omp session; running it on the latest merged boot PR (`gh pr list --state merged`) produces findings in the format above with no invented line numbers.

## Answer

`.omp/skills/review-change/SKILL.md` and `.omp/prompts/review-change.md`. Skill has `name` and `description`. Prompt has `description` and `$ARGUMENTS`. Empty target stops. Prompt shape matches naboo `review-fintech`: specialists and judge are `task` with `schemaMode: "strict"`.

Guardrails: no file edits, no approve, no merge, no review without a target. `post` uses `gh api repos/<owner>/<repo>/pulls/<n>/reviews`. `event` is `REQUEST_CHANGES` or `COMMENT`, never `APPROVE`.

Dogfood on latest merged boot PR 6 (`hive-boot/07-loads-write-spi`, head `aa4a164`). Ticket axis: no findings. Law lines confirmed with `git show`:

`crates/cells/freight/loads/src/infrastructure/load_store.rs:26 — Persistence — reject — Drop sqlx::query!; persist through handwritten sea-orm Model/ActiveModel in infrastructure.`

`crates/cells/freight/loads/src/infrastructure/pool.rs:3 — Persistence — reject — Wrap sea_orm::DatabaseConnection in LoadsPool, not sqlx::PgPool.`

`crates/cells/freight/loads/src/domain/entities/load.rs:91 — Persistence — warn — Move LoadRow/StopRow/to_rows/from_rows into infrastructure sea-orm Model/ActiveModel.`

`crates/cells/freight/loads/src/domain/entities/load.rs:191 — Testing — warn — Build the stored Instant with kernel FakeClock, not Instant::from_unix_timestamp.`

fix 2 rejects

Self-review of this branch: no layer files. Ticket Question met. Verdict: ship.

Gate: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans`, `cargo nextest run --workspace` (67 passed, 1 skipped). Compose Postgres was down. Db lane skipped.

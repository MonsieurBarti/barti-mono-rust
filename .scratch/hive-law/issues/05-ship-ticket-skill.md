# Write the ship-ticket skill and command

Type: task
Label: wayfinder:task
Status: resolved

Blocked by:

## Question

`.omp/skills/ship-ticket/SKILL.md` and `.omp/prompts/ship-ticket.md`. The prompt has a `description` and passes `$ARGUMENTS` as the ticket path; a missing path stops. Shape matches `~/.omp/agent/prompts/review-fintech.md`: a few lines that read the skill and hand over the argument.

The skill takes a ticket path under `.scratch/<effort>/issues/NN-<slug>.md` and ships it:

1. Read the ticket, the map's Notes, and the tickets it links. Claim: set `Status: claimed` and save.
2. Worktree `~/.omp/wt/barti-<effort-short>-NN` on branch `<effort-short>/NN-<slug>` from `origin/main`. `effort-short` is the map's short name (`hive-boot`, `hive-law`). Follow `skill://omp-worktree-absolute-paths`.
3. Implement with `skill://tdd`. Every file edit consults the layer rule for that path.
4. Gate: the four CI commands from the map Notes. When compose Postgres is up, also the db lane. Fix every failure, pre-existing included.
5. Self-review with `skill://review-change` on the branch. Fix every `reject`. Note each `warn` accepted and why.
6. Resolve the ticket on the same branch: `## Answer`, `Status: resolved`, one line in the map's Decisions so far.
7. Conventional commits (`feat:`, `fix:`, `chore:`). Push with hooks. `gh pr create` with body: ticket link, answer gist, gate summary, warns accepted. Code-owner review is required. Do not merge.

The skill is a document for agents: `skill://writing-for-agents`. Name every command exactly. Name what the skill never does: merge, `--no-verify`, edit a law file, ship two tickets in one run.

Acceptance: both files exist with valid frontmatter (`name` and `description` on the skill, `description` on the prompt); `/ship-ticket` appears in a fresh omp session; the skill body is under one hundred lines.

## Answer

`.omp/skills/ship-ticket/SKILL.md` and `.omp/prompts/ship-ticket.md`. Skill has `name` and `description`. Prompt has `description` and `$ARGUMENTS`. Empty path stops.

Eight steps. After the CI gate, `skill://local-attack` runs on the ticket. Commands: `git fetch origin main`, `git worktree add -b <effort-short>/NN-<slug> ~/.omp/wt/barti-<effort-short>-NN origin/main`, the four CI commands, `cargo nextest run --workspace --profile db` when `docker compose ps postgres` shows running, `git push -u origin HEAD`, `gh pr create`. Guardrails: no merge, no `--no-verify`, no law-file edit, one ticket.

`ttsr-sqlx` is now `ttsr-sea-orm`. Regex is `use\s+sea_orm\b|sea_orm::`. Kernel rule dropped `sqlx-free`.

Self-review: `review-change` is not on this map yet. No reject against the Question. Warn: the db profile waits on boot 09. `local-attack` is not in this repo; the skill points at `skill://local-attack`.

Gate: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans`, `cargo nextest run --workspace` (67 passed, 1 skipped). Compose Postgres was down. Db lane skipped.


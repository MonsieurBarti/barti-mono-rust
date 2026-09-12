# Write the ship-ticket skill and command

Type: task
Label: wayfinder:task
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

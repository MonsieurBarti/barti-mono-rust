# Write the review-change skill and command

Type: task
Label: wayfinder:task
Blocked by: 03

## Question

`.omp/skills/review-change/SKILL.md` and `.omp/prompts/review-change.md`. The prompt has a `description` and passes `$ARGUMENTS`: a PR number or a branch, plus the optional word `post`. A missing target stops. Shape matches `~/.omp/agent/prompts/review-fintech.md`.

The skill reviews the diff against `origin/main` on two axes in parallel `task` subagents, then one judge pass:

- Law. For each touched file, map its path to a layer rule name from the map's contract and read `rule://hive-<layer>`; read the ADRs that rule links. Each finding names the rule line and the ADR.
- Ticket. Find the ticket the PR body or branch name points at (`<effort-short>/NN-<slug>` maps to `.scratch/<effort>/issues/NN-<slug>.md`). Check that the diff does what the `## Question` asks and nothing it forbids.

Output is one finding per line: `path:line — ADR or ticket line — reject | warn — fix`. `reject` is a law break or a missed ask. `warn` is taste. No finding without a line. End with a verdict: ship, or fix N rejects. Chat only. With `post`, publish a GitHub review through `gh api` with one inline comment per finding and the verdict as the review body.

The skill is a document for agents: `skill://writing-for-agents`. Name what it never does: edit files, approve, merge, review without a target.

Acceptance: both files exist with valid frontmatter; `/review-change` appears in a fresh omp session; running it on the latest merged boot PR (`gh pr list --state merged`) produces findings in the format above with no invented line numbers.

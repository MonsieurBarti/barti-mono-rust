---
name: ship-ticket
description: Ship one hive ticket from `.scratch/<effort>/issues/NN-<slug>.md` through a PR.
---

# Ship a ticket

One path under `.scratch/<effort>/issues/NN-<slug>.md`. Stop if the path is missing.

Leave merge to the code owner. Push with hooks. Edit product and hive-law files only. Ship this ticket only.

Never merge. Never `--no-verify`. Never edit `docs/architecture.md`, `docs/adr/`, or `CONTEXT.md`. Never ship a second ticket.

## 1. Read and claim

Read the ticket, `.scratch/<effort>/map.md` Notes, and every ticket it links.

Set `Status: claimed` and save.

Done when the file shows `Status: claimed`.

## 2. Worktree

`effort-short` is `hive-law` from `.scratch/hive-law/`, and `hive-boot` from `.scratch/rust-hive-boot/`. Parse `NN` and `<slug>` from the filename.

Read `skill://omp-worktree-absolute-paths`. Prefix every path and command with the worktree.

```
git fetch origin main
git worktree add -b <effort-short>/NN-<slug> ~/.omp/wt/barti-<effort-short>-NN origin/main
```

If that directory exists, use it. Done when `git -C ~/.omp/wt/barti-<effort-short>-NN status` is on that branch.

## 3. Implement

Read `skill://tdd`. Before each edit, read the layer rule `rule://hive` names for that path.

Done when the ticket Question is met.

## 4. Gate

From the worktree:

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans
cargo nextest run --workspace
```

If `docker compose ps postgres` shows running:

```
cargo nextest run --workspace --profile db
```

Fix every failure, pre-existing included. Done when every run command exits 0.

## 5. Attack

Read `skill://local-attack` and run it on this ticket from the worktree.

Done when local-attack is green.

## 6. Self-review

Read `skill://review-change` on this branch. Fix every `reject`. Record each `warn` and why it stays.

Done when no `reject` remains.

## 7. Resolve

On the same branch: append `## Answer`, set `Status: resolved`, add one gist line to the map's Decisions so far.

Done when the ticket shows `Status: resolved` and the map line exists.

## 8. Push

Commit with `feat:`, `fix:`, or `chore:`. Then:

```
git push -u origin HEAD
gh pr create --base main --title "<conventional title>" --body "<ticket link, answer gist, gate summary, warns accepted>"
```

Done when the PR URL exists. Leave merge to the code owner.

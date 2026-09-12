---
name: local-attack
description: Attack a hive branch or PR against live local HTTP. Use when ship-ticket step 5 runs, when the target is a PR, or when endpoints need compose Postgres.
---

# Attack a change

Diff is the working tree against `origin/main`, or a PR against its base. Attack hits live `app serve`. The lock is a hive-tests cargo test. Green when every planned case passed and every new lock bit.

If this checkout is a worktree, read `skill://omp-worktree-absolute-paths` and prefix every path and command.

This run owns local cell tables in `hive`. Attack creates preconditions over HTTP. The cargo test is the memory. CI never binds the serve port.

Never SQL seed. Never point serve at `hive_test`. Never bind a server from a cargo test. Never kill a process this run did not start. Never skip a failure as pre-existing without a failing cargo test on `origin/main`.

## 1. Surface

`git fetch origin main`.

If the target is a PR number:

```
gh pr diff <n> --name-only
```

On that PR's worktree, also `git diff --name-only origin/<base>` so unstaged files count. `<base>` is `gh pr view <n> --json baseRefName`.

Otherwise `git diff --name-only origin/main`. That set includes staged and unstaged tracked files. Ship-ticket attacks before commit.

Skip when every path is under `docs/`, `.omp/`, `.scratch/`, or is `CONTEXT.md`. Chat `local-attack: green (no surface)`.

Otherwise the surface is every public route of every cell the diff touches. Read routes from that cell's `presentation/http`. A change in `crates/app`, `crates/kernel`, `postgres/`, `compose.yaml`, or `.env.example` includes every cell.

Done when the skip line is in chat, or the route list exists.

## 2. Plan

Write `.scratch/attack-plan.md`. Do not commit it.

For each route: happy path, documented failures, missing actor header, other actor's resource, replayed idempotency key, different body same key, and multi-row state the diff can break. Preconditions are HTTP calls in this file. Human REST headers live in `crates/app/src/http.rs`. POST and PATCH follow the cell handler's idempotency rules. Different body same key is whatever that handler returns. Concurrent save may be 409.

Done when the file lists every route and every case.

## 3. Boot

Export product `LOADS_MIGRATOR_DATABASE_URL`, `LOADS_DATABASE_URL`, and `LOADS_POOL_MAX` from the `hive` lines in `.env.example`. App does not load `.env`.

If that migrator DSN already accepts a connection, skip compose.

Otherwise:

```
docker compose up -d --wait
```

If `up` fails because 5432 is already bound, reuse that listener. Tear down a compose project this run created that never became ready.

Then `cargo run -- migrate`.

Copy the `TRUNCATE` SQL from each touched cell's `test_db.rs`. Run it as that cell's migrator against database `hive`. Do not call `migrated_pool`.

Start `cargo run -- serve` as a long-running process. Ready when `GET {BIND}/health` is 200. `BIND` is `crates/app/src/serve.rs`. If the port is taken, stop. Do not steal it. Do not wait on a log line.

Done when `/health` is 200.

## 4. Attack

Run every case in the plan with HTTP against `BIND`. Record status on each line of the plan. Exhaust the plan.

Done when every case has a recorded status.

## 5. Lock

A finding is in-scope until a cargo test on `origin/main` shows the same failure.

Fix in-scope bugs. Add a lock in the lane that observes the bug, per `rule://hive-tests`. HTTP contract goes in cell `tests/` with tower oneshot. Never boot `app` from the test.

Read `skill://prove-regression-guard-bites` and run it on each new lock.

Re-run the whole plan after fixes.

Done when every planned case passed and every new lock bit.

## 6. Tear down

Stop the serve this run started. TRUNCATE again. Leave compose up.

The campaign is the plan with every case status, then `local-attack: green` or `local-attack: red`.

If this branch has a PR, write that campaign into the PR body's Attack campaign section. Then delete `.scratch/attack-plan.md`.

If there is no PR, leave `.scratch/attack-plan.md` for ship-ticket.

Chat `local-attack: green` or `local-attack: red` with the remaining findings.

Done when `BIND` is down, cell tables are empty, the campaign is in the PR body or the plan file, and chat has the line.

---
name: review-change
description: Review a hive PR or branch against origin/main on Law and Ticket.
---

# Review a change

Target is a PR number or a branch, with optional trailing `post`. Stop if the target is missing.

Report in chat. Leave the tree untouched. Leave merge and approve to the code owner.

Never edit files. Never approve. Never merge. Never review without a target.

## 1. Parse

Last token `post` sets publish. The rest is the target.

Digits: `gh pr view <n> --json number,title,body,headRefName,headRefOid`. Diff: `gh pr diff <n>`.

Else: `git fetch origin main <target>` then `git diff origin/main...<target>`.

Done when the diff and the head ref exist.

## 2. Law and Ticket

Spawn two `task` subagents, `schemaMode: "strict"`. Never `reviewer`. Never `skill://code-review`.

### Law

For each touched file, first glob wins:

| glob | rule |
| --- | --- |
| `crates/cells/**/src/domain/**` | `rule://hive-domain` |
| `crates/cells/**/src/application/**` | `rule://hive-application` |
| `crates/cells/**/src/infrastructure/**` | `rule://hive-infrastructure` |
| `crates/cells/**/src/presentation/**` | `rule://hive-presentation` |
| `crates/cells/**/migrations/**` | `rule://hive-migrations` |
| `crates/app/**` | `rule://hive-app` |
| `crates/kernel/**` | `rule://hive-kernel` |
| `crates/**/tests/**` | `rule://hive-tests` |

Skip a file with no glob. Read the rule and every ADR it links. Each finding names the rule line and the ADR.

### Ticket

Branch or `headRefName` `<effort-short>/NN-<slug>`: `hive-law` → `.scratch/hive-law/issues/NN-<slug>.md`; `hive-boot` → `.scratch/rust-hive-boot/issues/NN-<slug>.md`. Also read a `.scratch/.../issues/` path in the PR body.

Read `## Question`. The diff does what it asks and nothing it forbids. A missed ask anchors on that ticket line.

Done when both reports return.

## 3. Judge

One `task` subagent, `schemaMode: "strict"`. Keep a finding only when `path:line` exists in the head file or the ticket file. Confirm with `git show <sha>:<path>` or a Read of the ticket.

Emit one line per finding:

`path:line — ADR or ticket line — reject | warn — fix`

`reject` is a law break or a missed ask. `warn` is taste.

Verdict: `ship`, or `fix N rejects`.

Done when every kept line is confirmed and the verdict is printed.

## 4. Publish

Chat the lines and the verdict.

With `post` on a PR:

```
gh repo view --json nameWithOwner
gh pr view <n> --json headRefOid
gh api repos/<owner>/<repo>/pulls/<n>/reviews --method POST --input <payload.json>
```

Payload: `commit_id` is `headRefOid`. `event` is `REQUEST_CHANGES` when any `reject` remains, else `COMMENT`. One inline comment per finding (`path`, `line`, `side: RIGHT`, `body`). Review `body` is the verdict. Stop `post` when the target is a branch.

Done when chat has the report, and with `post` the review URL exists.

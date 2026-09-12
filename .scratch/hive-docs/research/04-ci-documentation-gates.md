# CI gates that keep docs true to the code

Researched 2026-09-12. Ticket: [How do CI gates enforce documentation without becoming theater?](../issues/04-ci-documentation-gates.md)

## Verdict

Five blocking checks, all mechanical, all inside the existing `ci` job. Two advisory runs outside it.

| # | Gate | Command | Blocks | Catches |
| --- | --- | --- | --- | --- |
| 1 | Docs coverage | `scripts/check-docs` | yes | missing handbook page, missing cell `CONTEXT.md`, orphan page, cell absent from `CONTEXT-MAP.md`, public router with no spec file |
| 2 | Spec drift | regenerate, then `git diff --exit-code -- docs/openapi` | yes | committed OpenAPI no longer matches the handlers |
| 3 | Spec validity | `oasdiff validate` | yes | spec that is not a legal OpenAPI document |
| 4 | Consumer break | `oasdiff breaking` base vs HEAD, `--fail-on ERR` | yes | a change that breaks existing clients |
| 5 | Local links | `lychee --offline --include-fragments` | yes | broken relative links and anchors between docs |
| — | External links | `lychee` full, weekly schedule | no | rotted third-party URLs |
| — | Changelog | `oasdiff changelog` to the job summary | no | reviewer context on what consumers see |

**The gate that makes the other five real is not a job. It is a rule.** `.github/rulesets/protect-main.json` carries `deletion`, `non_fast_forward`, and `pull_request`, and no `required_status_checks` rule. Every job in `ci.yml` and `rulesets.yml` is advisory today: red CI does not block the merge button. A docs job added without that rule is theater by construction. The rule type is `required_status_checks` with `parameters.required_status_checks[].context` and a required `strict_required_status_checks_policy`; a `workflows` rule that names a workflow path instead of a check name is the alternative. Both are settable through the checked-in ruleset file the repo already diffs against live in `rulesets.yml`.

Anti-theater test, applied to every candidate below: **a gate earns merge-blocking power only when it can fail for a reason the author caused and can fix in the same PR.** Prose taste fails that test, so it stays out, matching chapter 2, which sends boundary judgment to design review and states "No hop-count CI". External network links fail it too, for a different reason: they break on someone else's schedule.

## Compared

### 1. Missing files

The mechanical form of "the docs cover the code" is a set equality check between two directory listings, not a quality judgment. Today the workspace holds three crates (`crates/app`, `crates/kernel`, `crates/cells/freight/loads`), so the listing is short and a shell script reads it in one `find`.

GitLab runs this shape as merge-blocking CI: `docs-lint redirects` fails when a doc file is deleted or renamed without a redirect, and `docs-i18n-lint paths` fails when a translated file has no corresponding English source. Both are pure existence checks over paths, and both block ([GitLab, Documentation testing](https://docs.gitlab.com/development/documentation/testing/)).

Options for hosting it:

| Option | Cost | Verdict |
| --- | --- | --- |
| `scripts/check-docs`, bash | ~30 lines, no new pin, no compile | **Take it.** Matches `scripts/sync-rulesets` and `scripts/test-*.sh`, already the repo's convention for "CI runs a script". |
| `xtask` crate | new workspace member, compiles in CI, imports nothing | Reject. A directory listing does not need a crate. |
| A docs-site generator's own link/nav check | pulls the site generator into the gate | Reject here. Tooling is still unspecified on the map. |

The check has four rules, all derived from decisions already locked on the map (a cell gets a handbook page; the spec lands `CONTEXT-MAP.md` and per-cell `CONTEXT.md`; OpenAPI is public REST only):

1. Every cell crate (`crates/cells/*/*/Cargo.toml`) has a handbook page and a cell `CONTEXT.md`.
2. Every handbook cell page maps back to a cell crate. This is the orphan direction, and it is the one that catches a deleted cell.
3. Every cell appears in `CONTEXT-MAP.md`.
4. Every cell that exposes a public REST router has a spec file.

Rule 4 is the only one needing a convention rather than a listing. Cheapest mechanical proxy: the spec emitter writes one file per cell that registers public routes, so rule 4 collapses into gate 2 and the script only has to assert the file is non-empty.

### 2. OpenAPI drift from handlers

Drift and breakage are two different failures and want two different tools.

**Drift** is "the committed spec no longer equals what the code emits". The only reliable mechanical form: commit the generated artifact, regenerate it in CI, and fail on any difference. `git diff --exit-code` exits 1 when there are differences and 0 when there are none ([git-diff docs](https://git-scm.com/docs/git-diff)), so the gate is one command plus one diff, no bespoke comparator.

Precedent for exactly this pattern, in both of the repo's neighbourhoods:

- sqlx ships `cargo sqlx prepare --check`, which "Exits with a nonzero exit status if the data in `.sqlx` is out of date with the current" queries ([sqlx-cli README](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)). `sqlx` 0.9 is already pinned in [ADR 0001](../../../docs/adr/0001-stack-pins.md).
- GitLab's `graphql-verify` job "Fails when `doc/api/graphql/reference/_index.md` is not updated" by the generation process, and they list it under a heading that names the whole category: files generated from scripts, where a CI job fails when source or docs move without the other ([GitLab, Documentation testing](https://docs.gitlab.com/development/documentation/testing/)).

The alternative, deriving the spec at request time and diffing a live server against the file, needs a booted process and a database. It buys nothing the regenerate-and-diff gate does not already prove, and chapter 10 keeps `app` out of cell test lanes.

The gate needs one thing from the emitter: a command that writes the document to a path, deterministically, with stable key ordering. Which crate provides it is [ticket 09](../issues/09-openapi-crate-and-annotations.md); the gate does not care, as long as byte-stability holds. Byte-instability is the one failure mode that turns this gate into noise, so the spec should state determinism as a requirement on the emitter, not as a CI flag.

**Breakage** is "the spec changed in a way that breaks existing clients". oasdiff is the tool that owns this question. Its subcommands split the space precisely: `diff` is every difference including documentation-only edits, `changelog` is the changes that can affect consumers, `breaking` is only the changes that break existing clients, and `validate` checks a single spec for per-RFC violations such as invalid types, missing required fields, and unresolved `$ref`s ([oasdiff README](https://github.com/oasdiff/oasdiff)). Inputs accept git revisions, so the base side is `origin/<base_ref>:docs/openapi/<cell>.yaml` and no second checkout is needed.

The official action publishes `breaking`, `changelog`, `diff`, and `validate` as separate sub-actions. `breaking` takes `fail-on: ERR | WARN`, writes inline `::error::` annotations on the Files changed tab, and defaults `allow-external-refs: false` to prevent SSRF on untrusted pull requests. It also defaults `review: true`, which uploads the two specs, encrypted, to oasdiff.com; set `review: false` so no spec leaves CI ([oasdiff-action README](https://github.com/oasdiff/oasdiff-action)). Latest releases at time of writing: oasdiff `v1.31.0` (2026-09-05), oasdiff-action `v0.1.15` (2026-09-05).

Alternatives checked so they are not silent successors:

| Tool | What it does | Verdict |
| --- | --- | --- |
| **oasdiff** | validate, breaking, changelog, git-ref inputs, GitHub Action | **Take it.** Only one of the three that answers "does this break a client", which is the question chapter 8 actually asks. |
| Redocly CLI `lint`, Spectral | rule-based spec linting and style rules | Lint only. Style rules on a spec are the prose-taste problem wearing a schema. `oasdiff validate` covers the mechanical half. |
| **vacuum** | OpenAPI linter, Spectral-compatible rules | Same category as above, but worth noting it is installable through the `taiki-e/install-action` step already in `ci.yml` ([install-action TOOLS.md](https://github.com/taiki-e/install-action/blob/main/TOOLS.md)), which neither oasdiff nor lychee is. If a spec linter is ever wanted, this is the cheap one. |

One wrinkle the spec must name: the first PR that adds a spec file has no base version to compare against. Run `breaking` only when the path exists on the base ref, or land the spec in its own PR before switching the gate on.

### 3. Broken links

lychee checks Markdown, HTML, and text; handles relative URLs; and exits 0 on success, 1 on runtime or input errors, 2 on link check failures, and 3 on config errors, so a CI step distinguishes "the docs are wrong" from "the tool is wrong" ([lychee README](https://github.com/lycheeverse/lychee)). Two flags carry the gate:

- `--offline` "Only check local files and block network requests". That makes the run deterministic and network-free, which is what a merge gate needs.
- `--include-fragments` enables checking of anchor fragments, so a link to a renamed handbook heading fails instead of silently pointing at the top of the page. Given the map's entrypoint-links-law structure, cross-document anchors are the links most likely to rot.

`--root-dir` is required if absolute links appear in local files, otherwise those links are flagged as errors.

GitLab reaches the same split from the other end. Their `docs-lint links` job checks links "including anchor links", and "Any link that requires a network connection is skipped"; the i18n variant lists broken links in the job log but does not fail the pipeline ([GitLab, Documentation testing](https://docs.gitlab.com/development/documentation/testing/)). Network-dependent link checking is not merge-gate material anywhere that has tried it at scale.

External links still rot, so run them on a schedule, not on the PR. The official action's documented usage is precisely this: a daily `schedule` trigger with `fail: false`, feeding `peter-evans/create-issue-from-file` to open an issue when `exit_code != 0`, plus an `actions/cache` entry for `.lycheecache` ([lychee-action README](https://github.com/lycheeverse/lychee-action)). Ignore patterns live in `.lycheeignore`. lychee `v0.24.2`, lychee-action `v2.9.0`; the action's `lycheeVersion` input pins the binary.

### 4. The wiring traps

Every one of these turns a green board into a lie, and all are documented by GitHub.

| Trap | Consequence | Rule |
| --- | --- | --- |
| Required workflow skipped by `paths`, branch filtering, or a skip commit message | "Associated checks stay in a 'Pending' state and block merging" | Never put `paths:` on a required workflow. GitHub's own advice: "Avoid requiring workflows that can be skipped." |
| Job skipped by a conditional | The job reports **Success** | A conditional docs job that skips is a pass, not a gate. |
| Job `needs:` a failed job | Dependent job is skipped and "may not block merging" | Use `always()` with `needs`, or keep one job. |
| Merge queue | Required Actions checks are not triggered without the `merge_group` event, and "The merge will fail as the required status check will not be reported" | Add `merge_group:` to `on:` if a queue is ever enabled. |
| Renamed job | The required `context` is never reported, so PRs sit pending | Rename the job and the ruleset context in the same commit, or require the workflow path instead of the check name. |
| Stale check | A required check "must have completed successfully in the chosen repository during the past seven days" and must pass on the latest commit SHA | Nothing to configure; worth knowing before debugging a stuck PR. |

Source for the first four rows and the staleness rule: [Troubleshooting required status checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks). Strict versus loose policy: strict forces the branch up to date before merging and costs a rebuild per base-branch merge; loose does not ([Available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)). For a one-maintainer repo, loose.

The conclusion that falls out: **put the docs steps in the existing `ci` job.** One check name, no `paths` filter, no `needs` graph, no second context to register. The cost is that a docs-only PR also compiles Rust; with `Swatinem/rust-cache` that is a cache hit, and it is cheaper than any of the four traps above.

## Recommended gate set

Appended to the existing `ci` job in `.github/workflows/ci.yml`, after `cargo nextest run`:

```yaml
      - run: bash scripts/check-docs
      - run: cargo run -p <emitter> -- --write   # ticket 09 names the emitter
      - run: git diff --exit-code -- docs/openapi
      - uses: oasdiff/oasdiff-action/validate@v0.1.15
        with: { spec: docs/openapi/loads.yaml }
      - run: git fetch --depth=1 origin ${{ github.base_ref }}
      - uses: oasdiff/oasdiff-action/breaking@v0.1.15
        with:
          base: origin/${{ github.base_ref }}:docs/openapi/loads.yaml
          revision: HEAD:docs/openapi/loads.yaml
          fail-on: ERR
          review: false
      - uses: lycheeverse/lychee-action@v2.9.0
        with:
          args: --offline --include-fragments --no-progress './**/*.md'
          fail: true
```

And the rule that makes them bite, in `.github/rulesets/protect-main.json`, where `rulesets.yml` already proves the live ruleset matches the file:

```json
{
  "type": "required_status_checks",
  "parameters": {
    "strict_required_status_checks_policy": false,
    "required_status_checks": [{ "context": "ci" }, { "context": "check" }]
  }
}
```

Rejected, with the reason: Vale and any prose linter (taste, and a full-time technical-writing function is what makes GitLab's Vale job affordable); markdownlint (formatting, not truth); LLM review of prose (ruled out by the ticket, and non-deterministic gates lose author trust first and merge power second); `typos` (mechanical and installable through the existing install-action step, but a dictionary gate needs an allowlist the moment freight vocabulary lands, so leave it off the floor); rustdoc `broken_intra_doc_links` (right tool for `cargo doc`, wrong surface for a Markdown handbook).

## Fit to hive

- Chapter 8 already says **"Additive JSON only"** ([REST](../../../docs/adr/0009-rest.md)). Gate 4 mechanizes a rule law has already written; it does not introduce policy. That is the strongest fit argument in this note, and it is why the breaking-change gate belongs in the floor rather than the wish list.
- Chapter 2 says boundary violations are detected "in design review" and "No hop-count CI" ([architecture.md](../../../docs/architecture.md)). Keeping prose and structure judgment out of CI follows the same line law already drew: CI proves mechanical facts, review carries taste.
- ADR 0001 pins tools, not only crates: `cargo-deny` 0.20.2, `cargo-nextest` 0.9.144, `cargo-llvm-cov` 0.9.1. Adopting oasdiff and lychee adds rows to that table and to the majors table in `docs/architecture.md`. **This reopens ADR 0001**, the same ADR the map already expects to reopen for the OpenAPI crate. Fold the tool pins into that one law change instead of a second one.
- ADR 0001's "Not pinned" list currently reads "No OpenAPI crate", and chapter 8 repeats "No OpenAPI crate pin". Gate 2 cannot exist until that line moves, so the CI spec depends on ticket 09 landing first. That ordering is already encoded in [ticket 12](../issues/12-ci-gate-contract.md)'s blockers.
- Chapter 10 declares "80% line coverage per cell is CI law via cargo-llvm-cov. CI fails the cell below 80%", and `ci.yml` does not run `cargo-llvm-cov`. Law already declares a gate that CI does not implement. Whatever the docs gate spec says, it should land the workflow change and the law change in the same PR, or it will add a second entry to that list.
- Nothing here needs CODEOWNERS, which is out of scope. `required_status_checks` is independent of the `pull_request` rule already in the ruleset.
- Nothing here contradicts law. The one law touch is the ADR 0001 pin table, named above.

## Sources

- [oasdiff](https://github.com/oasdiff/oasdiff) — subcommand semantics (`diff`, `changelog`, `breaking`, `validate`), git-revision inputs, release `v1.31.0`.
- [oasdiff-action](https://github.com/oasdiff/oasdiff-action) — `breaking` / `changelog` / `diff` / `validate` sub-actions, `fail-on`, `review`, `allow-external-refs`, base-vs-`github.base_ref` workflow, release `v0.1.15`.
- [lychee](https://github.com/lycheeverse/lychee) — `--offline`, `--include-fragments`, `--root-dir`, `.lycheeignore`, exit codes 0/1/2/3, release `lychee-v0.24.2`.
- [lychee-action](https://github.com/lycheeverse/lychee-action) — scheduled run with `fail: false` plus create-issue-from-file, `lycheeVersion`, `.lycheecache` caching, release `v2.9.0`.
- [GitHub Docs, Troubleshooting required status checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks) — skipped-but-required stays Pending, conditional skip reports Success, `needs` skip may not block, `merge_group`, latest-SHA and seven-day rules.
- [GitHub Docs, Available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets) — required status checks, strict versus loose.
- [GitHub Docs, REST API endpoints for repository rules](https://docs.github.com/en/rest/repos/rules) — `required_status_checks` and `workflows` rule JSON shapes.
- [git-diff](https://git-scm.com/docs/git-diff) — `--exit-code` exits 1 on differences, 0 on none.
- [sqlx-cli README](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) — `cargo sqlx prepare --check` exits nonzero when the committed artifact is stale.
- [GitLab, Documentation testing](https://docs.gitlab.com/development/documentation/testing/) — `graphql-verify` generated-file drift job, `docs-lint redirects` and `docs-i18n-lint paths` existence jobs, link jobs skip network links, i18n link job does not fail the pipeline.
- [taiki-e/install-action TOOLS.md](https://github.com/taiki-e/install-action/blob/main/TOOLS.md) — supported tools include `cargo-deny`, `cargo-nextest`, `typos`, `vacuum`; oasdiff and lychee are not listed.
- Repo state read on 2026-09-12: `.github/workflows/ci.yml`, `.github/workflows/rulesets.yml`, `.github/rulesets/protect-main.json`, `scripts/sync-rulesets`, `docs/architecture.md`, `docs/adr/0001-stack-pins.md`, `docs/adr/0009-rest.md`.

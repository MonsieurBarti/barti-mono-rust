# How do CI gates enforce documentation without becoming theater?

Type: research
Status: resolved
Label: wayfinder:research
Blocked by:

## Question

What merge gates actually keep docs true to the code, without blocking on prose taste?

This repo's CI today runs `fmt`, `clippy`, `cargo deny`, and `nextest`. There is no docs job.

Cover missing files, OpenAPI drift from handlers, and broken links. Not LLM review of prose.

Primary sources: GitHub Actions, OpenAPI diff tools, link checkers, and docs-as-code CI docs.

Recommend a small gate set for the spec.

Asset: `.scratch/hive-docs/research/04-ci-documentation-gates.md`

## Answer

Five blocking checks, all mechanical, all appended as steps to the existing `ci` job: a bash `scripts/check-docs` existence and orphan check, an OpenAPI drift check that regenerates the committed spec and runs `git diff --exit-code`, `oasdiff validate`, `oasdiff breaking` against the base ref with `fail-on: ERR`, and `lychee --offline --include-fragments`. External link rot runs weekly and does not block; `oasdiff changelog` goes to the job summary.

The gate that makes those real is not a job. `.github/rulesets/protect-main.json` has no `required_status_checks` rule, so every job in `ci.yml` is advisory today and a docs job added without that rule is theater by construction. One job means one check name and none of GitHub's skipped-but-required, conditional-skip, or `needs` traps. `oasdiff breaking` mechanizes chapter 8's "Additive JSON only" rather than inventing policy; prose taste stays in review, matching chapter 2's "No hop-count CI". The one law touch is the ADR 0001 pin table, which gains oasdiff and lychee rows alongside the OpenAPI crate pin that ticket 09 already reopens.

Findings: [../research/04-ci-documentation-gates.md](../research/04-ci-documentation-gates.md)

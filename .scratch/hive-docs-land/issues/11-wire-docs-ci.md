# Wire the documentation CI gates

Type: task
Label: wayfinder:task
Blocked by: 01, 08, 09, 10
Status: resolved

## Question

Put the five documentation checks on the existing `ci` job.

Run `scripts/check-docs`. Run `cargo run -p app --bin openapi`, then `git diff --exit-code -- docs/openapi/openapi.json`. Run `oasdiff validate` on that file. Run `oasdiff breaking` against `origin/${{ github.base_ref }}:docs/openapi/openapi.json` with `fail-on: ERR` and `review: false`. Skip breaking when the path is missing on the base ref. Run `lychee --offline --include-fragments` on the spec's path list.

`.github/rulesets/protect-main.json` gains `required_status_checks` for `ci` and `check`, with `strict_required_status_checks_policy` false.

Use the versions [What versions do we pin for oasdiff and lychee?](01-oasdiff-lychee-pins.md) records.

## Answer

The existing `ci` job runs the five documentation checks.
`scripts/check-docs` is path equality.
`cargo run -p app --bin openapi` regenerates `docs/openapi/openapi.json`, then `git diff --exit-code` fails on drift.
`oasdiff/oasdiff-action/validate@v0.1.15` validates that file.
`oasdiff/oasdiff-action/breaking@v0.1.15` compares `origin/${{ github.base_ref }}:docs/openapi/openapi.json` with `fail-on: ERR` and `review: false`.
The breaking step skips when the path is missing on the base ref.
`lycheeverse/lychee-action@v2.9.0` runs `--offline --include-fragments` on the spec path list.
`.github/rulesets/protect-main.json` requires `ci` and `check` with `strict_required_status_checks_policy` false.

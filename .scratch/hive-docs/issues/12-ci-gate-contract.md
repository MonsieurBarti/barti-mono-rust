# What does the CI gate require?

Type: grilling
Status: resolved

Label: wayfinder:grilling
Blocked by: 04, 06, 09

## Question

Which documentation checks block merge?

The checks must be mechanical. Missing handbook page, OpenAPI drift, and broken links are in play. Prose taste is not.

## Answer

Five blocking checks, all mechanical, all steps on the existing `ci` job. `.github/rulesets/protect-main.json` gains `required_status_checks` for `ci` and `check`, with `strict_required_status_checks_policy` false. Prose taste is not a gate. Page length is not a gate.

`scripts/check-docs` is path equality. Cell name is the last directory of `crates/cells/*/*/Cargo.toml`. It requires `docs/handbook/README.md`, `docs/handbook/kernel.md`, `docs/handbook/app.md`, repo-root `CONTEXT-MAP.md`, `docs/handbook/cells/<cell>.md` for every cell crate, `<crate>/CONTEXT.md` for every cell crate, and that `CONTEXT-MAP.md` contains the relative path to that `CONTEXT.md`. A handbook cell page with no crate fails. Named-only `CONTEXT-MAP.md` rows without links stay legal. `docs/handbook/` may contain only those files. `docs/openapi/` may contain only `openapi.json`.

OpenAPI drift: a generator in `app` writes `docs/openapi/openapi.json`. Pretty JSON. Stable keys. Nobody edits it by hand. CI regenerates, then `git diff --exit-code -- docs/openapi/openapi.json`.

`oasdiff validate` runs on that file.

`oasdiff breaking` compares `origin/${{ github.base_ref }}:docs/openapi/openapi.json` to `HEAD:docs/openapi/openapi.json`, `fail-on: ERR`, `review: false`. Skip when the path is missing on the base ref.

`lychee --offline --include-fragments` runs on `docs/handbook/**/*.md`, `docs/architecture.md`, `docs/adr/**/*.md`, `CONTEXT.md`, `CONTEXT-MAP.md`, and `crates/cells/**/CONTEXT.md`. Not `.scratch/`. Not `AGENTS.md`.

Two cargo tests ride the existing nextest step. Coverage instantiates each cell router `app` merges: every operation in that cell OpenApi is tagged with the cell name; the merged document contains those operations; a cell with no public routes contributes no tag and passes; `GET /health` is absent from the spec. Problem identity: before merge, among cell OpenApis that emit `components.schemas.Problem`, that schema JSON is identical.

External link rot does not block. `oasdiff changelog` may write the job summary. No weekly workflow on this map. Vale, markdownlint, LLM prose review, `typos`, rustdoc intra-doc links, and handbook-to-OpenAPI coupling are not gates.

ADR 0001 gains oasdiff, oasdiff-action, lychee, and lychee-action rows. Exact versions are the implementation pin. Same law change as the OpenAPI crate pin. This map does not add the workflow. [Write the cell-docs and OpenAPI spec](14-write-the-spec.md) writes this contract into the spec.

## Comments

### Round 1

Five arrows accepted.

Q1 A: cell crate listing. Every `crates/cells/*/*/Cargo.toml` has `docs/handbook/cells/<cell>.md`. Orphan page fails. Named-only graph nodes do not fail.

Q2 A: also require `docs/handbook/README.md`, `kernel.md`, `app.md`, repo-root `CONTEXT-MAP.md`, a cell `CONTEXT.md` for every cell crate, and a `CONTEXT-MAP.md` row for every cell crate. Named-only rows without links stay legal. OpenAPI existence stays on drift.

Q3 A: tagged-operation coverage and Problem identity are cargo tests, run by nextest. `scripts/check-docs` stays path equality. Drift is regenerate then `git diff --exit-code -- docs/openapi/openapi.json`.

Q4 A: `oasdiff breaking` against `origin/${{ github.base_ref }}:docs/openapi/openapi.json`, `fail-on: ERR`, `review: false`. Skip when the path is missing on the base ref.

Q5 A: `lychee --offline --include-fragments` on `docs/handbook/**/*.md`, `docs/architecture.md`, `docs/adr/**/*.md`, `CONTEXT.md`, `CONTEXT-MAP.md`, and `crates/cells/**/CONTEXT.md`. Not `.scratch/`. Not `AGENTS.md`.

### Round 2

Five arrows accepted.

Q6 A: closed trees. `docs/handbook/` only `README.md`, `kernel.md`, `app.md`, and `cells/<cell>.md` for cell crates. `docs/openapi/` only `openapi.json`.

Q7 A: coverage instantiates each cell router `app` merges. Every operation in that cell OpenApi is tagged with the cell name. The merged document contains those operations. No public routes: no tag, pass. `GET /health` is absent from the spec.

Q8 A: before merge, among cell OpenApis that emit `components.schemas.Problem`, that schema JSON is identical.

Q9 A: ADR 0001 gains oasdiff, oasdiff-action, lychee, and lychee-action rows. Exact versions are the implementation pin.

Q10 A: external link rot does not block. `oasdiff changelog` may write the job summary. No weekly workflow on this map.

### Round 3

Q11 A accepted. Draft recorded. Ticket closed.




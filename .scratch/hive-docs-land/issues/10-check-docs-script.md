# Add scripts/check-docs

Type: task
Label: wayfinder:task
Blocked by: 03, 04
Status: resolved


## Question

Add `scripts/check-docs` as path equality.

Cell name is the last directory of `crates/cells/*/*/Cargo.toml`. Require `docs/handbook/README.md`, `docs/handbook/kernel.md`, `docs/handbook/app.md`, repo-root `CONTEXT-MAP.md`, `docs/handbook/cells/<cell>.md` for every cell crate, `<crate>/CONTEXT.md` for every cell crate, and that `CONTEXT-MAP.md` contains the relative path to that `CONTEXT.md`. A handbook cell page with no crate fails. Named-only `CONTEXT-MAP.md` rows without links stay legal.

Closed trees. `docs/handbook/` may contain only those files. `docs/openapi/` may contain only `openapi.json`.

Do not add the CI workflow step.

## Answer

`scripts/check-docs` is path equality. Cell name is the last directory of `crates/cells/*/*/Cargo.toml`. It requires `docs/handbook/README.md`, `docs/handbook/kernel.md`, `docs/handbook/app.md`, repo-root `CONTEXT-MAP.md`, and per cell crate a `docs/handbook/cells/<cell>.md`, a `<crate>/CONTEXT.md`, and that crate path inside `CONTEXT-MAP.md`. Named-only rows stay legal. The closed trees fail an orphan handbook page, any other file under `docs/handbook/`, and any file under `docs/openapi/` other than `openapi.json`. No CI workflow step.

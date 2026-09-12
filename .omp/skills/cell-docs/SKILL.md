---
name: cell-docs
description: Update a cell handbook, cell CONTEXT.md, CONTEXT-MAP.md row, and public OpenAPI annotations when public REST, Open Host, a cell handbook page, a cell glossary, or a new cell changes. Kernel and app pages only when those crates change.
---

# Keep cell docs current

Diff is the working tree against `origin/main`. Match `.scratch/hive-docs/spec.md` and `docs/architecture.md` chapter 8.

A changed cell updates four files: `docs/handbook/cells/<cell>.md`, `<crate>/CONTEXT.md`, that cell's `CONTEXT-MAP.md` row, and public OpenAPI annotations. A new cell takes the same four. Kernel and `app` pages only when those crates change. Drop root `CONTEXT.md` terms that now live in a cell glossary.

This skill writes those files. CI, prose lint, and the site generator stay elsewhere.

If this checkout is a worktree, read `skill://omp-worktree-absolute-paths` and prefix every path and command.

## 1. Scope

`git diff --name-only origin/main`. Add untracked paths from `git status --short`.

A cell is in scope when the diff touches its Open Host, public REST, `docs/handbook/cells/<cell>.md`, or `<crate>/CONTEXT.md`.

A new `crates/cells/<domain>/<cell>/Cargo.toml` is in scope.

Kernel is in scope when the diff touches `crates/kernel/`. `app` is in scope when the diff touches `crates/app/`.

Skip when none of those. Chat `cell-docs: skip`.

Done when the skip line is in chat, or the in-scope list exists.

## 2. Handbook

Read `.scratch/hive-docs/spec.md` Writing and Cell page template. Read the in-scope crate's Open Host, leaving SPIs, and schema.

Write `docs/handbook/cells/<cell>.md` to that template and those edges.

A new cell also gets a link in `docs/handbook/README.md`.

When kernel is in scope, write `docs/handbook/kernel.md` from the spec Kernel and app template. When `app` is in scope, write `docs/handbook/app.md` from that template.

Done when each in-scope page matches the spec template and the crate's current edges.

## 3. Glossary

Skip when the in-scope list is only kernel or `app`.

Write `<crate>/CONTEXT.md`: title, one or two sentences, then `## Language` with `**Term**` / definition / `_Avoid_`.

Delete those terms from root `CONTEXT.md`.

Done when each in-scope cell glossary exists in that shape and root `CONTEXT.md` no longer defines those terms.

## 4. Context map

Skip when the in-scope list is only kernel or `app`.

A context row is the cell name, one purpose clause, and a link to that `CONTEXT.md`. Add or update the in-scope cell's row. Add a relationship row only when the crate emits or receives a hop.

Done when `CONTEXT-MAP.md` lists each in-scope cell with a link to its `CONTEXT.md`.

## 5. OpenAPI annotations

Skip when the in-scope list is only kernel or `app`.

Read `docs/architecture.md` chapter 8.

Public REST handlers on an in-scope cell carry `#[utoipa::path]`. The tag is the cell name. `operationId` is unique across cells. Public REST uses `routes!(handler)` with no turbofish.

Published Language in `domain/api` derives `ToSchema` beside `Deserialize`, `Serialize`, and `Validate`. Presentation owns `Problem`. Cells document the statuses they choose.

Done when each public REST handler on an in-scope cell is annotated and tagged, and each PL struct in that cell's `domain/api` derives `ToSchema`.

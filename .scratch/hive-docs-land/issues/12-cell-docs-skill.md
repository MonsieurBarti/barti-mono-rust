# Write the cell-docs skill and wire ship-ticket

Type: task
Label: wayfinder:task
Blocked by: 03, 04, 07
Status: resolved


## Question

Write `.omp/skills/cell-docs/` with writing-for-agents.

The skill is model-invoked. When a cell's public surface changes, it updates that cell's handbook page, `CONTEXT.md`, the `CONTEXT-MAP.md` row, and public OpenAPI annotations so they match [the locked spec](../../hive-docs/spec.md) and chapter 8. A new cell takes the same four. Kernel and `app` pages only when those crates change. It may drop terms from root `CONTEXT.md` that now live in a cell glossary. It does not run CI, lint prose, or build a site.

Point `AGENTS.md` at it. Add a numbered `ship-ticket` step that runs it when the ticket touches public REST, Open Host, a cell handbook page, a cell glossary, or a new cell. Otherwise the step is done. Map `.scratch/hive-docs-land/` to effort-short `docs-land`. `review-change` stays Law and Ticket.

## Answer

Wrote `.omp/skills/cell-docs/`. It is model-invoked. `ship-ticket` step 4 runs it on public REST, Open Host, a cell handbook page, a cell glossary, or a new cell. Effort-short `docs-land` maps `.scratch/hive-docs-land/`. `review-change` stays Law and Ticket.

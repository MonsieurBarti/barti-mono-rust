# How do DDD codebases keep a context map and per-context glossaries?

Type: research
Status: resolved
Label: wayfinder:research
Blocked by:

## Question

How do mature DDD codebases keep a context map and per-bounded-context glossaries without turning them into narrative docs?

This repo: [CONTEXT.md](../../../CONTEXT.md) is glossary only. [docs/agents/domain.md](../../../docs/agents/domain.md) gates `CONTEXT-MAP.md` on the first cell language. Freight terms currently live in root `CONTEXT.md`. First cells are `loads`, `shipments`, and `settlement`.

Primary sources: Evans, Vernon, DDD-Crew context mapping, and at least one public codebase that keeps a context map.

Recommend what `CONTEXT-MAP.md` lists, what a cell `CONTEXT.md` holds, and what stays in root `CONTEXT.md`.

Asset: `.scratch/hive-docs/research/05-context-map-glossaries.md`

## Answer

Three files, none of them a handbook. Root `CONTEXT.md` keeps hive process and kernel types. `CONTEXT-MAP.md` lists cells, glossary paths, and contact points (pattern + `Book` / local read model), not how-it-works. Each cell `CONTEXT.md` holds that cell's terms only; `Shipper` and `Carrier` are restated per cell. Freight nouns leave root when the first cell glossary lands. Do not write `CONTEXT-MAP.md` until cell crates exist (chapter 16 / ADR 0017). Findings: [../research/05-context-map-glossaries.md](../research/05-context-map-glossaries.md).

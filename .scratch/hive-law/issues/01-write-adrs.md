# Write ADRs 0002 through 0017

Type: task
Status: resolved

Label: wayfinder:task
Blocked by:

## Question

One ADR per chapter of [docs/architecture.md](../../../docs/architecture.md). Chapter N is `docs/adr/00(N+1)-<slug>.md` with the slugs from the map's name contract.

Format is `skill://domain-modeling/ADR-FORMAT.md`: title, then one to three sentences of context, decision, and why. Add `## Considered options` only when the chapter's grill rejected a real alternative; name the alternative and the reason in one line each. Add `## Consequences` only for a downstream effect the chapter does not state. Link the chapter and the grill ticket at the end. Style matches [Stack pins](../../../docs/adr/0001-stack-pins.md): STE, no hedging.

The ADR holds the why. The chapter holds the mechanics. Never copy a chapter table or list into an ADR. Where the chapter and the grill disagree, the chapter wins; note the disagreement in the answer.

In `docs/architecture.md`, each chapter's `Grill:` line gains ` · ADR: [<title>](adr/00NN-<slug>.md)`. Line 5 stays as written.

Fan the sixteen out to subagents by chapter if useful. One integration pass for voice and cross-links.

Acceptance: sixteen files exist; each is under forty lines; every chapter links its ADR; `docs/agents/domain.md` needs no change. No test lane runs.

## Answer

Sixteen ADRs landed: `docs/adr/0002-language.md` through `0017-freight.md`. Each is under forty lines. Each chapter `Grill:` line links its ADR. Line 5 of `docs/architecture.md` is unchanged. `docs/agents/domain.md` is unchanged. No test lane ran.

Chapter wins over grill:

- Unpublished ports: grill 07 said GraphQL-only, MCP-only, and tick. Chapter 3 says tick, webhook, and REST-only.
- Chapter 1 replacements: grill 06 listed schema and cell role. Chapter 1 Died/Lives maps collection to table only.
- Unit clock: grill 12 said a cell-local fake until the kernel ticket. Chapter 10 uses kernel `FakeClock`.
- Application integration home: grill 12 Question put it under cell `tests/`. Chapter 10 puts Cargo `tests/` as e2e.
- Logger/Metrics: grill 17 put them under each cell `domain/spi/`. Chapter 11 exports kernel ports and forbids per-cell trait files.

`## Considered options` landed on Composition root, Communication, REST, and Workers.

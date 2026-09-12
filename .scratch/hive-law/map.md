# Hive law as ADRs, rules, and skills

Label: wayfinder:map

## Destination

`docs/adr/0002` through `0017`, one per `docs/architecture.md` chapter. A compile wall that catches every forbidden rustc can see. `.omp/rules/` with one rule per layer, one short always-apply rule, and at most six TTSR interrupts for the rest. `.omp/skills/ship-ticket` and `.omp/skills/review-change`, each behind an `.omp/prompts/` command, proven once by shipping and reviewing boot ticket 09.

## Notes

- Domain: greenfield Rust hive-strict. Host product: B2B freight brokerage. Law is [docs/architecture.md](../../docs/architecture.md), [Stack pins](../../docs/adr/0001-stack-pins.md), and [CONTEXT.md](../../CONTEXT.md). Do not re-grill it. Do not rewrite it.
- This map implements. Each ticket lands files on `main` through a PR. Branch `hive-law/NN-<slug>`, worktree `~/.omp/wt/barti-hive-law-NN`, conventional commits, push with hooks. Follow `skill://omp-worktree-absolute-paths` inside the worktree.
- Skills every session: domain-modeling for ADR format, writing-for-agents for every rule, skill, and prompt file. Tracker: `docs/agents/issue-tracker.md`.
- Naboo skills (`cell-hexagonal`, `cell-conformance-audit`, `naboo-*`, `code-review`) are never a source. Hive law is only what the three law files say.
- Locked at charting. ADRs stay in `docs/adr/`; `.omp/` holds `rules/`, `skills/`, and `prompts/` only. One ADR per chapter; chapter N is ADR `00(N+1)`. The ADR holds context, decision, why, and rejected alternatives. The chapter keeps the mechanics. Rules link ADRs and never restate them. Rules are one per layer plus one short always-apply rule. TTSR interrupts are capped at six patterns. The compile wall lands first; a rule the compiler enforces is deleted. The ship skill claims, worktrees, runs tdd, gates, self-reviews, and resolves the ticket inside the PR. The review skill has two axes, Law and Ticket, reads the layer rules as its checklist, reports in chat, and posts only on request.
- Name contract. ADRs: `0002-language`, `0003-cell`, `0004-composition-root`, `0005-communication`, `0006-cqrs`, `0007-persistence`, `0008-event-sourcing`, `0009-rest`, `0010-validation`, `0011-testing`, `0012-observability`, `0013-authentication`, `0014-workers`, `0015-published-language-scalars`, `0016-shared-kernel`, `0017-freight`. Rules: `hive` (always-apply), `hive-domain`, `hive-application`, `hive-infrastructure`, `hive-presentation`, `hive-app`, `hive-kernel`, `hive-tests`, `hive-migrations`; TTSR files are `ttsr-<forbidden>`. Skills: `ship-ticket`, `review-change`. Prompts: `.omp/prompts/ship-ticket.md` and `.omp/prompts/review-change.md`, same names as the skills, same shape as `~/.omp/agent/prompts/review-fintech.md`.
- Gate is CI's four commands: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans`, `cargo nextest run --workspace`. The db lane runs when compose Postgres is up.

## Decisions so far
- [Write ADRs 0002 through 0017](issues/01-write-adrs.md) — one ADR per chapter; each Grill line links it.
- [Harden the compile wall](issues/02-harden-the-compile-wall.md) — unreachable_pub, clippy now(), deny per-cell comment; module-grain stays prose.
- [Write the layer rules](issues/03-write-the-layer-rules.md) — nine rule files; layer MUST/NEVER plus AGENTS.md Hive law.
- [Write the TTSR interrupts](issues/04-write-the-ttsr-interrupts.md) — four files; dropped tracing, now(), and runtime query().
- [Write the ship-ticket skill and command](issues/05-ship-ticket-skill.md) — skill plus `/ship-ticket` prompt; local-attack after the gate; `ttsr-sea-orm` replaces `ttsr-sqlx`.
- [Write the review-change skill and command](issues/06-review-change-skill.md) — skill plus `/review-change` prompt; Law and Ticket via task; dogfood on boot PR 6.

## Not yet specified

- A whole-cell conformance audit, every file rather than a diff. May fall out of `review-change` once the layer rules exist.
- OMP pre-edit hooks as a blocking tier above TTSR. Wait for TTSR false-positive data from the dogfood.

## Out of scope

- Changing a decision in `docs/architecture.md`, ADR 0001, or `CONTEXT.md`.
- Boot-map tickets other than 09 as the dogfood.
- A CI job that runs `review-change` on every PR.
- Per-cell `CONTEXT.md` and `CONTEXT-MAP.md`. `docs/agents/domain.md` gates that on the first cell language.
- Linear. Naboo production code.

# Write the layer rules

Type: task
Label: wayfinder:task
Blocked by: 01, 02

## Question

`.omp/rules/hive.md` with `alwaysApply: true`, and eight layer rules with `globs` and `description`, names from the map's contract.

`hive.md` is short: the cell is the unit; the glossary is `CONTEXT.md`; a table of chapter to ADR; read the layer rule for the file you edit. Under thirty lines.

Each layer rule is that layer's MUST and NEVER lines, drawn from the chapters, each line naming its ADR by title with a link. Globs: `hive-domain` `crates/cells/**/src/domain/**`; `hive-application` `crates/cells/**/src/application/**`; `hive-infrastructure` `crates/cells/**/src/infrastructure/**`; `hive-presentation` `crates/cells/**/src/presentation/**`; `hive-app` `crates/app/**`; `hive-kernel` `crates/kernel/**`; `hive-migrations` `crates/cells/**/migrations/**`. `hive-tests` covers `crates/**/tests/**`; unit and integration tests sit inside layer files, so the lane law for those goes in a `Tests` section of `hive-domain` and `hive-application`. `domain/api` is part of `hive-domain` with its own section: the only place serde and garde are legal inside the hexagon.

Omit every forbidden that [Harden the compile wall](02-harden-the-compile-wall.md) moved to the compiler. Never restate an ADR; one line plus the link.

Frontmatter is YAML `parseFrontmatter` accepts: `description`, `globs` as a list. See `omp://rulebook-matching-pipeline.md` §2 and §6. Rule files are documents for agents; follow `skill://writing-for-agents`.

`AGENTS.md` gains a `### Hive law` section: three lines pointing at `docs/architecture.md` plus `docs/adr/`, `.omp/rules/`, and the two skills.

Acceptance: nine rule files; every glob matches an existing path or a chapter-named path; a fresh omp session lists the eight layer rules in its rulebook and `rule://hive-domain` resolves; no rule line duplicates another rule's line.

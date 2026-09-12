# What are the test lanes, and what does each one boot?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 09

## Question

Port naboo chapter 12 to Rust. Packaging is decided: unit tests live inside the cell crate and see `pub(crate)`, integration tests under the cell's `tests/` see only the re-export set, and `app` tests boot the binary. Tests construct a cell with fake SPIs and never boot the composition root.

Decide: how many lanes, what each lane boots, which lane owns the sqlx adapter against a real schema, whether a fake SPI is a hand-written struct or a crate, the runner and its invocation, and whether a per-cell coverage floor is law.

No Nest harnesses exist to port. Postgres isolation must already be decided.

## Answer

Three lanes per cell, named by layer. Cargo homes differ from lane names.

**Unit** is domain. `#[cfg(test)]` next to the unit. Entities, value objects, errors, codec and persistence-mapper round-trips. No I/O. No doubles. Fluent builder next to the entity: `.build()` reconstructs, `.build_new()` creates, faker fills unused fields, never assert on faker output. Tests never call `SystemTime::now` or `Utc::now`. A cell-local fake clock is legal until [What lives in the kernel crate?](20-kernel-crate.md) owns one.

**Integration** is application. `#[cfg(test)]` inside the cell crate, in `integration` modules so nextest can exclude them from the unit profile. Direct `new` of the use-case with this cell’s real sqlx adapter on the cell-role pool. Handwritten fakes for leaving SPIs. No HTTP. No `app`. This lane owns the sqlx adapter against a real schema.

**E2E** is presentation. Cell `tests/` sees only the re-export set. Hits `new` + `router` with tower. Real sqlx, fake leaving SPIs. Never boots `app`. One command and one query per public use-case that has an HTTP handler. No handler unit tests. Unpublished REST is case by case, not a floor.

Cargo `tests/` is E2E, not application integration. Application tests must see `pub(crate)` sqlx types.

GRANT proof lives in `crates/app/tests`. Open two cell-role pools. A planted cross-schema `query` expects SQLSTATE `42501`. Not a product E2E. Not a coverage cell. Cell crates never name another cell’s tables. Cross-cell workflow tests are not a lane.

Always-on Postgres. Compose locally, CI service. Per-worker database. Production schema names, migrator LOGIN plus cell LOGIN, ops-shaped GRANTs. Fail fast if unreachable, before db lanes. Tests never start Postgres. Testcontainers is out. `#[sqlx::test]` is out.

A fake is a handwritten struct next to the SPI, `#[cfg(test)]`. Not InProc. mockall is not law. No workspace fake crate. An in-memory own-store exists only so the SPI contract has two adapters. Unit has no doubles.

A contract is a `pub` function next to the SPI, behind cell feature `contract`, off by default. Every adapter invokes it: fake and sqlx adapter from cell `#[cfg(test)]`; InProc from `app` tests with the feature enabled on a `dev-dependencies` path. Prod `cargo build` does not enable it. Not a spec.

Minima: every command ≥1 happy path and ≥1 error (integration). Every domain invariant a rejection test (unit). Every persistence mapper a round-trip (unit). Every SPI method on that port’s contract.

Runner is cargo-nextest 0.9. Invocation is `cargo nextest run`. `cargo test` is a local escape hatch, not the gate. Unit profile needs no DSN and excludes `integration::` plus the `tests/` target. Db profile runs integration, e2e, and app GRANT and requires worker databases.

80% line coverage per cell is CI law via cargo-llvm-cov. Merge unit + integration + e2e. Exclude tests, fakes, builders, and contract modules. Kernel and `app` have no floor. CI fails the cell below 80%. The floor follows the cell, not the domain folder.

Event-capture shape waits on [How does CQRS work inside a Rust cell?](14-cqrs-inside-a-cell.md). Minor and patch pins wait on [Write the stack-pin ADR](22-stack-pin-adr.md).

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Fake**, **Builder**, **Contract**.


## Comments

### Round 1

Accepted:

- Q1: three lanes by layer, not by cargo home. Unit = domain. Integration = application (full). E2E = presentation, case by case. Glossary **cell**, not domain folder. Not cross-cell tests.
- Q2 A: always-on hive Postgres. No testcontainers. No `#[sqlx::test]`.
- Q3 A: handwritten `#[cfg(test)]` struct next to the SPI. mockall out. No fake crate. Unit has no doubles. Leaving-SPI fakes live in integration and e2e only.
- Q4 A: cargo-nextest 0.9, `cargo nextest run`.
- Q5 A: 80% line per cell, cargo-llvm-cov, CI law. Kernel and `app` have no floor.

### Round 2

Four arrows accepted:

- Q6 A: integration constructs the use-case with this cell’s real sqlx adapter on the cell-role pool. Leaving SPIs are handwritten fakes. No HTTP. No `app`. In-memory own-store exists only so the SPI contract has two adapters.
- Q7 A: e2e in `tests/` hits `new` + `router`. Real sqlx, fake leaving SPIs, never `app`. One command and one query per public use-case that has an HTTP handler. No handler unit tests. Unpublished REST is case by case.
- Q8 A: GRANT `42501` lives in `crates/app/tests`.
- Q9 A: per-worker database. Production schema names, two LOGINs, ops-shaped GRANTs.

### Round 3

Four arrows accepted:

- Q10 A: every command happy+error, every invariant a rejection, every mapper a round-trip, every SPI method on the contract.
- Q11 A: `pub` contract behind cell feature `contract`. Fake + sqlx in the cell crate; InProc in `app` tests.
- Q12 A: `#[cfg(test)]` builder next to the entity. No `now()` in tests. Cell-local fake clock until the kernel ticket.
- Q13 A: fail fast if Postgres is down. Unit profile needs no DSN. Db profile requires worker databases.

# How is a cell packaged, and what is the composition root without Nest?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 01, 05

## Question

Port naboo chapters 2 and 3 to Rust.

The hive word for packaging is crate, not module. Decide workspace layout: crate-per-cell vs one crate with Rust mods; where the composition-root binary lives; how Open Host tokens are exported; how leaving SPIs are bound; the compile-time import wall.

Research tickets [What is the current SOTA Rust HTTP stack for a hive composition root?](../issues/01-sota-http-stack.md) and [How should a Cargo workspace express one crate per cell, a kernel crate, and a composition-root binary?](../issues/05-sota-cargo-workspace-cells.md) feed this grill.

## Answer

Virtual workspace. `crates/kernel` rlib, `crates/cells/<domain>/<cell>` one rlib per cell, `crates/app` bin. A directory under `crates/cells/` with no `Cargo.toml` is a domain folder. Package name is the bare cell name. Intra-cell layers are modules: `domain/` (with `api/` and `spi/`), `application/`, `infrastructure/`, `presentation/`.

InProc adapters live in `app`, not in the consumer crate. No cell crate ever path-depends on another cell, so there is no legal cell-to-cell edge to allow-list. `app` layout: `main.rs`, `wiring/<cell>.rs`, `inproc/<consumer>/<provider>.rs`, `http.rs`, `telemetry.rs`. No aggregator crate or module that re-exports cells.

`lib.rs` re-exports Open Host API-port traits, their Published Language types, the leaving SPI traits, and `new`. Writes may sit in the Open Host set. GraphQL-only, MCP-only, and tick operations stay `pub(crate)`. Leaving SPIs bind as generic type parameters on the cell struct, not `Arc<dyn _>`: AFIT traits are not dyn-compatible and one in-process adapter does not earn a vtable and a box per hop.

Import wall: rustc crate privacy, `pub(crate)`, `unreachable_pub`, and cargo-deny `bans.deny` with `wrappers = ["app"]` per cell crate. Reject cell to cell, cell to `app`, kernel to cell. Features are not a wall.

Cell identity tests port from naboo: too-small, too-big, chatty, and the remedy order (fatten Open Host, then local read model, then merge). Multi-aggregate allowed. Team ownership is not identity. Hop-count is review-only. Neighbour, legacy bridge, and promote tests drop.

Tests construct the cell with fake SPIs. They never boot `app`.

Glossary correction: [CONTEXT.md](../../../CONTEXT.md) **InProc adapter** is composition-root infrastructure, not consumer-crate infrastructure.

## Comments

### Round 1

Six arrows accepted: workspace paths; ported cell identity tests; InProc in `app` (option A) over consumer-crate InProc or a per-edge `*-inproc` crate; rustc plus cargo-deny wall; `app` as the only crate depending on cells; Open Host versus `pub(crate)` unpublished ports.

InProc placement was the live tension. Naboo puts InProc in consumer `infrastructure/inproc/`, which in Cargo would make the consumer crate path-depend on the provider and hand every module in that crate a legal `use`. Rust cannot confine that import to one module, so the adapter moves to the composition root.

### Round 2

Six arrows accepted: generics for leaving SPIs; the shortened allow/reject list; the `CONTEXT.md` InProc rewrite; the four in-cell layer modules; the `app` internal layout; bare cell name as package name.

Consequence of Q3=A: naboo's "consumer inproc → provider `domain/api`" allow line does not get ported. It has no Cargo edge left to permit.

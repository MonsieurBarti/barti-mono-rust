# How should a Cargo workspace express one crate per cell, a kernel crate, and a composition-root binary?

Type: research
Status: resolved
Label: wayfinder:research
Blocked by:

## Question

What is the current state of the art for a Rust cargo workspace that maps naboo hive packaging onto crates: one crate per cell, a framework-free kernel crate, and one composition-root binary that binds leaving SPIs?

Constraints:

- Latest stable versions only. Rust edition current stable.
- Cover: crate-type (lib vs bin), visibility (`pub(crate)` vs public API = Open Host only), how to forbid cell A application from importing cell B, `cargo deny` / crate-graph lint vs feature flags, DI without Nest (nject, shaku, inventory, hand-rolled composition root).
- Naboo law to port: cell module never names a provider cell; only the composition root binds InProc; Open Host is the exported API-port set.
- Recommend a workspace layout and a compile-time wall tool. Pin majors.

Asset: `.scratch/rust-hive/research/05-sota-cargo-workspace-cells.md`

## Answer

Virtual workspace, edition 2024, rustc 1.98.1. One rlib per cell, kernel rlib, app bin. Hand-rolled DI. Wall: rustc privacy + cargo-deny 0.20.2. Findings: [05-sota-cargo-workspace-cells.md](../research/05-sota-cargo-workspace-cells.md).

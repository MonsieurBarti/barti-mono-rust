# How is a cell packaged, and what is the composition root without Nest?

Type: grilling
Status: open
Label: wayfinder:grilling
Blocked by: 01, 05

## Question

Port naboo chapters 2 and 3 to Rust.

Decide: crate-per-cell vs module-per-cell; where the composition-root binary lives; how Open Host tokens are exported; how leaving SPIs are bound; the compile-time import wall.

Research tickets [What is the current SOTA Rust HTTP stack for a hive composition root?](../issues/01-sota-http-stack.md) and [How should a Cargo workspace express one crate per cell, a kernel crate, and a composition-root binary?](../issues/05-sota-cargo-workspace-cells.md) feed this grill.

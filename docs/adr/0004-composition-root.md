# Composition root

`crates/app` is the only crate that imports cells, because a consumer-crate InProc adapter would give every module in that crate a legal `use` of the provider. Leaving SPIs bind as generics on the cell struct: AFIT traits are not dyn-compatible, and one in-process adapter does not earn a vtable and a box per hop.

## Considered options

- InProc in the consumer crate: Cargo cannot confine the provider import to one module.
- Per-edge `*-inproc` crate: a third crate per hop for one adapter.
- `Arc<dyn _>` for leaving SPIs: AFIT ports are not dyn-compatible.

Chapter: [3. Composition root](../architecture.md#3-composition-root)

Grill: [How is a cell packaged, and what is the composition root without Nest?](../../.scratch/rust-hive/issues/07-cell-packaging-and-composition-root.md)

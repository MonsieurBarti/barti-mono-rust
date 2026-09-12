# Shared kernel

`crates/kernel` is a library, not a cell: it has no Open Host and no exclusive datastore. Kernel depends on `time` only, so it stays framework-free. Kernel omits Envelope, Id, and CorrelationId so each cell owns its edge types.

Chapter: [15. Shared kernel](../architecture.md#15-shared-kernel)

Grill: [What lives in the kernel crate?](../../.scratch/rust-hive/issues/20-kernel-crate.md)

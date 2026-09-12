# kernel

Kernel holds shared types and ports every cell may use. It is not a cell. It has no Open Host and no schema.

## How it works

- Money and Instant — shared value objects
- Clock — application reads now
- Logger and Metrics — bound on cell `new`
- Violation — payload on `VALIDATION_FAILED`

Cells import these. Kernel never imports a cell. See [Shared kernel](../architecture.md#15-shared-kernel).

## See also

- [Shared kernel](../architecture.md#15-shared-kernel)
- [Shared kernel ADR](../adr/0016-shared-kernel.md)
- [CONTEXT.md](../../CONTEXT.md)

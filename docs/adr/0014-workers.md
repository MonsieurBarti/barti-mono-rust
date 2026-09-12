# Workers

Ticks are unpublished presentation adapters. Once-work uniqueness is the cell work table, so a queue crate and a second worker binary stay out. `app` spawns ticks and work drains so the cell does not spawn at `new`.

## Considered options

- pgmq, apalis, sqlxmq: shared schema and framework tables break cell schema isolation.
- Second worker binary: HTTP, ticks, outbox drains, and work drains already share one `app` process.

Chapter: [13. Workers](../architecture.md#13-workers)

Grill: [Where do ticks, queues, and workers live?](../../.scratch/rust-hive/issues/18-workers-and-ticks.md)

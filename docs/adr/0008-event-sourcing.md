# Event sourcing

Event sourcing is per-aggregate opt-in because temporal queries do not pick storage. The stream is the write model only where a write must be reversible without silent edit, so that aggregate keeps no state row beside the stream. Application uses the same `get_by_id` / `save` write SPI so it does not branch on storage.

Chapter: [7. Event sourcing](../architecture.md#7-event-sourcing)

Grill: [Do we event-source, where, and what is the contract?](../../.scratch/rust-hive/issues/15-event-sourcing.md)

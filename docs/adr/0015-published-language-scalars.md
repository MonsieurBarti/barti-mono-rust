# Published Language scalars

PL Money is JSON `{ amount, currency }` in integer minor units so kernel `Money` never hits the wire. Ids are opaque strings that application mints as UUIDv7 for new aggregates. Instant and calendar date keep distinct encodings so a date is never midnight UTC.

Chapter: [14. Published Language scalars](../architecture.md#14-published-language-scalars)

Grill: [How are Money, ids, and dates encoded in Published Language?](../../.scratch/rust-hive/issues/19-pl-scalars.md)

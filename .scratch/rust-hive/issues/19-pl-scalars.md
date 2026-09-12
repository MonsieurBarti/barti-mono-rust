# How are Money, ids, and dates encoded in Published Language?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 04, 06

## Question

Pin Published Language encodings for Money, ids, instants, calendar dates, and Clock.

Locked: Published Language is JSON primitives; value objects stay inside the hexagon; codec is serde + garde. Glossary: [What ubiquitous language does the Rust hive keep](06-rust-hive-glossary.md). Research: [What is the current SOTA Rust approach to Published Language validation at a cell edge?](04-sota-cell-edge-validation.md).

Decide: Money on the wire and in the kernel; ids; instants; calendar dates; Clock. Re-justify from this hive, not from a port.

Do not write the chapter here.

## Answer

PL Money is `{ amount, currency }`. `amount` is a JSON number, integer minor units, inside `±(2^53-1)`. `currency` is ISO 4217 alpha-3, uppercase. Signed amounts including zero are legal. No exponent. No IEEE fractional amount. No `bigint` on the wire. Unknown ISO code is `VALIDATION_FAILED`. Product allowlists stay out. Processor exponent quirks stay in that cell's vendor adapter.

Kernel `Money` is `i64` minor units (same cap) plus ISO currency. Same-currency `add`, `subtract`, `compare`. `allocate(weights)` only; remainder pennies to the first recipients; parts sum to the original. No `divide` that returns one `Money`. No FX method. `create` rejects unknown ISO and out-of-range amounts. `to_plain()` is `{ amount: i64, currency: String }` and is not serde. Kernel stays serde-free. ISO 4217 fraction digits live in kernel as data.

Domain, application, and this cell's sqlx adapter MAY import kernel `Money` and `Instant`. Presentation, InProc, and `domain/api` MUST NOT. `domain/api` MAY import the ISO 4217 table. Application calls `Money::create` / `to_plain()` and formats instants. Decode never names `Money`.

Ids are opaque non-empty strings. New aggregates mint UUIDv7, lowercase, hyphenated (RFC 9562), via `uuid` 1.26. Kernel has no generic `Id`. A cell MAY wrap. Reconstruct takes the stored string. Application mints. Domain does not call `Uuid::now_v7`.

Instant PL is ISO-8601 datetime. Offset input is legal. Encode UTC `Z` with milliseconds. Decode requires seconds; fractional seconds are allowed. Nanoseconds are not on the wire. Kernel `Instant` is a newtype over `time::OffsetDateTime`.

Calendar date is `YYYY-MM-DD` on the wire and as the domain string. A cell MAY wrap. Never encode as midnight UTC.

Kernel `Clock` with `now()` returning `Instant`. `app` binds the real clock. Application passes that instant into the entity. Domain never reads the clock. Tests use a fake clock.

Postgres: `amount BIGINT`, `currency TEXT`, id `UUID`, instant `TIMESTAMPTZ`, calendar date `DATE`. Never `NUMERIC` for amount. Never `TIMESTAMPTZ` for a calendar date. A corrupt own row is a server bug, not `VALIDATION_FAILED`.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Money**, **Instant**, **Calendar date**, **Clock**.

## Comments

### Round 1

Five arrows accepted (user: lgtm):

- Q1 A: PL Money is `{ amount, currency }`. `amount` is a JSON number, integer minor units, inside `±(2^53-1)`. `currency` is ISO 4217 alpha-3, uppercase. Signed amounts including zero are legal. No exponent. No IEEE fractional amount. No `bigint` on the wire. Unknown ISO code is `VALIDATION_FAILED`. Product allowlists stay out. Processor exponent quirks stay in that cell's vendor adapter.
- Q2 A: Ids are opaque non-empty strings. New aggregates mint UUIDv7, lowercase, hyphenated (RFC 9562), via `uuid` 1.26. Kernel has no generic `Id`. A cell MAY wrap. Reconstruct takes the stored string. No ObjectId. No `coerceId`.
- Q3 A: Instant PL is ISO-8601 datetime. Offset input is legal. Encode UTC `Z` with milliseconds. Decode requires seconds; fractional seconds are allowed. Nanoseconds are not on the wire.
- Q4 A: Calendar date is `YYYY-MM-DD` on the wire and as the domain string. A cell MAY wrap. Never encode as midnight UTC.
- Q5 A: Kernel `Clock` with `now()` returning the instant domain type. `app` binds the real clock. Application passes that instant into the entity. Domain never reads the clock. Tests use a fake clock.

### Round 2

Five arrows accepted (user: lgtm), then the port framing was rejected.

- Q6 A: Kernel `Money` is `i64` minor units (same `±(2^53-1)` cap) plus ISO currency. Same-currency `add` / `subtract` / `compare`. `allocate(weights)` only. No `divide`. No FX. `to_plain()` is not serde. ISO 4217 fraction digits are kernel data.
- Q7 A: Domain, application, and this cell's sqlx adapter MAY import kernel `Money` and `Instant`. Presentation, InProc, and `domain/api` MUST NOT. `domain/api` MAY import the ISO 4217 table. Decode never names `Money`.
- Q8 A: Kernel `Instant` over `time::OffsetDateTime`. Calendar date stays a domain `String`.
- Q9 A: `amount BIGINT`, `currency TEXT`, id `UUID`, instant `TIMESTAMPTZ`, calendar date `DATE`.
- Q10 A: Application mints UUIDv7. Domain does not call `Uuid::now_v7`.

### Round 3

Q11 A accepted (user: lgtm). Map Notes stripped of source law.

Q12 and Q13 were accepted then challenged: the user wants something they can actually use. Destination (spec-only vs bootable) and host product are reopened.

### Round 4

Q14 A applied, then rejected (user: don't redraw destination, go with what is planned).

Destination stays the charted spec: locked `docs/architecture.md` and stack-pin ADR, named first cells, hand off to implementation. No product code on this map. Q15 host product stays B2B freight brokerage.

### Round 5

Q16 A accepted (user: y). Draft recorded. Glossary written. Ticket closed.

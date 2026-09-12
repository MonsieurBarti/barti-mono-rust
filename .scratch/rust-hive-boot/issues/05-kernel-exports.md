# Export the kernel crate

Type: task
Status: resolved

Label: wayfinder:task
Blocked by: 04

## Question

Fill `crates/kernel` with the closed export set in architecture chapter 15.

Export `Money`, `Instant` plus `checked_add_seconds`, `Clock`, `SystemClock`, `FakeClock`, `is_known_currency`, `fraction_digits`, `Violation`, `Logger`, and `Metrics`. Crate dep is `time` only. Serde, axum, sqlx, tracing, and uuid stay out. No Envelope, Id, CorrelationId, or ActorId.

Unit tests next to the types. No I/O. Kernel has no coverage floor. Use tdd.

Do not touch `app` or `loads` beyond the path dep the skeleton already has.

## Answer

Kernel rlib exports the chapter 15 set. The only crate dep is `time` 0.3.55.

`Money::create`, `add`, `subtract`, `compare`, and `allocate` return `Option`. `to_plain` is `(amount, currency)`. Amounts sit in `±(2^53-1)`. ISO 4217 table A.1 is SIX list-one published 2026-01-01. Metals and special codes are known and have no fraction digits.

`Instant` is a UTC `OffsetDateTime` newtype. `from_unix_timestamp` constructs without cells taking `time`. `FakeClock` stores through `Mutex`. Tests never call `SystemClock::now`.

`Logger` and `Metrics` are `Send + Sync` traits. `Violation` is a public struct. No Envelope, Id, CorrelationId, or ActorId.

Unit tests sit next to the types. `cargo test -p kernel --lib` is 33 tests. App and loads are untouched.

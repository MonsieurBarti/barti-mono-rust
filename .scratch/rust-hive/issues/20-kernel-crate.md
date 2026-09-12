# What lives in the kernel crate?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 07, 19

## Question

Port naboo chapter 18. Hex platform kit does not exist.

Locked: `crates/kernel` is a framework-free rlib; kernel to cell is illegal. Packaging: [How is a cell packaged, and what is the composition root without Nest?](07-cell-packaging-and-composition-root.md). Money, ids, dates, and Clock must already be decided: [How are Money, ids, and dates encoded in Published Language?](19-pl-scalars.md). Logger and Metrics implementations live in `app` as generics; cells declare the SPIs; domain never logs: [How do we observe a REST hive process?](17-observability.md).

Decide: what the kernel crate exports; what stays cell-local; Logger and Metrics as kernel port types versus cell SPIs.

Do not write the chapter here.

## Answer

Kernel is a framework-free rlib of shared types and ports. It never depends on a cell. It stays serde-free, axum-free, sqlx-free, tracing-free, uuid-free. The only crate dep is `time`.

Exports exactly:

- `Money` as locked in [How are Money, ids, and dates encoded in Published Language?](19-pl-scalars.md)
- `Instant`, plus `checked_add_seconds(i64) -> Option<Instant>`
- `Clock` with `now() -> Instant`
- `SystemClock`
- `FakeClock` with `new(Instant)`, `set(Instant)`, `now()`
- `is_known_currency(&str) -> bool` and `fraction_digits(&str) -> Option<u8>`
- `Violation { path, code, message }` as a public struct, no serde
- `Logger`: `debug` / `info` / `warn` / `error`, each `msg: &str` plus `fields: &[(&str, &str)]`
- `Metrics`: `increment(name, value: u64, tags)` and `distribution(name, value: f64, tags)`

`Clock`, `Logger`, and `Metrics` are `Send + Sync`, synchronous, non-throwing, bound as generics on cell `new`. Not `dyn`. No per-cell trait files under `domain/spi/`. That tightens [How do we observe a REST hive process?](17-observability.md): these two are kernel ports bound as leaving SPIs, not cell-written traits.

`app` binds `SystemClock` and the Logger/Metrics implementations. Tests pass `FakeClock` and fake Logger/Metrics. Domain never logs, never counts, never reads the clock.

Forbidden in kernel: Envelope, Id, CorrelationId, ActorId, a Result alias, outbox helpers, pagination, problem+json, a Level enum, a Currency enum, macros, gauge.

Layer imports:

- Money / Instant — domain, application, this cell’s sqlx adapter; not presentation, InProc, or `domain/api`
- Clock trait — application only
- Logger / Metrics — application, infrastructure, presentation; not inner domain, not `domain/api`
- Violation — `domain/api` only
- ISO functions — `domain/api` may; Money::create uses them inside kernel
- `SystemClock` / `FakeClock` — constructed in `app` and tests only

Each port envelope variant holds `Vec<Violation>` and serdes at the cell edge.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Kernel**, **Logger SPI**, **Metrics SPI**, **Violation**.

## Comments

### Round 1

Four arrows accepted (user: lgtm):

- Q1 A: Kernel exports `Logger` and `Metrics` traits. Cell `new` is generic over them. No per-cell trait files.
- Q2 A: Kernel exports serde-free `Violation { path, code, message }`. Each port envelope variant holds `Vec<Violation>`.
- Q3 A: Kernel exports `Clock`, `SystemClock`, and `FakeClock`.
- Q4 A: Neither CorrelationId nor ActorId lives in kernel.

### Round 2

Four arrows accepted (user: lgtm):

- Q5 A: Logger is four methods `debug` / `info` / `warn` / `error`, `msg` plus `fields: &[(&str, &str)]`. No level enum. No macros.
- Q6 A: Metrics is `increment` and `distribution`. No gauge. App impl adds `cell:`.
- Q7 A: Clock trait — application only. Logger/Metrics — application, infrastructure, presentation. Violation — `domain/api` only. `SystemClock` / `FakeClock` — `app` and tests.
- Q8 A: Closed export set. Crate dep is `time` only.

### Round 3

Two arrows accepted (user: lgtm):

- Q9 A: Instant has `checked_add_seconds(i64) -> Option<Instant>`. `FakeClock::new(Instant)`, `set`, `now()`.
- Q10 A: Private ISO table. Public `is_known_currency` and `fraction_digits`.

### Round 4

Q11 A accepted. Draft recorded. Glossary written. Ticket closed.


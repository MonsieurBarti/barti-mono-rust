# Where does the codec validate versus domain invariants?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 04, 10

## Question

Port chapter 11 to serde and garde.

Locked: serde + garde; value objects stay inside the hexagon; both sides of InProc encode and decode. Research: [What is the current SOTA Rust approach to Published Language validation at a cell edge?](04-sota-cell-edge-validation.md). Public REST: [What is the public driving-adapter surface?](10-public-driving-adapters.md).

Decide: where decode and encode run; that domain entities and handlers import no codec crate; that infrastructure MAY parse a foreign wire; that persistence does not re-validate this cell's own rows.

Do not pick Money, ids, or dates here. Do not write the chapter here.

## Answer

`domain/api/<use-case>/` owns input PL (`Deserialize` + garde + `deny_unknown_fields`), output PL (`Serialize`, no deny), `decode`, and the envelope enum. The use-case's first act is `decode(input)`. `decode` is the only function that names garde. It returns `Result<PlInput, Envelope>`. `Valid<T>` never leaves that module. Inner domain and application never `use serde` or `use garde`. Application maps `PlInput` to value objects, returns output PL, and maps domain errors to the envelope. Presentation serializes success and maps `serde_path_to_error` on HTTP JSON into the same `VALIDATION_FAILED` / `violations[]` document, then skips the port. InProc has no serde step. It still calls `decode`.

Codec failures are shape and field constraints on PL primitives, including cross-field rules that need no entity. Entity invariants, VO `TryFrom`, and uniqueness against a loaded row are domain or SPI and never `VALIDATION_FAILED`. sqlx does not garde-parse this cell's own rows. A corrupt own row is a server bug. Infrastructure MAY serde+garde a foreign wire into SPI types. That failure is not `VALIDATION_FAILED`. Webhook: signature on the raw body, then this cell's input PL, then `decode`. A serde-to-violations helper in kernel waits on [What lives in the kernel crate?](20-kernel-crate.md).

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Codec**.

## Comments

### Round 1

Four arrows accepted:

- Q1 A: inner domain imports neither serde nor garde. Application calls `decode`/`encode` only. Presentation may serde HTTP JSON into PL, then the port still decodes. sqlx does not garde-parse this cell's own rows; a corrupt own row is a server bug, not `VALIDATION_FAILED`. Infrastructure MAY serde+garde a foreign wire, then map into this cell's types. Webhook: signature in the driving adapter, then this cell's PL into the unpublished port, then `decode`.
- Q2 A: codec is JSON shape plus field constraints on PL primitives, including cross-field rules that need no entity. Failure is `VALIDATION_FAILED`. Domain invariants and VO `TryFrom` throw domain errors. Uniqueness against a loaded row is domain or SPI.
- Q3 A: `#[serde(deny_unknown_fields)]` on input PL only. Output PL structs do not deny. Do not share one struct for both if that would put deny on an output.
- Q4 A: the port takes unvalidated PL. `decode` in `domain/api` is the only function that names garde. It returns `Result<PlInput, Envelope>`. `Valid<T>` never leaves that module.

### Round 2

Three arrows accepted:

- Q5 A: presentation maps `serde_path_to_error` into `VALIDATION_FAILED` with `violations[]` and does not call the port. `decode` runs garde on a typed struct. InProc has no serde step.
- Q6 A: foreign-wire parse failure is an SPI/infrastructure error. The use-case maps it to a typed envelope, never `VALIDATION_FAILED`.
- Q7 A: the use-case returns output PL and maps domain errors to the per-port envelope. No `encode()` that serializes JSON. Serde serialize is presentation.

### Round 3

Q8 A: record that draft, close the ticket.

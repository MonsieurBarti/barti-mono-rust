# SOTA Rust Published Language validation at a cell edge

As of 2026-09-11. Latest stable crates.io versions only.

## Verdict

Pin this stack beside each API port:

| Crate                 | Version | Role                                        |
| --------------------- | ------- | ------------------------------------------- |
| `serde`               | 1.0.229 | Shape encode/decode of PL primitive structs |
| `serde_json`          | 1.0.151 | JSON value/string form of the same PL       |
| `serde_path_to_error` | 0.1.20  | Field path on shape failures                |
| `garde`               | 0.23.0  | Constraint decode after shape parse         |

There is no Zod crate. Rust’s type is the schema. `serde` is the bidirectional codec. `garde` is the semantic gate Zod’s `.parse()` adds on top of TypeScript types.

Decode at the provider API port:

1. Deserialize into `garde::Unvalidated<PlInput>`.
2. Call `validate()` and obtain `garde::Valid<PlInput>`.
3. Map `serde` errors and `garde::Report` into `{ type: "VALIDATION_FAILED", context }`.

Encode at the same port: `Serialize` the PL output struct. Encode domain rejections as `{ type, context }` with a `SCREAMING_SNAKE` `type` and primitive `context`. Do not ship `thiserror` types across cells.

Consumer InProc does the inverse. It maps SPI types onto the provider PL structs, calls the API port, then validates and maps the reply. After extract, the same structs ride `serde_json`. In-process hops do not JSON-roundtrip.

Value objects stay inside the hexagon. Domain and application import neither `serde` nor `garde`. Shared-kernel types do not appear on PL structs.

Default PL containers use `#[serde(deny_unknown_fields)]`. Unknown JSON keys fail decode.

## Compared

| Approach                  | Latest stable                                | Why it lost / why it sits beside the winner                                                                                                                                                                                                                                                                                                        |
| ------------------------- | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **serde + serde_json**    | 1.0.229 / 1.0.151                            | Winner for encode/decode. Implements `Serialize`/`Deserialize` without runtime reflection. Maps JSON primitives onto Rust structs. Does not check email, length, range, or pattern.                                                                                                                                                                |
| **garde**                 | 0.23.0                                       | Winner for constraints. Full rewrite of `validator`. Rules are traits. Derive covers enums. `Unvalidated<T>` / `Valid<T>` is a compile-time proof of validation. `Report` is a flat `(Path, Error)` list. Error messages use constraint parameters only. They do not attach submitted values.                                                      |
| **validator**             | 0.21.0                                       | Still maintained. Larger download count. Nested errors are a tree, not a flat report. `ValidationError` auto-inserts the field `value` into `params`. That fights hive “no submitted values” on `VALIDATION_FAILED`. No `Valid<T>` wrapper. Garde exists because this crate’s author asked for a rewrite.                                          |
| **serde_valid**           | 3.1.2                                        | 2026 JSON-Schema-on-serde option. `from_json_value` does deserialize-plus-validate. Errors are nested JSON Schema trees (`errors` / `properties`), not `{ type, context }`. Couples PL to the JSON Schema vocabulary. Hive law is the Rust PL type beside the port, not a JSON Schema document.                                                    |
| **schemars + jsonschema** | 1.2.2 / 0.56.0                               | Generator plus runtime JSON Schema validator. Draft 2020-12. Useful later to _emit_ MCP/OpenAPI schema from the same PL types. A second runtime gate duplicates serde. `jsonschema` validates `serde_json::Value`, not a typed port. `valico` 4.0.0 last published 2023-05-13. `jsonschema-valid` 0.5.2 last published 2023-11-08. Both are stale. |
| **nutype**                | 0.7.0 (0.8.0-beta.2 is pre-release)          | Guaranteed newtypes. `try_new` and serde `Deserialize` refuse invalid inners. That is a value object, not PL. Putting nutypes on the API port leaks VOs onto the wire. Keep nutype inside the hexagon. Application maps PL primitives through `TryFrom`.                                                                                           |
| **typestate crate**       | 0.8.0 stable; 0.9.0-rc2 from 2021-09-02      | Proc-macro DSL for object protocols (`#[automaton]`, `#[state]`). Last crates.io activity 2021. Wrong layer. Aggregate lifecycle may use handwritten typestate inside domain. Cell-edge PL is runtime data, not a compile-time traffic light. Garde’s `Valid<T>` already covers the only typestate the edge needs.                                 |
| **facet / facet-json**    | 0.46.5 / 0.46.1 (0.50.0-rc.7 is pre-release) | 2026 serde competitor (compile-time reflection). Not 1.0. Do not pin a hive on an rc.                                                                                                                                                                                                                                                              |
| **validify**              | 2.0.0 (2025-02-09)                           | Validate-and-modify derive. Quiet. Small download count. Adds mutation (sanitize-in-place) that hive decode should not do at the edge.                                                                                                                                                                                                             |

`thiserror` 2.0.20 stays in-cell for domain errors. The port encodes those errors into the envelope. Cells never `downcast` a neighbour error.

## Fit to hive

Rust maps that 1:1.

**Cell / Open Host.** Each use-case under `domain/api/<use-case>/` owns the PL structs, the `garde` derives, and the `decode`/`encode` functions. The API port trait takes and returns those PL types, or `Result<PlOut, Envelope>`. Presentation and InProc call that trait. The application use-case implements it. Adapters never implement an API port.

**PL types.** Fields are JSON primitives: `String`, `bool`, `i64`, nested structs, `Vec<_>`, stringly enums. Money on the wire is `{ amount, currency }`, not a kernel `Money` class. Ids are opaque strings. Instant is an ISO-8601 string. The later scalars ticket owns the exact shapes. This ticket only forbids VOs and kernel types on the port.

**Decode / encode.** Provider decode: shape (`serde`) then constraints (`garde`). Provider encode: `Serialize` output and errors. Consumer InProc encode: map consumer SPI → provider PL. Consumer InProc decode: validate the reply, then map PL → SPI. No `instanceof` across cells. After extract, swap the InProc hop for HTTP/gRPC and keep the same structs.

**Envelope.** Shape and constraint failures become `type: "VALIDATION_FAILED"` with `context.violations: [{ path, code, message }]`. `serde_path_to_error` fills `path` for shape errors. `garde::Report` is already a flat path list. Domain rejections become a `SCREAMING_SNAKE` `type` plus primitive `context` (example: `{ type: "LOAD_NOT_FOUND", context: { loadId: "…" } }`). Public HTTP/GraphQL may hide `context`. InProc must not.

**SPI.** Consumer owns SPI types under `domain/spi/`. Those types are not PL. Only `infrastructure/inproc/<provider-cell>/` maps PL. Application, domain, and presentation import no foreign cell.

**Composition root.** It binds leaving SPIs. It does not own codecs. Shared PL helpers (money/id/instant) live next to the composition root and are imported only by `domain/api/`, presentation, and InProc. Entities, value objects, and application cannot import them.

**Inside the hexagon.** Application maps `Valid<PlInput>` to a command. That is where `Money::try_from` and nutype `try_new` run. Domain stays codec-free.

**Garde `Valid<T>` vs nutype.** `Valid<PlInput>` proves the edge ran. It still holds primitives. A nutype `LoadId(String)` is a VO. Keep the first on the port. Keep the second in domain.

## Sources

- https://crates.io/crates/serde (1.0.229, 2026-07-18)
- https://crates.io/crates/serde_json (1.0.151, 2026-07-20)
- https://crates.io/crates/serde_path_to_error (0.1.20, 2025-09-15)
- https://crates.io/crates/garde (0.23.0, 2026-05-23)
- https://crates.io/crates/validator (0.21.0, 2026-07-27)
- https://crates.io/crates/serde_valid (3.1.2, 2026-09-10)
- https://crates.io/crates/schemars (1.2.2, 2026-07-27)
- https://crates.io/crates/jsonschema (0.56.0, 2026-09-10)
- https://crates.io/crates/nutype (0.7.0 stable, 0.8.0-beta.2 pre-release)
- https://crates.io/crates/typestate (0.8.0 stable, 0.9.0-rc2 on 2021-09-02)
- https://crates.io/crates/facet (0.46.5 stable, 0.50.0-rc.7 pre-release)
- https://crates.io/crates/facet-json (0.46.1)
- https://crates.io/crates/validify (2.0.0, 2025-02-09)
- https://crates.io/crates/valico (4.0.0, 2023-05-13)
- https://crates.io/crates/jsonschema-valid (0.5.2, 2023-11-08)
- https://crates.io/crates/thiserror (2.0.20, 2026-08-08)
- https://serde.rs/
- https://serde.rs/container-attrs.html (`deny_unknown_fields`)
- https://docs.rs/serde/1.0.229/serde/
- https://docs.rs/serde_json/1.0.151/serde_json/
- https://docs.rs/serde_path_to_error/0.1.20/serde_path_to_error/
- https://docs.rs/garde/0.23.0/garde/
- https://docs.rs/garde/0.23.0/garde/struct.Valid.html
- https://docs.rs/garde/0.23.0/garde/struct.Unvalidated.html
- https://docs.rs/garde/0.23.0/garde/error/struct.Report.html
- https://docs.rs/validator/0.21.0/validator/
- https://github.com/Keats/validator/blob/master/README.md
- https://github.com/jprochazk/garde/blob/master/README.md
- https://github.com/Keats/validator/issues/201 (garde rewrite origin)
- https://docs.rs/serde_valid/3.1.2/serde_valid/
- https://docs.rs/schemars/1.2.2/schemars/
- https://graham.cool/schemars/
- https://docs.rs/jsonschema/0.56.0/jsonschema/
- https://json-schema.org/draft/2020-12/json-schema-validation
- https://docs.rs/nutype/0.7.0/nutype/
- https://docs.rs/typestate/0.8.0/typestate/
- https://docs.rs/thiserror/2.0.20/thiserror/

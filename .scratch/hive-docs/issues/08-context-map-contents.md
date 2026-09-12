# What belongs in CONTEXT-MAP.md and per-cell CONTEXT.md?

Type: grilling
Label: wayfinder:grilling
Blocked by: 05
Status: resolved

## Question

What does `CONTEXT-MAP.md` list, and what terms move from root `CONTEXT.md` into a cell `CONTEXT.md`?

First cell language is `loads`. `shipments` and `settlement` may stay named-only on the map. `CONTEXT.md` stays a glossary. No how-it-works.

## Answer

`CONTEXT-MAP.md` lives at repo root. It has two headings. No mermaid. No Open Host list. No SPI list. No REST. No kernel node. No `app` node. No domain-folder node. No term dump.

```markdown
# Context Map

## Contexts

- [loads](./crates/cells/freight/loads/CONTEXT.md) — Shipper posted demand to move freight from origin to destination
- shipments — contracted move after a Load is booked with a Carrier
- settlement — Invoice to the Shipper and Payable to the Carrier

## Relationships

- **loads → shipments**: loads emits `Book` (integration event, Published Language). shipments receives it on its own API port. Consumer SPI plus InProc is the anticorruption layer.
- **shipments → settlement**: settlement keeps a local read model of CustomerRate and CarrierRate locked on the Shipment at book.
```

A context row is the cell name, one purpose clause, and a link to that cell `CONTEXT.md` when the file exists. `shipments` and `settlement` are named-only: purpose clause, no link, no empty glossary, no invented crate.

A relationship row names the contact and the hive words for the pattern. Shared Kernel stays off this file. Money and Instant live in root `CONTEXT.md` and [Shared kernel](../../../docs/adr/0016-shared-kernel.md).

`loads` `CONTEXT.md` lives at `crates/cells/freight/loads/CONTEXT.md`. Title, one or two sentences of what this context is, then `## Language` with `**Term**` / definition / `_Avoid_`. Terms: Load, Quote, Stop, Consignee, Shipper (the party that posts), Carrier (the party that quotes). Quote is in even though the crate has no Quote aggregate yet. Shipper and Carrier are restated here. This file does not hold Shipment, Invoice, Payable, CustomerRate, or CarrierRate. It does not list Open Host. It does not write invariant essays.

Root `CONTEXT.md` keeps hive process and kernel. When the `loads` glossary lands, `### Freight` drops Load, Quote, Stop, Consignee, Shipper, and Carrier. It keeps Shipment, CustomerRate, CarrierRate, Invoice, and Payable until those cell glossaries exist. Then the heading goes.

Write these files with writing-for-agents. One source of truth. Pointers, not copies. Positive definition first. `_Avoid_` is the synonym guardrail.

This map does not write the files. [Write the cell-docs and OpenAPI spec](14-write-the-spec.md) proposes them. Chapter 16 still gates the repo-root map on cell crates.


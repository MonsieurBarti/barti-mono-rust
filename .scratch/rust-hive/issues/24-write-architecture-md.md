# Write the architecture document

Type: task
Status: resolved
Label: wayfinder:task
Blocked by: 22, 23

## Question

Write the locked `docs/architecture.md` named in the destination.

Assemble hive law from resolved tickets and [Write the stack-pin ADR](22-stack-pin-adr.md). This doc restates majors only. Minors and patches stay in the ADR.

Wait for [How are mutating REST commands made idempotent?](23-rest-command-idempotency.md) so the REST chapter can include that law.

Path: `docs/architecture.md`. Do not implement product code. Do not rewrite the ADR.

## Answer

Wrote [docs/architecture.md](../../../docs/architecture.md). Sixteen chapters. Majors only. Minors stay in [ADR 0001](../../../docs/adr/0001-stack-pins.md).

Dropped GraphQL, MCP, legacy promotion, and Appendix A. REST includes `Idempotency-Key`. Freight names `loads`, `shipments`, `settlement`.

No product code. ADR unchanged.


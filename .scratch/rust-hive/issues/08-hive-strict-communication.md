# How do cells communicate in Rust hive-strict?

Type: grilling
Status: open
Label: wayfinder:grilling
Blocked by: 03, 04, 07

## Question

Port naboo chapter 4.

Decide: consumer-owned SPI traits; InProc adapters; Published Language codecs; envelope `{ type, context }`; request/response default vs integration events; no application import of a foreign cell.

Research tickets [What is the current SOTA Rust approach to in-process CQRS, domain events, and a transactional outbox on Postgres?](../issues/03-sota-cqrs-events-outbox.md) and [What is the current SOTA Rust approach to Published Language validation at a cell edge?](../issues/04-sota-cell-edge-validation.md) feed this grill. Packaging must already be decided.

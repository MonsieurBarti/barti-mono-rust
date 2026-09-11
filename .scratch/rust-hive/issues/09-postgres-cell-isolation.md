# How is Postgres isolation enforced per cell?

Type: grilling
Status: open
Label: wayfinder:grilling
Blocked by: 02, 07

## Question

Port naboo chapter 6 from Mongo users/collections to Postgres.

Decide: schema-per-cell vs database-per-cell; one role per cell; named pool per cell; no cross-cell transaction; how a planted import of another cell's tables fails at compile time and at runtime.

Research ticket [What is the current SOTA Rust Postgres stack for exclusive per-cell schemas and roles?](../issues/02-sota-postgres-isolation.md) feeds this grill. Packaging must already be decided.

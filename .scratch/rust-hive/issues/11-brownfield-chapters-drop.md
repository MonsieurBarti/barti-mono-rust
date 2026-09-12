# Which naboo chapters drop as brownfield-only?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 06

## Question

The glossary killed neighbour, legacy bridge, promote, slice, hex platform kit, Nest module, AppModule, mongoose, sheriff, and Mongo collection.

Decide which naboo architecture chapters, or chapter sections, this repo's `docs/architecture.md` must omit as brownfield-only. Decide which remaining chapters this map still has to grill.

Do not write those chapters here. Source law: `/Users/pierrelecorff/Projects/naboo/docs/architecture.md`.

## Answer

Drop as chapters: 8 Presentation (GraphQL), 10 MCP, 14 Legacy promotion, and Appendix A.

Keep 9 REST. The error catalog and suffix map already live on [What is the public driving-adapter surface?](10-public-driving-adapters.md).

In every kept chapter, omit paragraphs whose only job is neighbour, promote, slice, legacy bridge, hex platform kit, Nest module / `AppModule` / `forRootAsync`, mongoose / Atlas / Mongo user, GraphQL Args or ObjectType twins, MCP, sheriff / dependency-cruiser, Linear grill links, or `wome-api` path targets. Keep the hive law those paragraphs wrapped.

This map still grills CQRS, event sourcing, validation, observability, workers, Published Language scalars, the kernel crate, and freight language plus first cells. [What are the test lanes, and what does each one boot?](12-test-lanes.md) and [How does AuthN/AuthZ work at the REST process edge?](13-authn-rest-edge.md) stay. The stack-pin ADR is a final task. Writing `docs/architecture.md` is the handoff after tickets are empty.

No further grill: Language, Cell, Composition root, Communication, Persistence, REST.

## Comments

### Round 1

Both arrows accepted.

- Q1: Drop chapters 8 Presentation (GraphQL), 10 MCP, 14 Legacy promotion, and Appendix A. Keep 9 REST. Move the error catalog and suffix map into REST. Ticket 10 already holds that catalog.
- Q2: In every kept chapter, omit paragraphs whose only job is neighbour, promote, slice, legacy bridge, hex platform kit, Nest module / `AppModule` / `forRootAsync`, mongoose / Atlas / Mongo user, GraphQL Args or ObjectType twins, MCP, sheriff / dependency-cruiser, Linear grill links, or `wome-api` path targets. Keep the hive law those paragraphs were wrapping.

### Round 2

Q3 arrow accepted.

- New grill tickets for CQRS, event sourcing, validation, observability, workers, Published Language scalars, the kernel crate, and freight language plus first cells.
- [What are the test lanes, and what does each one boot?](12-test-lanes.md) and [How does AuthN/AuthZ work at the REST process edge?](13-authn-rest-edge.md) stay.
- Stack-pin ADR is a final task on this map.
- Writing `docs/architecture.md` is the handoff after tickets are empty.



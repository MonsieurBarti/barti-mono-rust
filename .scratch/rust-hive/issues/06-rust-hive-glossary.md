# What ubiquitous language does the Rust hive keep from naboo, and what Nest/Mongo terms die?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 01, 02, 03, 04, 05

## Question

Pin the chapter-1 glossary for `docs/architecture.md` in this repo.

Keep cell, domain folder, Open Host, API port, SPI, Published Language, InProc, kernel, hive-strict, envelope `{ type, context }`. Decide Rust replacements for Nest module, mongoose adapter, Zod codec, `AppModule`, sheriff/dependency-cruiser, and Mongo collection.

Write `CONTEXT.md` terms as they resolve. This ticket does not write the full architecture doc.

## Answer

Hive terms stay. Nest and Mongo words die.

Replacements: crate, sqlx adapter, codec, composition root, import wall, schema, table, cell role.

One Postgres database. One schema per cell. One or more exclusive tables. One cell role per schema.

Died: Nest module, AppModule, mongoose, Zod, sheriff, dependency-cruiser, collection, promote, slice, neighbour, legacy bridge, hex platform kit.

Fatten stays.

Glossary: [CONTEXT.md](../../../CONTEXT.md).

## Comments

### Round 1

Arrows accepted. Postgres locked: one database, schema per cell, one or more tables per cell, restricted access per cell LOGIN.

- Nest module → crate
- mongoose adapter → sqlx adapter
- Zod codec → codec
- AppModule → composition root (`app` binary)
- sheriff / dependency-cruiser → import wall
- Mongo collection → table
- promote dies; fatten stays

Terms written to `CONTEXT.md`. Slice and the LOGIN word still open.

### Round 2

Arrows accepted. Slice dies. LOGIN word is cell role.

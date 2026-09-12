# What does a documentation factory look like for a multi-cell hive?

Type: research
Status: resolved
Label: wayfinder:research
Blocked by:

## Question

How do mature codebases assemble per-module docs, one maintainer entrypoint, and generated API specs without mixing audiences?

The factory is maintainer-only. OpenAPI is a separate consumer artifact. Cover generation versus handwritten prose, entrypoint shape, and how CI fits.

Primary sources: docs-as-code tool docs, and at least two large public codebases.

Recommend a factory shape. Do not pick a site generator as law yet.

Asset: `.scratch/hive-docs/research/02-documentation-factory.md`

## Answer

The factory is a book assembler with one maintainer TOC: handwritten cell pages, short kernel/`app` pages, a start page with a one-page graph, and links to law and glossaries. OpenAPI is a separate generated consumer artifact, off that TOC. rustdoc is not the handbook. CI builds the book; OpenAPI is a second job. Do not pin a site generator. Findings: [../research/02-documentation-factory.md](../research/02-documentation-factory.md).

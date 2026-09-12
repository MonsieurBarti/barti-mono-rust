# Where do handbook files live, and what is the maintainer entrypoint?

Type: grilling
Label: wayfinder:grilling
Blocked by: 02
Status: resolved

## Question

Where does a cell handbook page live, and what file is the maintainer entrypoint?

One entrypoint. It links cell pages, `CONTEXT-MAP.md`, kernel and `app` short pages, and existing law. It does not present OpenAPI as maintainer docs. It does not rewrite law.

## Answer

Handbook sources live under `docs/handbook/`. They do not sit next to crates.

```
docs/handbook/README.md
docs/handbook/cells/<cell>.md
docs/handbook/kernel.md
docs/handbook/app.md
```

The maintainer entrypoint is `docs/handbook/README.md`. It is the start page. It is not `app`. It is not the TOC.

The start page title is `Handbook`. The lead is untitled and freight-plain. Next is a handwritten mermaid graph, then links. Graph nodes are cells. First-cut nodes are `loads`, `shipments`, and `settlement`. Arrows are cell-to-cell hops at that grain. Kernel and `app` are links under the graph, not nodes. The page links cell pages, `CONTEXT-MAP.md`, `kernel.md`, `app.md`, [docs/architecture.md](../../../docs/architecture.md), and [docs/adr/](../../../docs/adr/). It does not present OpenAPI. It does not rewrite law. It does not restate [Communication](../../../docs/adr/0005-communication.md).

A domain folder is not a handbook path segment. There is no `docs/handbook/freight/`.

A cell page is `docs/handbook/cells/<cell>.md`. Its sections stay on [What is a cell handbook page?](06-cell-handbook-page.md).

The TOC filename waits on the site generator. Cell glossary files stay out of this tree.


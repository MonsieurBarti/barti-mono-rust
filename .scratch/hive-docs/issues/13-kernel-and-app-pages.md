# What are the kernel and app short pages?

Type: grilling
Status: resolved


Label: wayfinder:grilling
Blocked by: 06, 07

## Question

What does the kernel page say, and what does the `app` page say?

Kernel is not a cell. `app` is not a cell. Each page is short. They must not become a second [docs/architecture.md](../../../docs/architecture.md).

## Answer

Both pages share this skeleton. Title, untitled lead, How it works, See also. Not the cell skeleton. No Open Host. No Leaving SPIs. No Invariants. No schema sentence. No ticks, Work, HTTP, rustdoc, or file-tree headings.

The lead is hive-plain. No crate path. No file list. Lead ≤80 words. Page ≤200 words. A page over 400 words is a spec reject. Length is a writing contract. It is not a CI gate.

How it works names groups, one line each, then one why sentence that links law. Exclusive export list, forbidden list, layer-import matrix, import wall, and `app` file tree stay in law.

```markdown
# kernel

Kernel holds shared types and ports every cell may use. It is not a cell. It has no Open Host and no schema.

## How it works

- Money and Instant — shared value objects
- Clock — application reads now
- Logger and Metrics — bound on cell `new`
- Violation — payload on `VALIDATION_FAILED`

Cells import these. Kernel never imports a cell. See [Shared kernel](../architecture.md#15-shared-kernel).

## See also

- [Shared kernel](../architecture.md#15-shared-kernel)
- [Shared kernel ADR](../adr/0016-shared-kernel.md)
- [CONTEXT.md](../../CONTEXT.md)
```

```markdown
# app

`app` is the composition root that starts the hive. It is not a cell. It is not the gateway. It does not issue AuthN.

## How it works

- migrate and serve — the two process entries
- binds cells, InProc, named pools, Clock, Logger, and Metrics
- merges OpenAPI and does not serve it
- `GET /health` is served and off the spec

Cells never import `app`. Tests never boot `app`. See [Composition root](../architecture.md#3-composition-root).

## See also

- [Composition root](../architecture.md#3-composition-root)
- [Composition root ADR](../adr/0004-composition-root.md)
- [REST](../adr/0009-rest.md)
```

Relative links are from `docs/handbook/`. No `.scratch/` links. No context-map link.

This map does not write the files. [Write the cell-docs and OpenAPI spec](14-write-the-spec.md) copies these pages into the spec.


## Comments

### Round 1

Four arrows accepted.

Q1 A: shared skeleton. Title, untitled lead, How it works, See also. No Open Host, Leaving SPIs, Invariants, schema sentence, ticks, Work, HTTP, rustdoc, or file-tree headings.

Q2 A: lead ≤80 words. Page ≤200 words. Reject over 400. Writing contract, not a CI gate.

Q3 A: hive-plain lead. Kernel: shared types and ports, not a cell, no Open Host, no schema. `app`: composition root that starts the process, not a cell. No crate path. No file list.

Q4 A: How it works names groups, one line each, then links law. Kernel: Money and Instant, Clock, Logger and Metrics, Violation. `app`: migrate and serve; binds cells, InProc, pools, Clock, Logger, Metrics; merges OpenAPI and does not serve it; `GET /health` is served and off the spec. Exclusive export list, forbidden list, layer-import matrix, import wall, and `app` file tree stay in law.

### Round 2

Three arrows accepted.

Q5 A: kernel See also is Shared kernel, ADR 0016, root CONTEXT.md. `app` See also is Composition root, ADR 0004, REST ADR 0009. No `.scratch/` links. No context-map link.

Q6 A: one sentence after each list. Kernel: cells import these; kernel never imports a cell. `app`: cells never import `app`; tests never boot `app`. No second paragraph.

Q7 A: `app` lead says composition root, not a cell, not the gateway, does not issue AuthN. Kernel lead stays Round 1.

### Round 3

Q8 A accepted. Drafts recorded. Ticket closed.




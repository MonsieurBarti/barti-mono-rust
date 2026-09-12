# What is a cell handbook page?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 01, 05

## Question

What sections does a cell handbook page have, and how long is it?

The page must cover what the cell does in plain language, how it works, domain rules versus the cell glossary, Open Host, and leaving SPIs. It must not restate [docs/architecture.md](../../../docs/architecture.md) or the cell `CONTEXT.md`.

## Answer

A cell handbook page uses this skeleton.

```markdown
# <cell>

<untitled lead, ≤120 words: freight-plain owns / does not own. No crate paths, no ports.>

## How it works

### Open Host

- createLoad — post a Load

### Leaving SPIs

- None.

This cell owns the `<cell>` schema.

<why prose: why these edges, where a hop goes. Link law. Do not rewrite it.>

## Invariants

- A Load has two Stops on the first cut.
- Consignee is a name on the delivery stop, not a party.

## See also

- Cell glossary
- Context map
- ADRs this page actually cites
```

The title is the cell name. The lead is untitled and freight-plain. It states what the cell owns and does not own. It names no crate path and no port.

How it works lists edges, then why. An Open Host bullet is the port name plus one-line intent. A Leaving SPIs bullet is an other-cell consumer SPI, an `IntegrationEventSink`, or a vendor SPI. An empty leaving list is the line `None.` Persistence is one sentence: this cell owns the `<cell>` schema. Why prose links law. It does not rewrite law.

An invariant is one sentence. Terms link to the cell glossary. Identity and communication stay links to law.

See also links the cell glossary, the context map, and ADRs this page cites.

The lead is at most 120 words. The page is at most 500 words. A page over 800 words is a spec reject. Length is a writing contract. It is not a CI gate.

The page has no headings for ticks, Work, unpublished ports, HTTP, or rustdoc. A tick or Work drain is one sentence under How it works.

Open Host bullets omit PL structs, envelope variants, REST paths, and unpublished ports. Write SPIs and read SPIs do not appear.

File paths wait on [Where do handbook files live, and what is the maintainer entrypoint?](07-handbook-entrypoint.md). The filled `loads` page waits on [Write the cell-docs and OpenAPI spec](14-write-the-spec.md).

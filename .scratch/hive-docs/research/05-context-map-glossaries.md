# How DDD codebases keep a context map and per-context glossaries

Researched 2026-09-12.

## Verdict

Keep three files, none of them a handbook.

1. **Root `CONTEXT.md`**: hive process language and kernel types only. Not freight.
2. **`CONTEXT-MAP.md`**: named cells, path to each cell glossary, and contact points (pattern + integration-event or port name). Not how-it-works. Not Open Host catalogs.
3. **Cell `CONTEXT.md`**: that cell's ubiquitous language only. Terms, one- or two-sentence definitions, `_Avoid_`. Same format as today's root glossary.

Freight nouns move out of root when the first cell glossary lands. `Shipper` and `Carrier` are restated per cell, not a shared freight glossary. `kernel` and `app` are not contexts. Domain folder `freight` is not a context.

Do not write `CONTEXT-MAP.md` in the repo root on this map. Chapter 16 and [ADR 0017](../../../docs/adr/0017-freight.md) forbid it until cell crates exist. The spec proposes contents. Ticket 08 grills the wording.

## Compared

| Artifact | What it is | What it is not |
|---|---|---|
| **Ubiquitous language (Evans)** | One language inside one bounded context, used in speech, writing, diagrams, and code. A change in the language is a change to the model. | A company-wide dictionary. A term may mean something else in the next context. |
| **Bounded context (Evans)** | The boundary inside which one model applies. Set in team, code base, and schema. | A domain folder. A deployable. A second unit beside the hive cell. |
| **Context map (Evans)** | Named contexts plus points of contact: translation, sharing, isolation, influence. Map the terrain that exists. | A narrative of how a context works. A domain vision statement. A port catalog. |
| **Mapping patterns (Evans / DDD-Crew)** | Partnership, Shared Kernel, Customer/Supplier, Conformist, ACL, Open-host Service, Published Language, Separate Ways, Big Ball of Mud. | Team-topology diagrams unless that is the question the map answers. |
| **Bounded Context Canvas (DDD-Crew)** | Workshop sheet: name, purpose, collaborators, messages, ubiquitous language, decisions. | A file to check in as `CONTEXT.md`. Purpose, metrics, assumptions, and business decisions belong in the handbook or stay in the workshop. |
| **Domain Message Flow (DDD-Crew)** | Named commands, events, queries between contexts for one scenario. 5–9 messages. | A substitute for the map. A substitute for the glossary. |
| **Context Mapper CML** | Declarative `ContextMap { contains …; A [D,ACL]<-[U,OHS,PL] B }`. Lives in the repo. Not prose. | A hive dependency. Three cells do not earn a DSL. |
| **Lakeside Mutual / dddsample-core** | Product code. Language lives in types. dddsample-core is one context; its README is run instructions plus an entity diagram. | Per-context glossaries. A checked-in context map. |
| **kgrzybek/modular-monolith-with-ddd** | Full modular monolith. Catalog of DDD *pattern* terms. Strategic mapping is explicitly out of scope. | A model to copy. The README is a handbook. |

Evans (DDD Reference, 2015 extract of the 2004 book):

- Ubiquitous language is “structured around the domain model and used by all team members **within a bounded context**.”
- Bounded context: “Explicitly set boundaries in terms of team organization, usage within specific parts of the application, and physical manifestations such as code bases and database schemas.”
- Context map: “Name each bounded context, and make the names part of the ubiquitous language. Describe the points of contact between the models, outlining explicit translation for any communication, highlighting any sharing, isolation mechanisms, and levels of influence. Map the existing terrain. Take up transformations later.”
- Open-host Service + Published Language are the upstream pair. Anticorruption Layer is the downstream translator. Shared Kernel: “Keep this kernel small” and do not change it without the other team.
- Distillation extras stay out of the glossary: Domain Vision Statement is “about one page”; Highlighted Core is “three to seven sparse pages.” Those are handbook, not `CONTEXT.md`.

Vernon (*Implementing Domain-Driven Design*, 2013) puts strategy first: chapter 2 is Domains, Subdomains, and Bounded Contexts (including “Room for More than the Model” and “Aligning with Technical Components”); chapter 3 is Context Maps (“Why Context Maps Are So Essential”, “Drawing Context Maps”, “Projects and Organizational Relationships”). A bounded context is the linguistic boundary, not a class diagram. The map is the drawing of how those boundaries touch.

DDD-Crew splits the work the hive already split:

- **Connect** (context map + message flow): contacts between contexts.
- **Define** (Bounded Context Canvas): one context’s language and collaborators.
- **Organise**: team relationships. Hive is one team and one process, so skip team maps.
- Prefer small maps for one question. Document which patterns you use. Do not dump every pattern onto one sheet.

Brandolini (InfoQ, 2009): the map is a whiteboard of contours, not UML. Ambiguity of a term across contexts is the reason the map exists. Upstream/downstream is influence, not HTTP.

Public codebase that *keeps* a context map: [ContextMapper/context-mapper-examples](https://github.com/ContextMapper/context-mapper-examples). The DDD cargo sample is a `ContextMap` of three named contexts and two relationship lines (Shared Kernel; OHS+PL upstream of an implicit conformist/ACL downstream). No prose. The same repo reverse-engineers [Lakeside Mutual](https://github.com/Microservice-API-Patterns/LakesideMutual) into the same shape. Lakeside Mutual itself does not keep that map; it keeps per-service READMEs. [citerus/dddsample-core](https://github.com/citerus/dddsample-core) (Evans / Domain Language sample) keeps the language in Java types and `package.html`, not a glossary file.

## Fit to hive

Hive law already picked the mapping patterns. Do not reopen them.

| Evans / DDD-Crew | Hive |
|---|---|
| Bounded context | Cell. [ADR 0003](../../../docs/adr/0003-cell.md). Never a second unit. |
| Code base + schema as the boundary | One crate, exclusive tables in one schema. |
| Open-host Service | Open Host: exported API ports under `domain/api/`. Not HTTP. Not an integration event. |
| Published Language | JSON primitives at the cell edge. [ADR 0015](../../../docs/adr/0015-published-language-scalars.md). |
| Anticorruption Layer | Consumer SPI in consumer primitives + `app` InProc adapter. [ADR 0005](../../../docs/adr/0005-communication.md). |
| Shared Kernel | `crates/kernel`: Money, Instant, Clock, Logger, Metrics, Violation. No freight nouns. [ADR 0016](../../../docs/adr/0016-shared-kernel.md). |
| Integration event | Producer outbox PL, not Open Host. Consumer inbound API port. |
| Domain folder | Path segment. Not a context. Not on the map as a node. |
| Composition root | `app`. Not a context. |

Locked on this map: `CONTEXT.md` stays a glossary. Handbook is a separate narrative. The maintainer entrypoint links both. A cell handbook page names Open Host ports and leaving SPIs. The map does not.

`docs/agents/domain.md` and the domain-modeling skill `CONTEXT-FORMAT.md` already describe the file split. First cell language is the gate. That matches ADR 0017: “`CONTEXT-MAP.md` waits on cell crates so the glossary stays single-context until those languages exist.”

### What `CONTEXT-MAP.md` lists

Two headings. Nothing else.

```md
# Context Map

## Contexts

- [loads](./crates/cells/freight/loads/CONTEXT.md) — Shipper posted demand; owns Load, Quote, Stop
- [shipments](./crates/cells/freight/shipments/CONTEXT.md) — contracted move after a Load is booked
- [settlement](./crates/cells/freight/settlement/CONTEXT.md) — Invoice to the Shipper and Payable to the Carrier

## Relationships

- **loads → shipments**: loads emits `Book` (integration event, Published Language). shipments receives it on its own API port. Consumer SPI + InProc is the anticorruption layer. Upstream is Open Host + Published Language.
- **shipments → settlement**: CustomerRate and CarrierRate lock on the Shipment at book. settlement keeps a local read model of those rates.
- **kernel**: Shared Kernel of Money and Instant. Not a cell. Freight nouns do not live here.
```

Rules:

- Name the cell, one clause of purpose, link to its `CONTEXT.md`.
- Name the contact: integration event or the fact copied into a local read model. Name the Evans pattern in hive words (Open Host, Published Language, anticorruption layer).
- `shipments` and `settlement` may stay named-only until their glossaries exist (ticket 08). Still list them as contexts. Empty glossary files are worse than a named row.
- Omit: Open Host port lists, SPI lists, how Book is drained, REST paths, team topology, Wardley/core-domain charts, mermaid that restates [ADR 0005](../../../docs/adr/0005-communication.md).
- Omit Context Mapper. Three cells, two arrows, markdown.

### What a cell `CONTEXT.md` holds

Path: `crates/cells/freight/<cell>/CONTEXT.md`. Format is the domain-modeling skill `CONTEXT-FORMAT.md`: title, one or two sentences of what this context is, then `## Language` with `**Term**:` / definition / `_Avoid_`.

Only terms this cell owns or restates.

| Cell | Holds | Does not hold |
|---|---|---|
| `loads` | Load, Quote, Stop, Consignee, Shipper (posts), Carrier (quotes) | Shipment, Invoice, Payable, CustomerRate, CarrierRate |
| `shipments` | Shipment, CustomerRate, CarrierRate, Stop (local read model), Shipper, Carrier | Load as an aggregate, Quote as an aggregate, Invoice, Payable |
| `settlement` | Invoice, Payable, CustomerRate, CarrierRate (local read model), Shipper, Carrier | Load, Quote, Shipment as an aggregate |

`Shipper` and `Carrier` appear in more than one glossary on purpose. Evans: the language is per context. Settlement's Carrier is the party you pay. Loads' Carrier is the party that quotes. Do not merge them in root to save duplication.

Leave out: relationships (the map), Open Host ports (handbook), REST, invariants-as-essays, “how book works.” Business Decisions and Verification Metrics from the Bounded Context Canvas stay out.

### What stays in root `CONTEXT.md`

Hive process and kernel. Today's `## Language` through **Wide event**. Drop `### Freight` when the first cell `CONTEXT.md` exists.

Keep: Cell, Domain folder, Bounded context, Hexagon, Crate, Composition root, Open Host, API port, Command, Query, SPI, Published Language, Codec, adapters, Kernel, Money, Instant, Calendar date, Clock, Envelope, ActorId, Gateway, Domain event, Integration event, Tick, Work, Schema, Table, Fatten, Logger, Metrics, CorrelationId, Idempotency-Key, Wide event.

Do not keep: Load, Shipment, Quote, CustomerRate, CarrierRate, Invoice, Payable, Shipper, Carrier, Consignee, Stop.

Kernel types stay in root because they are the Shared Kernel, not a cell language. They are not freight.

Until cell crates exist, freight terms stay in root. That is current law (chapter 16, ADR 0017, domain.md). The spec describes the split. The implementation map writes the files.

### Law

Writing `CONTEXT-MAP.md` now would reopen chapter 16 and ADR 0017. This research does not reopen them. Ticket 08 decides the exact sentences once this split is accepted.

Do not put a domain-level `CONTEXT.md` under `crates/cells/freight/`. A domain folder is not a bounded context.

Do not list unpublished ports on the map. OpenAPI is public REST only (locked). The map is not OpenAPI.

## Sources

- Eric Evans, *Domain-Driven Design: Tackling Complexity in the Heart of Software*, Addison-Wesley, 2004. Pattern summaries: [DDD Reference (2015)](https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf), [landing page](https://www.domainlanguage.com/ddd/reference/). Ubiquitous Language; Bounded Context; Context Map; Partnership; Shared Kernel; Customer/Supplier; Conformist; Anticorruption Layer; Open-host Service; Published Language; Separate Ways; Domain Vision Statement; Highlighted Core.
- Vaughn Vernon, *Implementing Domain-Driven Design*, Addison-Wesley, 2013, ISBN 978-0-321-83457-7. [Sample pages / TOC](https://ptgmedia.pearsoncmg.com/images/9780321834577/samplepages/0321834577.pdf): ch. 2 Domains, Subdomains, and Bounded Contexts (pp. 43–84); ch. 3 Context Maps (pp. 87–111).
- [ddd-crew/context-mapping](https://github.com/ddd-crew/context-mapping): nine patterns, three team relationships, “prefer small context maps for explicit questions.”
- [ddd-crew/bounded-context-canvas](https://github.com/ddd-crew/bounded-context-canvas): Ubiquitous Language section vs Purpose / Strategic Classification / Business Decisions / Metrics.
- [ddd-crew/ddd-starter-modelling-process](https://github.com/ddd-crew/ddd-starter-modelling-process): Connect (map + message flow) then Define (canvas).
- [ddd-crew/domain-message-flow-modelling](https://github.com/ddd-crew/domain-message-flow-modelling): named messages between contexts; 5–9 per diagram.
- [ddd-crew/welcome-to-ddd](https://github.com/ddd-crew/welcome-to-ddd): points at the DDD Reference for pattern definitions.
- Alberto Brandolini, [Strategic Domain Driven Design with Context Mapping](https://www.infoq.com/articles/ddd-contextmapping/), InfoQ, 2009.
- [ContextMapper language: Context Map](https://github.com/ContextMapper/contextmapper.github.io/blob/master/_docs/language-reference/context-map.md) and [context-mapper-examples](https://github.com/ContextMapper/context-mapper-examples), especially [DDD-Sample-Stage-5.cml](https://github.com/ContextMapper/context-mapper-examples/blob/master/src/main/cml/ddd-sample/DDD-Sample-Stage-5.cml).
- [citerus/dddsample-core](https://github.com/citerus/dddsample-core) (Evans / Domain Language sample).
- [Microservice-API-Patterns/LakesideMutual](https://github.com/Microservice-API-Patterns/LakesideMutual).
- Hive: [CONTEXT.md](../../../CONTEXT.md), [docs/agents/domain.md](../../../docs/agents/domain.md), [docs/architecture.md](../../../docs/architecture.md) ch. 1, 2, 4, 15, 16, [ADR 0002](../../../docs/adr/0002-language.md), [ADR 0003](../../../docs/adr/0003-cell.md), [ADR 0005](../../../docs/adr/0005-communication.md), [ADR 0016](../../../docs/adr/0016-shared-kernel.md), [ADR 0017](../../../docs/adr/0017-freight.md), domain-modeling skill `CONTEXT-FORMAT.md`.

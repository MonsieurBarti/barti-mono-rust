# Good maintainer documentation

Researched 2026-09-12.

## Verdict

Adopt eight rules. Do not import a style guide.

1. **The handbook is explanation.** Diátaxis maps four needs. Tutorials and how-to guides inform action. Reference and explanation inform cognition. Explanation serves study: context, why, connections. Reference serves work: austere facts about the machinery. The maintainer handbook answers "tell me about this cell." It is not a lesson, not a procedure, not a map of the API. Crossing those boundaries is the usual failure. ([Diátaxis start here](https://diataxis.fr/start-here/), [compass](https://diataxis.fr/compass/), [explanation](https://diataxis.fr/explanation/), [reference](https://diataxis.fr/reference/))

2. **One job per source.** Write the Docs calls this Unique: overlapping sources rot because one copy dies. `CONTEXT.md` owns terms. `docs/architecture.md` and ADRs own law. OpenAPI owns public REST for API consumers. The handbook owns the narrative: what the cell does, why the boundary exists, how it talks to others. Link the other three. Do not copy them. Some restatement of purpose is required (ARID: Accept some Repetition In Documentation). Restating law is not. ([WTD principles: Unique, ARID, Current](https://www.writethedocs.org/guide/writing/docs-principles/))

3. **The lead is freight-plain.** Microsoft: put the takeaway first, on the first screen, in short sentences. Google: conversational, no jargon unless the reader needs the term. Diátaxis: talk *about* the subject. The lead states the freight problem this cell owns, in Shipper / Carrier / Load language a person who has not opened the crate can read. It names what the cell does not own. It does not name crate paths, SPI traits, Open Host ports, or layer folders. ([Microsoft top 10](https://learn.microsoft.com/en-us/style-guide/top-10-tips-style-voice), [scannable content](https://learn.microsoft.com/en-us/style-guide/scannable-content/), [Google tone](https://developers.google.com/style/tone), [jargon](https://developers.google.com/style/jargon), [Diátaxis explanation](https://diataxis.fr/explanation/))

4. **"How it works" unfolds the machinery.** Diátaxis explanation is allowed to circle: history, choices, why, analogies, alternatives. That is this section. Open Host ports and leaving SPIs belong here as named facts (map lock), written as short lists in reference language: describe, do not instruct. Prose after the list says why those edges exist and where a call goes. Link [Communication](../../docs/adr/0005-communication.md). Do not rewrite it. ([Diátaxis explanation](https://diataxis.fr/explanation/), [reference](https://diataxis.fr/reference/), [reference vs explanation](https://diataxis.fr/reference-explanation/))

5. **Name the actor. After the lead, use CONTEXT terms.** Google active voice: the grammatical subject performs the action. After the lead, hive words come from [`CONTEXT.md`](../../CONTEXT.md). Load is not Shipment. Cell is not crate. Open Host is not HTTP. Avoid lists in that glossary stay avoided. Precision is the named actor plus the glossary noun, not a longer sentence. ([Google active voice](https://developers.google.com/style/voice), [highlights](https://developers.google.com/style/highlights))

6. **Scan, then stop.** Microsoft: short headings, 3–7 line paragraphs, sentence-style capitalization, serial comma, important idea at the start of the heading and the paragraph. Write the Docs skimmable: headings describe the section, hyperlinks surround words that name the target, list items open on the concept. Google: if a sentence is an instruction, the condition comes first so the reader can skip it. ([Microsoft scannable content](https://learn.microsoft.com/en-us/style-guide/scannable-content/), [headings](https://learn.microsoft.com/en-us/style-guide/scannable-content/headings), [WTD skimmable](https://www.writethedocs.org/guide/writing/docs-principles/#skimmable), [Google sentence structure](https://developers.google.com/style/sentence-structure))

7. **Stale beats missing.** Write the Docs Current: incorrect documentation is worse than absent documentation. A handbook page that names a port the cell no longer exports is a bug. Prefer version-agnostic wording. ([WTD Current](https://www.writethedocs.org/guide/writing/docs-principles/#current))

8. **Keep modes apart inside the page.** Diátaxis: explanation that absorbs instruction or technical description ruins both. Port and SPI lists stay lists. Why they exist stays prose. FAQs are not a page shape. ([Diátaxis quality](https://diataxis.fr/quality/), [WTD on FAQs](https://www.writethedocs.org/guide/writing/beginners-guide-to-docs/))

## Lead versus how it works

| | Lead | How it works |
| --- | --- | --- |
| Reader need | Study: what is this, in the brokerage | Study: why the machinery is this shape |
| Language | Freight nouns. Plain English. No hive machinery names | CONTEXT terms. Named ports and SPIs. Links to law |
| Shape | A few short paragraphs. Takeaway first | Headings, then a short list of edges, then why |
| Test | A person who does not know the repo can say what the cell is for and what it is not for | A maintainer can name Open Host, leaving SPIs, and the other cell on each hop, without being taught HTTP or InProc from scratch |
| Out | Crate paths, `domain/spi/`, layer folders, REST paths, code samples as the story | Restating chapter 4, copying `CONTEXT.md` definitions, a tutorial, a how-to |

Lead, invented for `loads` as a shape check, not as the cell page:

> `loads` holds a Shipper's posted demand to move freight from origin to destination. A Carrier answers with a Quote. Booking is not this cell's job; that becomes a Shipment elsewhere.

How it works, same cell, still a shape check:

> Open Host: post a Load, list open Loads, accept a Quote. Leaving SPIs: none to other cells on the first cut; the sea-orm adapter owns the `loads` schema. A booked Load stops being this cell's write model. See [Communication](../../docs/adr/0005-communication.md) for how a later consumer would hear that.

The lead uses Load, Shipper, Carrier, Quote, Shipment and stops. The second block names ports and the schema, then points at law.

## Compared

| Source | Current canonical | Job for this spec | Leave on the shelf |
| --- | --- | --- | --- |
| Diátaxis | https://diataxis.fr/ (successor of the Divio documentation system) | Classify the handbook as explanation. Split lead (about the topic) from how-it-works (unfold the machinery). Keep OpenAPI, glossaries, and law in reference. Keep agent skills out. | A four-quadrant handbook site. Tutorials and how-to guides as cell-page sections. |
| Google developer documentation style guide | https://developers.google.com/style (highlights, tone, jargon, voice, person, sentence structure) | Active voice. Named actor. Write around hive jargon in the lead; define or link it later. Condition before instruction. Conversational, not cute. No *simply* / *easy*. | Second-person *you* as the default on an explanation page. Google itself uses third person for what the software does. Imperative *you* belongs in how-tos, which this product is not. |
| Microsoft Writing Style Guide | https://learn.microsoft.com/en-us/style-guide/welcome/ (replaces the Microsoft Manual of Style) | Get to the point. Scan first. Sentence-case headings. Serial comma. Short paragraphs. Front-load the keyword. | Consumer UI voice ("Ready to buy?"). Marketing contractions-as-personality. |
| Write the Docs | https://www.writethedocs.org/guide/ (principles, beginners guide, style-guide index) | Unique sources. ARID. Skimmable. Current. Nearby (docs live with the code, next map). No FAQ pages. | The OSS README template (install, license, contribute). That is a different publication. |

Federal Plain Language (https://www.plainlanguage.gov/guidelines/) sits on the Write the Docs style-guide index. It repeats "short, active, audience words first." It is not a fifth law.

## Fit to hive

Locked on this map: two products, two audiences; OpenAPI for API consumers; handbook for maintainers; they do not share an entrypoint; `CONTEXT.md` stays a glossary; non-technical is a writing constraint on the lead; law stays in `docs/architecture.md` and ADRs; a cell page names Open Host and leaving SPIs; neither the page nor the entrypoint restates Communication.

These rules fit that lock. They do not reopen chapters or ADRs. Pinning Google or Microsoft as house style is a spec choice, not an [ADR 0001](../../docs/adr/0001-stack-pins.md) crate pin.

Architecture chapters already use Context / Decision / Consequences. That shape is law. Cell handbook pages do not copy it.

Ticket 06 decides section list and length. This research only fixes the writing contract: lead is freight-plain explanation; how-it-works is explanation plus austere edge lists; glossary and law stay linked, not inlined.

## Sources

- https://diataxis.fr/
- https://diataxis.fr/start-here/
- https://diataxis.fr/explanation/
- https://diataxis.fr/reference/
- https://diataxis.fr/reference-explanation/
- https://diataxis.fr/compass/
- https://diataxis.fr/quality/
- https://diataxis.fr/application/
- https://developers.google.com/style
- https://developers.google.com/style/highlights
- https://developers.google.com/style/tone
- https://developers.google.com/style/jargon
- https://developers.google.com/style/voice
- https://developers.google.com/style/person
- https://developers.google.com/style/sentence-structure
- https://learn.microsoft.com/en-us/style-guide/welcome/
- https://learn.microsoft.com/en-us/style-guide/top-10-tips-style-voice
- https://learn.microsoft.com/en-us/style-guide/brand-voice-above-all-simple-human
- https://learn.microsoft.com/en-us/style-guide/scannable-content/
- https://learn.microsoft.com/en-us/style-guide/scannable-content/headings
- https://www.writethedocs.org/guide/
- https://www.writethedocs.org/guide/writing/docs-principles/
- https://www.writethedocs.org/guide/writing/beginners-guide-to-docs/
- https://www.writethedocs.org/guide/writing/style-guides/
- https://www.plainlanguage.gov/guidelines/ (adjacent; not adopted)

# How should we write good maintainer documentation?

Type: research
Status: resolved
Label: wayfinder:research
Blocked by:

## Question

How should maintainer docs be written so they stay precise, and so the lead section is plain language?

Audience is maintainers of this hive. The lead must still read to a person who does not know the codebase. Do not treat blog roundups as law.

Primary sources: Diátaxis, Google developer documentation style, Microsoft Writing Style Guide, and current Write the Docs material, or the documents that replaced them.

Recommend a small set of rules this spec can adopt. Not a style-guide dump.

Asset: `.scratch/hive-docs/research/01-good-maintainer-docs.md`

## Answer

The handbook is Diátaxis explanation, not a tutorial, how-to, or API map. The lead is freight-plain: what the cell owns and does not own, in Shipper/Carrier/Load language, no crate paths. How-it-works unfolds the machinery: Open Host and leaving SPIs as short lists, then why, with links to law and `CONTEXT.md` instead of copies. Eight rules in [../research/01-good-maintainer-docs.md](../research/01-good-maintainer-docs.md).

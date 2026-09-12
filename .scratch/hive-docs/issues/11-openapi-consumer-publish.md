# How do API consumers get the OpenAPI document?

Type: grilling
Label: wayfinder:grilling
Blocked by: 10
Status: resolved

## Question

Where does the consumer fetch the OpenAPI document?

Not Swagger UI in this process. Options: a committed artifact, JSON served from `app`, or a release asset. Pick one for the spec.

## Answer

Consumers fetch `docs/openapi/openapi.json` from git at the ref they integrate against.

The file is the one document `app` assembled. It is pretty JSON. A generator writes it. Nobody edits it by hand. Cell tags stay inside it. `app` does not serve it. GitHub Releases do not carry a copy. This process does not serve Swagger UI.

[How do CI gates enforce documentation without becoming theater?](04-ci-documentation-gates.md) regenerates this file for drift. [What does the CI gate require?](12-ci-gate-contract.md) rewrites coverage to: the merged spec contains tagged operations for every public router.

`info.title` and `info.version` wait on [Write the cell-docs and OpenAPI spec](14-write-the-spec.md).

## Comments

### Round 1

One arrow accepted.

Q1 A: consumers fetch the generated file from git at the ref they integrate against. `app` does not serve it. No GitHub Release copy.

### Round 2

Two arrows accepted.

Q2 A: one merged file. Consumers fetch the document `app` assembled. Cell tags stay inside it. Ticket 12 rewrites coverage to: the merged spec contains tagged operations for every public router.

Q3 A: pretty JSON. Native emit. No `yaml` feature. No `serde_norway` pin.

### Round 3

One arrow accepted.

Q4 A: consumers fetch `docs/openapi/openapi.json`.

### Round 4

Q5 A accepted. Draft recorded. Ticket closed.


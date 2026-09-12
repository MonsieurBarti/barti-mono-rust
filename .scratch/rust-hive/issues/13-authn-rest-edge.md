# How does AuthN/AuthZ work at the REST process edge?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 10

## Question

Port chapter 15 onto REST-only axum.

Locked: composition-root middleware inserts `ActorId`; handlers extract a string; the body omits it; cells never see the token; tick ports take no `actor_id`. Public error mapping is [What is the public driving-adapter surface?](10-public-driving-adapters.md).

Decide: identity mechanism, where the axum layer sits, 401 vs `UNAUTHENTICATED` envelope, whether AuthZ is process-edge or in-cell, and how unpublished webhook routes authenticate.

Do not pick an observability vendor here. Auth routes as a product (SSO pages) can stay out if the grill would explode.

## Answer

A gateway sits in front of this hive. It authenticates humans and allows routes. This process does not mint tokens, does not validate human Bearer JWTs, and does not serve SSO pages. Gateway product, IdP, and roles catalog are not this architecture.

`app` middleware copies gateway identity headers into request extensions. This bind is not public. Human REST requires `ActorId`. Health does not. Cell `router` has no AuthN layer. Cell e2e inserts `ActorId` via extension and never boots `app`.

Missing identity is HTTP 401, `application/problem+json`, `type` is `UNAUTHENTICATED`. It never enters a cell.

The API port takes `ActorId` as a string. Presentation extracts it. The body omits it. Roles and permissions do not enter the cell. Resource ownership lives in the use-case. `FORBIDDEN` comes from the use-case. No Policy SPI. No `organizationId` on the port.

Webhook routes are unpublished and sit off the human gateway JWT. The driving adapter verifies the vendor signature. Webhook ports take no `actor_id`. A bad or missing signature is 401 `UNAUTHENTICATED`. The cell never sees the secret.

Tick ports take no `actor_id`.

This ticket pins no JWT crate.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Gateway**, **ActorId**.

## Comments

### Round 1

A gateway sits in front of this API. That gateway handles AuthN, roles, and permissions. This process only reads the result. It does not mint tokens, does not validate human Bearer JWTs, and does not serve SSO pages.

Q1–Q4 of the first round are superseded by that placement. Layer, 401, port shape, and webhooks wait on the next round.

### Round 2

Five arrows accepted:

- Q6 A: gateway injects identity headers. `app` middleware copies them into extensions. This bind is not public. No human JWT validation in this process.
- Q7 A: gateway allows the route. The cell still enforces resource ownership. `FORBIDDEN` from the use-case. No Policy SPI.
- Q8 A: missing identity on this process is HTTP 401 problem+json, `type` is `UNAUTHENTICATED`, never enters a cell.
- Q9 A: webhooks are unpublished, off the human gateway JWT. Driving adapter verifies the vendor signature. No `actor_id`. Bad signature is 401 `UNAUTHENTICATED`.
- Q10 A: architecture names a gateway in front and this hive’s read contract. Gateway product, vendor, and roles catalog are not this map.

### Round 3

Two arrows accepted:

- Q11 A: API port takes `ActorId` as a string. Roles and permissions do not enter the cell.
- Q12 A: Health on `app` requires no identity headers.

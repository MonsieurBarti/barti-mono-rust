# Roles and permissions in hexagonal cells

Researched 2026-09-12. Primary sources only. Hive law is compared, not treated as industry proof. Vernon *Implementing Domain-Driven Design* has no reachable first-party page; Evans’s 2015 Domain Language reference is the DDD cite.

## Verdict

**Do not put IAM role or permission strings on a cell API port.**

On this boot map: the Gateway allows `POST /loads`. The API port takes `ActorId` as a string. The use-case records who posted the Load and later returns `FORBIDDEN` when that `ActorId` does not own the resource. No Policy SPI. No `organizationId`. No `admin`, `loads:write`, or realm-role list on the port.

Reopen on a later map if a roles catalog exists, more than one party type mutates the same Load, or a dedicated authorization service (Zanzibar-shaped) is in scope.

## Compared

Four questions. Do not collapse them.

| Split | Question | Who answers on this map | Industry backing |
| --- | --- | --- | --- |
| 1. Route allow | May this identity hit `POST /loads` at all? | Gateway. Function-level. | OWASP API5 (BFLA); RFC 6749 token + scope at the resource server; NIST RBAC as job-function transactions |
| 2. Resource ownership | May this `ActorId` mutate *this* Load? | Cell use-case. Object-level. `FORBIDDEN` from the use-case. | OWASP API1 (BOLA); OWASP Authorization Cheat Sheet object checks; NIST SP 800-162 object attributes |
| 3. IAM strings on the port | Do `admin`, `loads:write`, realm roles enter the cell? | No. | RFC 6749 scopes belong to the authorization server; Evans ubiquitous language; Cockburn inside/outside; Zanzibar as a *separate* service |
| 4. Freight capabilities | Do dispatcher / shipper-user nouns live in the cell? | Only if they are freight ubiquitous language. Today they are not user types. `Shipper` is the party that posts a Load, not an IAM role. `dispatcher` is absent from `CONTEXT.md`. | Evans ubiquitous language + generic subdomain |

### 1. Route allow

OWASP API5: function-level authorization is whether a caller may hit an endpoint they should not have as a regular user. “Frequently, such protection is provided by one or more components external to the application code.” Deny by default; explicit grants per function. That is Gateway work: can this identity call `POST /loads`.

NIST RBAC (Ferraiolo and Kuhn 1992; ANSI/INCITS 359) assigns transactions to organizational roles (doctor, teller). A role is a set of transactions a user may perform. That maps to route allow, not to “may this actor edit Load 17.”

RFC 6749: an access token is “a string representing an authorization issued to the client,” denoting “specific scopes and durations of access, granted by the resource owner, and enforced by the resource server and authorization server” (§1.4). Scope strings are defined by the authorization server (§3.3). The resource server “MUST validate the access token and ensure that it has not expired and that its scope covers the requested resource.” How the resource server talks to the authorization server is “beyond the scope of this specification” (§7). OAuth scopes are a Gateway/resource-server contract. They are not a freight model.

### 2. Resource ownership

OWASP API1 (BOLA): “Every API endpoint that receives an ID of an object, and performs any action on the object, should implement object-level authorization checks.” The checks “should validate that the logged-in user has permissions to perform the requested action on the requested object.” Being allowed onto the endpoint is not enough; that would be BFLA. Comparing session user id to an id parameter “isn't a sufficient solution” in general, but on this map the only object is a Load the same `ActorId` just posted, and Quote/Book are out of scope.

OWASP Authorization Cheat Sheet: “Perform access control checks on *every* request for the *specific* object or functionality being accessed. Just because a user has access to an object of a particular type does not mean they should have access to every object of that particular type.” Horizontal privilege elevation is the named failure. Checks “must be performed server-side, at the gateway, or using serverless function”; client-side checks do not count. Gateway route allow does not replace the object check.

NIST SP 800-162: ABAC grants or denies “based on assigned attributes of the subject, assigned attributes of the object, environment conditions, and a set of policies that are specified in terms of those attributes and conditions.” Object attributes are bound to the object. RBAC is the special case whose subject attribute is “role.” OWASP’s cheat sheet prefers ABAC/ReBAC over RBAC for object-level and horizontal decisions because RBAC “does a poor job of supporting object-level or horizontal access control.”

On this map the object attribute is “which `ActorId` posted this Load.” That lives in the use-case because the Load lives in `loads`. A Gateway cannot answer it without reading cell state.

### 3. IAM role/permission strings on the port

Putting `admin`, `loads:write`, or token realm roles on the API port imports an identity-product language into `loads`.

RFC 6749 does not define application authorization. Scope tokens are authorization-server strings. Token attributes and resource-server validation methods are “beyond the scope of this specification” (§1.4, §7). Passing those strings through the cell API port treats OAuth vocabulary as domain.

Cockburn’s hexagon: the application is “blissfully ignorant of the nature of the input device.” Adapters convert outside protocols into a procedure call. Use cases are written “at the application boundary (the inner hexagon) … regardless of external technology.” A Bearer token, a realm role, and a Keycloak group are outside-world protocol. The driving adapter’s job is to turn them into the call the application already understands: `ActorId`.

Evans (Domain Language reference, 2015): ubiquitous language is “structured around the domain model.” A change in the language is a change to the model. IAM strings are not freight. Generic subdomain: “Identify cohesive subdomains that are not the motivation for your project. Factor out generic models of these subdomains and place them in separate modules. Leave no trace of your specialties in them.” Identity and access is that generic subdomain. Mixing it into `loads` clogs the core. Anticorruption layer: the downstream client (this hive) talks to the upstream (Gateway / IdP) through an isolating translation; internally it uses its own model (`ActorId`), not the upstream’s.

Zanzibar (Pang et al., USENIX ATC 2019; Google Research PDF): Google did not put ACL evaluation inside Calendar, Drive, or YouTube. It built a *separate* authorization system those services query: “does user U have relation R to object O.” Relations (`owner`, `editor`, `viewer`) live in Zanzibar tuples, not as IAM role strings inside each product. A unified authorization service is a later map, not a reason to thread realm roles through `loads` today.

### 4. Domain capabilities (freight nouns)

`Shipper` is “the party that posts a Load,” not a token role. `Carrier` hauls a Shipment. The broker is the system, not a party. `dispatcher` is not in `CONTEXT.md`. Do not invent IAM-shaped user types and call them ubiquitous language.

If a later map makes “broker employee vs Shipper user vs Carrier user” a freight fact about who may post or book, that fact belongs in the cell as a domain concept with its own name. It still does not justify copying `realm_access.roles` onto the API port.

## What breaks if role strings enter `loads`

This boot map is one human `POST /loads`. Quote HTTP, Book, `shipments`, and `settlement` are out of scope.

- **InProc has no caller identity.** Chapter 3: inter-cell permission is which Open Host tokens `app` binds; “No Policy SPI between cells. No caller identity on the port.” A `loads` port that requires role strings cannot be called from another cell, from a test that constructs the cell with fakes, or from any InProc adapter. Those callers have no roles to pass.
- **Ticks have no actor.** Tick ports take no `actor_id`. This cut has no ticks, but a port shape that demands roles cannot grow a tick later without lying or minting a fake actor.
- **Roles catalog is out of scope.** Architecture chapter 12 and this map: Gateway product, IdP, JWT crate, SSO pages, and a roles catalog are not this work. There is no store of `admin` / `loads:write` for the cell to read. A Policy SPI would be an empty port.
- **Quote and Book are not this POST.** Object-level rules for quoting a Load or booking a Shipment are not specified here. Threading roles “for later” is a catalog the map refused.

The failure mode is BOLA with extra steps: Gateway already allowed the route; the cell then trusts a role string it cannot verify, and skips the ownership check it can verify.

## Fit to hive

Hive law already matches the verdict. That is a comparison, not a reason.

| Hive law (ch. 12, issue 13 Q7/Q11, `CONTEXT.md`) | Industry sources | Match? |
| --- | --- | --- |
| Gateway authenticates and allows routes | RFC 6749 resource server + scope; OWASP API5; OWASP “external to the application code” | Yes. Function-level at the edge. |
| API port takes `ActorId` as a string; body omits it; cell never sees the token | Cockburn adapter converts outside protocol; RFC 6749 token stays at the resource server | Yes. |
| Roles and permissions do not enter the cell | Evans UL / generic subdomain; RFC 6749 scopes are AS strings; Zanzibar is a separate system | Yes. Hive is stricter than “ABAC in every service,” and that strictness fits a map with no roles catalog. |
| Resource ownership lives in the use-case; `FORBIDDEN` from the use-case | OWASP API1; OWASP object-level checks on every request; NIST object attributes | Yes. This is the check Gateway cannot do. |
| No Policy SPI; no `organizationId` on the port | A Policy SPI with no catalog is an empty port. NIST ABAC enterprise needs attribute infrastructure this map does not build. | Yes for this map. Reopen when a catalog or multi-tenant org exists. |
| Ticks and webhooks take no `actor_id`; InProc takes no caller identity | Cockburn: batch and app-to-app drive the same inner API without a human identity | Yes. Role-shaped ports would break those drivers. |

OWASP would not accept Gateway-only authorization: BOLA still has to run in the function that loads the record. Hive already puts that in the use-case. NIST ABAC would eventually want richer subject and object attributes than `ActorId` plus “posted by.” That is the reopen condition, not a reason to pass `loads:write` through the port now.

Boot-map standing: `X-Actor-Id` is the identity header; create path is `POST /loads`. The Gateway (or a local stand-in) sets that header. `loads` stores the `ActorId` on the Load and uses it for ownership. Nothing else.

## Sources

- https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html
- https://owasp.org/www-project-api-security/
- https://api-security.owasp.org/editions/2019/en/0xa1-broken-object-level-authorization/
- https://nvlpubs.nist.gov/nistpubs/specialpublications/NIST.SP.800-162.pdf
- https://doi.org/10.6028/NIST.SP.800-162
- https://csrc.nist.gov/pubs/sp/800/162/upd2/final
- https://csrc.nist.gov/projects/role-based-access-control
- https://csrc.nist.gov/CSRC/media/Publications/conference-paper/1992/10/13/role-based-access-controls/documents/ferraiolo-kuhn-92.pdf
- https://www.rfc-editor.org/rfc/rfc6749
- https://www.rfc-editor.org/rfc/rfc6749#section-1.4
- https://www.rfc-editor.org/rfc/rfc6749#section-3.3
- https://www.rfc-editor.org/rfc/rfc6749#section-7
- https://www.usenix.org/system/files/atc19-pang.pdf
- https://www.usenix.org/conference/atc19/presentation/pang
- https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/
- https://alistair.cockburn.us/hexagonal-architecture/
- https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf
- https://www.domainlanguage.com/ddd/

Local law compared, not cited as industry authority: `docs/architecture.md` chapters 3 and 12; `.scratch/rust-hive/issues/13-authn-rest-edge.md` (Q7, Q11); `CONTEXT.md` (`ActorId`, `Gateway`, `Load`, `Shipper`); `.scratch/rust-hive-boot/map.md` out of scope.

Unreachable this pass: Vaughn Vernon first-party IDDD pages (`https://kalele.io/essays/` returned 404). No Medium recap substituted.

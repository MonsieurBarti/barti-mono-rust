# Which Load fields does the first POST require?

Type: grilling
Status: resolved

Label: wayfinder:grilling
Blocked by:

## Question

Pin the Published Language for the first human POST that creates a Load.

Hive law: a Load is a Shipper's posted demand to move freight from origin to destination. A Load has a list of Stops. First cut is two. Consignee is a name and address on the delivery stop. No asking amount on a Load. First language is FTL road. Quote lives in `loads` and is not this POST. Application mints UUIDv7 ids. Calendar date is `YYYY-MM-DD`. Instant encodes UTC `Z` with milliseconds.

Decide required body fields, optional fields, server-minted fields, and the 201 JSON. Stops must match the glossary. Shipper identity must be a field or rejected.

Do not pick DSN env names. Do not pick compose.

## Answer

Create-Load input PL is camelCase JSON. Required: `shipperId` (opaque non-empty string, no lookup) and `stops` (exactly two). Each stop has `kind` (`pickup` or `delivery`), `date` (`YYYY-MM-DD`), and `address` `{ line1, city, region, postalCode, country }`. `country` is ISO 3166-1 alpha-2. Delivery requires `name` (the Consignee). Optional: pickup `name`, `address.line2`.

Codec `deny_unknown_fields`. Reject wrong count, duplicate kinds, delivery before pickup, empty `shipperId`, and missing delivery `name`. No equipment, weight, commodity, asking amount, `actorId`, client `id`s, or `createdAt`.

`ActorId` stays on the port from `X-Actor-Id`. `Idempotency-Key` stays a header. Neither is JSON.

Application mints Load `id` and each Stop `id` (UUIDv7, lowercase, hyphenated). Application mints `createdAt` from Clock (UTC `Z` milliseconds). Store `ActorId` on the row. No `status`.

201 `application/json` is the Load: `id`, `shipperId`, `stops` (each with `id`, `kind`, `date`, `address`, and `name` when present), `createdAt`. Omit empty optionals. Do not echo `actorId` or `Idempotency-Key`.

Chapter 12 stays. No IAM role or permission strings on the port. Resource ownership lives in the use-case.


## Comments

### Round 1

Q0 A accepted (user: lgtm). Chapter 12 stays. No roles catalog. No permissions in `loads`. Resource ownership stays in the use-case. Gateway product stays out of this map.


### Round 2

Q0 A accepted after research (user: lgtm). Gateway allows the route. Port takes `ActorId`. Use-case owns resource checks and `FORBIDDEN`. No IAM role or permission strings on the port.


### Round 3

Four arrows accepted (user: lgtm):

- Q1 A: required `shipperId`, opaque non-empty string, no lookup. Actor and Shipper may differ.
- Q2 A: exactly two stops. `kind` is `pickup` or `delivery`. Codec rejects the wrong count, duplicate kinds, or delivery before pickup.
- Q3 A: every stop has `kind`, `date`, `address` `{ line1, city, region, postalCode, country }`. `country` is ISO 3166-1 alpha-2. `line2` optional. Delivery requires `name` (Consignee). Pickup `name` optional. No lat/lng, window, or phone.
- Q4 A: body is `shipperId` plus `stops`. No equipment, weight, commodity, or references.




### Round 4

Two arrows accepted (user: lgtm):

- Q5 A: mint Load `id` and each Stop `id` (UUIDv7). Mint `createdAt`. Store `ActorId` on the row. No `status`. Client must not send minted fields.
- Q6 A: 201 body is the Load: `id`, `shipperId`, `stops` (with `id`), `createdAt`. Omit empty optionals. Do not echo `actorId` or `Idempotency-Key`.


### Round 5

Q7 A accepted (user: lgtm). Draft recorded. Ticket closed.



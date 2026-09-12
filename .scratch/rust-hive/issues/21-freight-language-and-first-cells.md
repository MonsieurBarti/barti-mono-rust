# What freight ubiquitous language and first cells?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 06, 07

## Question

Pin the freight ubiquitous language and the named first cells for this hive.

The host product is a B2B freight brokerage. The sketch is post a load → quote → book → settle. Hive terms stay. Cell identity tests stay: too-small, too-big, chatty, then fatten, local read model, merge. Glossary: [What ubiquitous language does the Rust hive keep from naboo, and what Nest/Mongo terms die?](06-rust-hive-glossary.md). Packaging: [How is a cell packaged, and what is the composition root without Nest?](07-cell-packaging-and-composition-root.md).

Decide: freight glossary terms; domain folder names; which first cells exist.

Write `CONTEXT.md` freight terms as they resolve. Do not implement product code. Do not write architecture chapters here.

## Answer

Domain folder `freight`. First cells: `loads`, `shipments`, `settlement`. Paths: `crates/cells/freight/loads`, `crates/cells/freight/shipments`, `crates/cells/freight/settlement`.

`loads` owns Load, Quote, and Stop. Quote is an aggregate, not a cell. A Load has a list of Stops. First cut is two. Consignee is a name and address on the delivery stop.

`shipments` owns Shipment. Book is an integration event from `loads`. Shipments copies Stops into a local read model. CustomerRate and CarrierRate lock on the Shipment at book. CarrierRate comes from the winning Quote. No asking amount on a Load.

`settlement` owns Invoice to the Shipper at CustomerRate and Payable to the Carrier at CarrierRate. It keeps a local read model of those rates.

First invariants are FTL road. Other modes fatten these cells. A new cell only when identity tests fail. No mode in a cell name. No reserved empty cells. No Booking aggregate. The broker is the system, not a party.

Glossary: [CONTEXT.md](../../../CONTEXT.md) Freight. No `CONTEXT-MAP.md` until cell crates exist.

## Comments

### Round 1

Four arrows accepted (user: lgtm, with easy expansion).

- Q1 A: First language is FTL road. LTL, ocean, air, and rail stay out of first glossary.
- Q2 A: A Quote is a carrier offer on a Load. CustomerRate is Money, not an aggregate.
- Q3 A: Load is posted demand. Shipment is the booked move. Book is the transition.
- Q4 A: Shipper posts. Carrier hauls. Consignee is a name and address. The broker is the system.

Expansion is a later-round doctrine, not extra modes in first language.

Terms written to [CONTEXT.md](../../../CONTEXT.md).

### Round 2

Four arrows accepted (user: lgtm).

- Q5 A: Fatten. Same cells and nouns. New cell only when identity tests fail. No reserved empty cells. No mode in a cell name.
- Q6 A: CustomerRate locks onto the Shipment at book. An asking amount on a Load is not CustomerRate.
- Q7 A: A Load has a list of Stops. First cut is two. Consignee sits on the delivery stop.
- Q8 A: Quote is an aggregate in the loads cell. Not a cell.

Terms written to [CONTEXT.md](../../../CONTEXT.md).

### Round 3

Three arrows accepted (user: lgtm).

- Q9 A: `loads` owns Load, Quote, Stop. `shipments` owns Shipment. Book is an integration event. Shipments copies Stops into a local read model.
- Q10 A: Invoice (Shipper) and Payable (Carrier) are first language.
- Q11 B: No asking amount on a Load. No shipper-facing Money until book.

Terms written to [CONTEXT.md](../../../CONTEXT.md).

### Round 4

Three arrows accepted (user: lgtm).

- Q12 A: `settlement` owns Invoice and Payable. Local read model of CustomerRate and CarrierRate.
- Q13 A: Domain folder `freight` holds `loads`, `shipments`, `settlement`.
- Q14 A: CarrierRate locks onto the Shipment at book, from the winning Quote.

Terms written to [CONTEXT.md](../../../CONTEXT.md).

### Round 5

Q15 A accepted. Draft recorded. Glossary written. Ticket closed.

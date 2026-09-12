# loads

This context is a Shipper's posted demand to move freight from origin to destination. A Carrier answers with a Quote.

## Language

**Load**:
A Shipper's posted demand to move freight from origin to destination.
_Avoid_: Order, job, tender as this noun, Shipment

**Quote**:
A Carrier's priced offer on a Load.
_Avoid_: Bid, Rate as this noun, a shipper-facing quote aggregate

**Stop**:
A pickup or delivery location on a Load. First cut is two.
_Avoid_: Leg, unstructured origin and destination fields

**Consignee**:
The receiving name and address on a destination stop. Not a party with identity.
_Avoid_: Consignee aggregate, treating Consignee as a Shipper

**Shipper**:
The party that posts a Load.
_Avoid_: Customer, client, account

**Carrier**:
The party that hauls a Shipment.
_Avoid_: Trucker, vendor, supplier

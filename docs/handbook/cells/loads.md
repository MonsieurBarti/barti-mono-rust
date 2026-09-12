# loads

`loads` holds a Shipper's posted demand to move freight from origin to destination. A Carrier answers with a Quote. This cell owns that demand, the Quote, and the Stops. Booking is not this cell's job. That move becomes a Shipment elsewhere.

## How it works

### Open Host

- createLoad — post a Load

### Leaving SPIs

- None.

This cell owns the `loads` schema.

Posting demand is this cell's write. Quote is in the language and not yet an Open Host port. Book is not on this cut, so no leaving SPI points at shipments. See [Communication](../../adr/0005-communication.md). See [Freight](../../adr/0017-freight.md).

## Invariants

- A [Load](../../../crates/cells/freight/loads/CONTEXT.md) has two [Stops](../../../crates/cells/freight/loads/CONTEXT.md) on the first cut: one pickup and one delivery.
- [Consignee](../../../crates/cells/freight/loads/CONTEXT.md) is a name on the delivery stop, not a party.
- Delivery date is not before pickup date.

## See also

- [loads glossary](../../../crates/cells/freight/loads/CONTEXT.md)
- [Context map](../../../CONTEXT-MAP.md)
- [Communication](../../adr/0005-communication.md)
- [Freight](../../adr/0017-freight.md)

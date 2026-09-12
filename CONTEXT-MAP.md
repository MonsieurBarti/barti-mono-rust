# Context Map

## Contexts

- [loads](./crates/cells/freight/loads/CONTEXT.md) — Shipper posted demand to move freight from origin to destination
- shipments — contracted move after a Load is booked with a Carrier
- settlement — Invoice to the Shipper and Payable to the Carrier

## Relationships

- **loads → shipments**: loads emits `Book` (integration event, Published Language). shipments receives it on its own API port. Consumer SPI plus InProc is the anticorruption layer.
- **shipments → settlement**: settlement keeps a local read model of CustomerRate and CarrierRate locked on the Shipment at book.

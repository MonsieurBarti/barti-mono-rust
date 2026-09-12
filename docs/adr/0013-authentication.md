# Authentication

The gateway authenticates humans. This process copies identity headers so it does not become an IdP. Cells receive `ActorId` as a string and own resource `FORBIDDEN`, so roles never enter the hexagon.

Chapter: [12. Authentication](../architecture.md#12-authentication)

Grill: [How does AuthN/AuthZ work at the REST process edge?](../../.scratch/rust-hive/issues/13-authn-rest-edge.md)

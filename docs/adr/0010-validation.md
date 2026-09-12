# Validation

`domain/api` owns decode so inner domain and application stay serde-free and garde-free. `VALIDATION_FAILED` names shape and field constraints on Published Language primitives, not entity invariants or VO `TryFrom`. The sea-orm adapter does not re-validate this cell's own rows: a corrupt own row is a server bug.

Chapter: [9. Validation](../architecture.md#9-validation)

Grill: [Where does the codec validate versus domain invariants?](../../.scratch/rust-hive/issues/16-codec-versus-invariants.md)

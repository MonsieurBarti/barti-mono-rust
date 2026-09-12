# Testing

Three lanes per cell take their names from the layer, so Cargo `tests/` is e2e, not application. Integration and e2e run this cell's real sea-orm adapter plus handwritten fakes, because mockall is not law. Cell e2e never boots `app`, so the 80% line floor follows the cell.

Chapter: [10. Testing](../architecture.md#10-testing)

Grill: [What are the test lanes, and what does each one boot?](../../.scratch/rust-hive/issues/12-test-lanes.md)

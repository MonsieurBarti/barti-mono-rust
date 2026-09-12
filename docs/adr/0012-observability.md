# Observability

`app` owns OpenTelemetry, and kernel `Logger` and `Metrics` bind as generics on `new`, so cells stay tracing-free. `CorrelationId` is a hive UUID distinct from the OpenTelemetry trace id. One wide event covers each REST request that hits a cell handler, not each InProc hop.

Chapter: [11. Observability](../architecture.md#11-observability)

Grill: [How do we observe a REST hive process?](../../.scratch/rust-hive/issues/17-observability.md)

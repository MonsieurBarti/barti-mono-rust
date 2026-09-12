# app

`app` is the composition root that starts the hive. It is not a cell. It is not the gateway. It does not issue AuthN.

## How it works

- migrate and serve — the two process entries
- binds cells, InProc, named pools, Clock, Logger, and Metrics
- merges OpenAPI and does not serve it
- `GET /health` is served and off the spec

Cells never import `app`. Tests never boot `app`. See [Composition root](../architecture.md#3-composition-root).

## See also

- [Composition root](../architecture.md#3-composition-root)
- [Composition root ADR](../adr/0004-composition-root.md)
- [REST](../adr/0009-rest.md)

# Prove OpenAPI coverage and Problem identity

Type: task
Label: wayfinder:task
Blocked by: 07, 08
Status: resolved



## Question

Add the two cargo tests from [the locked spec](../../hive-docs/spec.md). They ride the existing nextest step.

Coverage instantiates each cell router `app` merges. Every operation in that cell OpenApi is tagged with the cell name. The merged document contains those operations. A cell with no public routes contributes no tag and passes. `GET /health` is absent from the spec.

Problem identity: before merge, among cell OpenApis that emit `components.schemas.Problem`, that schema JSON is identical.

## Answer

Two `app` unit tests ride nextest. Coverage walks each cell OpenApi `app` merges, checks the cell tag, checks the merged paths, proves an empty router adds no tag, and asserts `GET /health` is absent. Problem identity compares `components.schemas.Problem` JSON across cell OpenApis before merge.


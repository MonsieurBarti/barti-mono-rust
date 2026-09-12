# How does presentation emit OpenAPI, and which crate do we pin?

Type: grilling
Label: wayfinder:grilling
Blocked by: 03

## Question

Which OpenAPI crate does chapter 8 pin, and where do the annotations live?

Presentation maps verb plus path onto a use-case API port. Published Language structs already live in `domain/api`. Handlers live in `presentation/http`. Choose handler annotations, PL derives, or both. Name the [Stack pins](../../../docs/adr/0001-stack-pins.md) change.

# Apply the OpenAPI law change

Type: task
Label: wayfinder:task
Blocked by: 01
Status: resolved


## Question

Apply the OpenAPI law change from [the locked spec](../../hive-docs/spec.md).

Reopen chapter 8 and [Stack pins](../../../docs/adr/0001-stack-pins.md) only. Pin `utoipa` 5.5.0 and `utoipa-axum` 0.2.0. Add oasdiff, oasdiff-action, lychee, and lychee-action at the versions [What versions do we pin for oasdiff and lychee?](01-oasdiff-lychee-pins.md) records. Drop "No OpenAPI crate" from Not pinned.

Do not add crates to presentation. Do not add CI steps.

## Answer

Applied the spec's OpenAPI law. Chapter 8 takes the `utoipa` 5 and `utoipa-axum` 0.2 pins, the presentation duty, the `app` merge rules, the tag and uniqueness rules, the consumer fetch path, and the `RUSTSEC-2024-0436` ignore. Architecture stack pins take both majors. ADR 0001 takes `utoipa` 5.5.0, `utoipa-axum` 0.2.0, oasdiff 1.31.0, oasdiff-action v0.1.15, lychee 0.24.2, and lychee-action v2.9.0. Both not-pinned lists dropped the OpenAPI crate. No crate and no CI step moved.


# What is the current SOTA Rust HTTP stack for a hive composition root?

Type: research
Status: claimed
Label: wayfinder:research
Blocked by:

## Question

Which HTTP crate (and supporting stack: router, extractor, middleware, graceful shutdown) is current state of the art for a greenfield 2026 Rust service whose composition root binds many cells?

Constraints:

- Latest stable versions only.
- Must support a process-edge AuthN layer that injects `actor_id` into cell API ports, never tokens.
- Must not force a Nest-like module system.
- Compare at least axum, actix-web, and poem (or the then-current successors) against primary sources: crate docs, tokio/hyper, and recent release notes.
- Recommend one stack and pin the major versions.

Asset (research subagent writes here): `.scratch/rust-hive/research/01-sota-http-stack.md`

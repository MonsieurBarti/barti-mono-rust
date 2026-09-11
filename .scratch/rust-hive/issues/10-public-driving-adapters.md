# What is the public driving-adapter surface?

Type: grilling
Status: open
Label: wayfinder:grilling
Blocked by: 01, 06

## Question

Port naboo chapters 8–10 for a greenfield Rust hive.

Decide: REST, GraphQL, MCP, or a subset. Default recommendation: HTTP REST via the chosen stack; MCP out of v1 unless a cell needs agent tools; GraphQL out unless a client exists.

This is the public edge only. InProc is not public. AuthN mapping to `actor_id` can stay fog if this grill would otherwise explode.

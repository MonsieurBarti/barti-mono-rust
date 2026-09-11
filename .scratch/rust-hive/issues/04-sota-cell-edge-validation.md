# What is the current SOTA Rust approach to Published Language validation at a cell edge?

Type: research
Status: claimed
Label: wayfinder:research
Blocked by:

## Question

How should a Rust cell validate Published Language at the API-port edge (primitives in, primitives out, `{ type, context }` errors), equivalent to naboo's Zod codecs beside each port?

Constraints:

- Latest stable versions only.
- Compare serde, validator, garde, typestate, JSON Schema crates, and any 2026 successor against official docs.
- Both sides of InProc must encode/decode. Value objects stay inside the hexagon.
- Recommend one approach. Pin majors.

Asset: `.scratch/rust-hive/research/04-sota-cell-edge-validation.md`

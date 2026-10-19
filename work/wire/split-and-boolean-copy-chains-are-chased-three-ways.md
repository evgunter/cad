---
id: split-and-boolean-copy-chains-are-chased-three-ways
kind: issue
title: Three hand-written chain chases (pair_original, chase_split_lineage, Fused::origin) handle a cycle three different ways
status: closed
opened: 2026-10-18
priority: P3
cost: M
pr: 5405
branch: wire/one-chain-chase
closed: 2026-10-19
---


## Finding

From the review of PR 5275 (2026-10-18). Three loops in
`crates/editor-core/src/names/emit_topo.rs` follow a chain of
copy-or-lineage rows to its root. Each handles a cycle differently:

- `pair_original` follows a split's null-pair rows `(copy, original)`.
  It refuses with `NamingError::Emission("a split's null-pair rows cycle")`.
- `chase_split_lineage` follows `Provenance::SplitEdge` and
  `joined_lineage`. It refuses with `topo::SplitLineageCycle`.
- `Fused::origin` follows the boolean's null copies through
  `cones.root`. It is total by construction and refuses nothing.

Sweep `names/` for other hand-written chases of this shape. Then put one
helper behind them that takes the step function and a bound, with one
refusal shape.

Signed (wire-keeps lane).

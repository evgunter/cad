---
id: b-arena-edges-skip-the-split-lineage-chase
kind: issue
title: In a result that is B's clone, the boolean edge pass does not chase a split sub-edge's lineage; the A lane in the mirror layout does
status: open
opened: 2026-09-23
---


## What

`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`, the
operand-descended root read:

- `(Direct, Absent)` (the result is A's clone): `chase_edge_to_table(body, a.table, e)`
  walks the edge's `SplitEdge` provenance in the result body until A's
  table names a key.
- `(Absent, Direct)` (the result is B's clone): `chase_b(e)`, whose walk
  steps through `fwd_edges` — the graft rows, which are EMPTY in this
  layout. A sub-edge the reduction split off a B edge is therefore
  returned unchased, reads as `resolves == false`, and is handed to
  `chord_kind`, which re-derives the root as the one rim its two faces
  share (`rim_between`) and refuses `NamingError::SharedRim` when they
  share other than one.

In this layout the result body IS B's clone, so its provenance speaks
B keys exactly as it speaks A keys in the mirror layout, and the direct
chase the A lane uses is available. The two layouts reach the same root
by different rules.

## Measured

Found by the sweep of `the-b-side-contact-record-rescue-arm-never-fires`.
The ell-and-tip union in
`crates/editor-core/tests/emit_boolean_vertex_keys.rs`
(`a_b_edge_split_by_an_a_vertex_is_named_by_its_a_partner`, tip first:
`OperandB`) splits the ell's reflex edge; both halves come out
`FromB(LateralEdge v3)` with fragment ordinals, the minted half through
the `chord_kind` rescue. So the rescue gives the right answer there.

**Unmeasured**: a B-clone result where the split edge's two faces share
more than one rim (so the rescue refuses `SharedRim` while the mirror
order names the edge). That is the row to build first; if no legal
document reaches it, the finding is a second rule for one fact, not a
wrong answer.

## Fix shape

Give `(Absent, Direct)` the A lane's chase (`chase_edge_to_table(body,
b.table, e)`), keeping `chase_b` for the grafted layout, whose verbatim
keys it exists for (`work/origin/graft-copies-provenance-keys-verbatim.md`).

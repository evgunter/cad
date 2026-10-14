---
id: the-joined-edge-reading-has-two-implementations
kind: issue
title: The joined-edge cover reading has two implementations, the pair emitter's and the union's
status: closed
opened: 2026-10-06
closed: 2026-10-14
priority: P3
cost: E
branch: emit/union-readings
---



## What

PR 4161 (FUSE step 4) reads "which edges does this joined edge lie
along" in two places:
- the pair boolean: `joined_cover` and `set_holding` in
  `crates/editor-core/src/names/emit_topo.rs`;
- the union's end pass: `Flush::edge_cover` and `Flush::lines_at` in
  `crates/editor-core/src/names/emit_union.rs`.

Both call `Segment::cover` and `Segment::place` through
`ON_MEMBER_EDGE`, so the numeric reading is shared. What is duplicated
is the candidate gathering (edges between faces the edge's two faces
descend from) and the vertex reading (which listed edges hold a vertex).
They differ in their inputs: operand sides and keys for the pair,
member names for the union.

## What a fix looks like

One candidate builder, generic over how a face's descents and an
edge's name are read, called from both sites; and one "edges holding
this point" helper. Small, local to `names/`. The witnesses are
`emit_shared_rim_several`'s order-freedom and lies-on rows, which
exercise both sites.

## Closed (branch `emit/union-readings`, 2026-10-14)

It fell out of the two rows before it. `names/emit_topo.rs` now holds
the one reading, generic over the source (a pair's operand, a union's
member by index):

- `joined_candidates`: the edges between two faces of one source that
  a joined edge's two faces descend from, straight or curved as the
  edge is, each with its `CarrierArc`;
- `joined_along`: the cover along the edge's line or curved carrier;
- `edges_holding`: which listed edges hold a point (`edge_holds`).

`joined_cover` (pair) and `Flush::edge_cover` (union) both gather with
`joined_candidates` and read with `joined_along`; `set_holding` (pair)
and `Flush::lines_at` (union) both read vertices with `edges_holding`.
What stays per site is how a candidate is named and the union's
straight-only gate. The witnesses (`emit_shared_rim_several`'s
order-freedom and lies-on rows) pass unchanged.

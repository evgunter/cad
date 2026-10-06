---
id: the-joined-edge-reading-has-two-implementations
kind: issue
title: The joined-edge cover reading has two implementations, the pair emitter's and the union's
status: open
opened: 2026-10-06
priority: P3
cost: E
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

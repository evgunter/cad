---
id: two-dangling-null-edges-with-one-segment-refuse-shared-vertex-crossings
kind: issue
title: Two dangling null edges with one segment nest by their codes
status: dispatched
opened: 2026-10-03
priority: P0
cost: M
branch: fuse/one-arc-struts
---


## What

At a vertex that several crossing pairs cut, two pairs whose
dangling null edges have one segment are two pieces touching along
both of its ends' directions (pinch lines in the corner's face).
`insert::holds_whole` (`crates/topo/src/boolean/insert.rs`) nests
them by their codes on the segment, never by mint order. One piece is
In there, the other Out (two Ins or two Outs would overlap), and the
Out strut holds the In one. The vertex between them, the holder's tip
and the inner's root, then keeps the corner's region outside both
pieces. The other way round, it would be inside both.

The code is the lower germ's forward code (`insert::segment`): the
classification's own data, read at the germ that walk order (a
certified `precedes`) puts first. So the A-major record order, which
flips with `y`'s union order, no longer reaches it.

## Where it stands

- **Builds:** `two_dangling_null_edges_with_one_segment_build_in_every_op`
  (`crates/topo/tests/union_flush_onto_edge_contact.rs`). The inner lens
  reaches past the cube. Every op builds in both operand orders, with
  `y` built both ways, and passes 3′.
- **Builds, ending in the face:**
  `two_dangling_null_edges_with_one_segment_ending_in_the_cubes_face_build_in_every_op`.
  The inner lens ends inside the cube's face: `y` built both ways, and
  notched with and without a lens in the notch (three struts nested at
  the origin). Every op builds and passes 3′.
- **Order:** `every_tied_strut_witness_holds_with_its_vertex_pairs_reversed`
  reruns every tied-strut witness with the reductions' vertex pairs
  reversed (`topo::test_support::with_vertex_pairs_reversed`).

## Owed

Nothing beyond the merge.

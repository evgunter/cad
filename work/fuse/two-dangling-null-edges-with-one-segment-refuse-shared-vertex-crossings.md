---
id: two-dangling-null-edges-with-one-segment-refuse-shared-vertex-crossings
kind: issue
title: Two dangling null edges with one segment nest by their codes; the witness whose pinch lines end in the face fails beyond the origin
status: open
opened: 2026-10-03
priority: P0
cost: M
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
- **Pinned:** `two_dangling_null_edges_with_one_segment_ending_in_the_cubes_face`.
  The inner lens ends inside the cube's face, and each case fails
  beyond the origin, on another row:
  - `y = cut ∪ lens` builds in all six. `y ∖ cube` fails 3′ at the far
    ends (`a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped`).
  - `y = lens ∪ cut` refuses `RingHomingAmbiguous` in all six
    (`work/tang/a-pierce-strut-at-a-pinch-has-no-vertex-off-the-run.md`).
  - Notched, with and without a lens in the notch: both ∩ build clean,
    and `y ∖ cube` fails 3′ as above. The ∪s and `cube ∖ y` refuse
    `JoinDesync` (`work/cleave/a-discarded-vertex-between-nested-struts-has-two-kept-copies.md`).
- **Order:** `every_tied_strut_witness_holds_with_its_vertex_pairs_reversed`
  reruns every tied-strut witness with the reductions' vertex pairs
  reversed (`topo::test_support::with_vertex_pairs_reversed`).

## Owed

Flip the pinned cases as their rows land.

---
id: a-vertex-crossing-both-sides-of-a-pinch-refuses-shared-vertex-crossings
kind: issue
title: A vertex that crosses into both neighbourhoods of an operand's pinch refuses SharedVertexCrossings: the insertion handles one crossing pair per vertex
status: closed
opened: 2026-10-02
priority: P0
cost: H
closed: 2026-10-03
pr: 3927
---



## What

An operand whose own contact left two vertices at one point (a pinch:
two bricks touching along an edge keep the edge as two coincident
edges, so there are two vertices at each end) meets the other operand's
vertex there. The vertex-vertex lane pairs that vertex with both. When
only one pair crosses, the other touches, inserts nothing and builds
(`crates/topo/src/boolean/mod.rs`, the VV classification in the
reduction, reads every pair before the first insertion).

When **both** pairs cross, each pair's null edges would split the
shared vertex's orbit that the other pair read. `insert_null_pairs`
(`crates/topo/src/boolean/insert.rs`) works one pair at a time on one
orbit, so the reduction refuses `BooleanError::SharedVertexCrossings`,
naming the shared vertex, its operand and its two partners. Before `fuse/corrupt-operand-edge-contact` it
refused `CorruptOperand { operand: B, corruption: Vertex }` instead,
which blamed the operand's integrity for a classification it could not
make.

## Witness

`crates/topo/tests/union_flush_onto_edge_contact.rs`,
`a_vertex_crossing_both_sides_of_a_pinch_refuses_typed`:
- the pinch is `q1 = brick((0,1),(0,1),(0.5,1.5))` ∪
  `q3 = brick((-1,0),(-1,0),(0.5,1.5))`, along the z-axis;
- B is `prism_z` over the triangle (0,0), 190°, 80° (unit radius),
  z ∈ (0.5, 1), whose corner wedge on the axis overlaps both bricks'
  quadrants.

The state is buildable, since the union is a solid, so the refusal is
a missing arm, not a ruling. **P0** (FUSE orchestrator's ruling,
2026-10-02, on the parent row's rule): the pinch is itself a boolean
output (`union_with(q1, q3)`), and the next boolean with a valid
operand refuses it, so a boolean output is not a legal operand.

A second witness, from the same suite's
`a_four_row_remap_group_certifies_a_subtract_of_two_pinches` fixture.
That pinch, ∪ the complementary pinch `brick((-1,0),(0,1),(1,2))` ∪
`brick((0,1),(-1,0),(1,2))` (the four quadrants around one axis),
refuses `SharedVertexCrossings` in both operand orders. The subtract
builds.

## Owed

Classify a vertex against the union of the coincident vertices'
neighbourhoods, or re-read the later pair against the orbit pieces
the earlier insertion left (each a vertex at the same point, joined by
null edges). Either reaches `run_fan`'s one-cyclic-orbit assumption.
When it lands, that test's expectation flips to a build at its volume.

## Closed (FUSE, PR 3927, 2026-10-03)

The reduction now plans every v-v pair before it mints any
(`plan_null_pairs`, then `mint_plan`). `reconcile_shared` keeps the
runs disjoint, struts are minted first in angular order, and a fold
along a shared edge is reversed (`recl::Reversed`). With that, a vertex
crossing into both neighbourhoods of a pinch builds under ∪, ∖ and ∩
in both orders and passes 3′. So do three and four crossings at one
corner.

The row's own witness wedge was clockwise, so it was an inside-out
operand. That is filed on cleave as
`an-inside-out-operand-passes-the-boolean-gates-as-its-complement`
(P1); every test uses the counterclockwise wedge. This row's second
witness, two pinches crossing on one line, still refuses typed and
moves to `two-pinches-crossing-on-one-line-refuse-their-union` (P0).
The tie and interleave arms are in
`shared-vertex-crossings-that-tie-or-interleave-are-unprobed` (P2),
with the tie witness pinned. A pinch line through a face interior
fails 3′, and the same failure happens on main without a shared
vertex; that is filed as
`a-pinch-line-crossing-a-face-interior-drops-the-pinchs-records`.

Review tier: single FULL, with one fix pass. The reviewer reproduced
every red-when-off claim, matched every volume with an independent
clipper, and fuzzed 60 further ops, all at the right volume and
passing 3′.


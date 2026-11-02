---
id: a-pinched-cone-loop-reaches-the-gate-unnamed
kind: issue
title: A cone loop through the apex between a seam vertex's two copies refuses props_cone_loop_vertex, the azimuth turn lost at the apex
status: closed
opened: 2026-11-02
closed: 2026-11-02
priority: P2
cost: M
refs: [props-measures-a-cone-loop-pinched-at-its-apex]
---


Found by SPLITDOOR unit B stage B2 (`splitdoor/cone-apex-edge-face`),
pinning a box edge through a cone's apex
(`crates/sweep/tests/a_cone_apex_on_an_edge.rs`,
`an_edge_along_the_axis_holds_a_quarter_of_the_cones_top`).

## What

The 60° wrap-tipped cone (`common::wrap_tip::one_wrap_edge`) against the
box `[0, 2] × [0.55, 1.6] × [0, 2.1]`, its edge along `y` the cone's
axis, turned `0` about it: `cone ∪ box` refuses

`ResultInvalid { errors: [VolumeUncomputable { source: Face { face, source:
ConeLoop { what: "props_cone_loop_vertex" } } }] }`

At 30° and 45°, and at other turns, the same pose's remainder ops either
build or refuse `ResultPinchedAtConeApex` by name
(`boolean::ops::pinched_at_cone_apex`).

`props_cone_loop_vertex` (`geom_brep::props::curved`,
`require_cone_loop_vertices_distinct`) refuses a cone loop with two
non-adjacent vertices that coincide in space. The named reading refuses
a loop that starts two of its EDGE half-edges at one vertex key decided
the apex. So this loop meets itself at the apex in a way the named
reading does not see. Two readings are open:

- the loop passes the apex vertex through a member the named reading
  skips (not an `Edge` start), and the refusal only needs naming;
- two distinct vertices sit at the apex, the box edge's split vertex
  not identified with the cone's apex vertex. That would be a
  topological defect of the result, which the gate refuses but which
  the vertex pair should never leave.

## Where to look

Build the result short of the gate (the at-rest gate refuses it) and
list the cone face's loop vertices at the apex: their keys tell the two
readings apart. The pin accepts this refusal for `∪` and `cone ∖ box`
only, beside the named pinch.

## Settled (2026-11-02): a props gap, no topology defect

The result short of the gate shows the pose's cone face, the wrap-tipped
60° cone less the box's quadrant above `y = 0.55`, with one loop:

`v4 (0.779, 0.55, 0) → arc → v5 (0, 0.55, 0.779) → ruling → v6 the apex
→ ruling → v4 → wrap → v2 (1.732, 0, 0) → base circle → v2 → wrap → v4`

- One vertex is at the apex, visited once, so the named reading is right
  not to fire.
- The vertex visited twice is `v4`, by one key: at turn 0 the box's side
  `z = 0` holds the wrap seam, so `v4` is both the cut's corner and the
  wrap edge's top. Its two visits are the seam's two copies of one
  vertex, a period apart. `props_cone_loop_vertex_sheet` admits them when
  the loop's azimuth turns whole turns between them.
- Between the two copies the loop passes through the apex (a ruling in
  at azimuth 90°, out at 360°), where the apex has no azimuth. The turn
  is lost there, and `require_cone_loop_vertices_distinct` refuses.

The face is valid: the lower band and the upper three-quarter sector,
glued along the seam. The fix is in props. The loop's turn across the
apex is the angle between its in and out rulings, taken on the face's
side, which needs the face's material side at the apex. Why the same
pose at 30° and 45° measures is not traced: at turn 0 both build every
op, for a reason not yet traced.

Filed on CONEARM's slate, beside its sibling
`props-measures-a-cone-loop-pinched-at-its-apex` (a loop meeting itself
AT the apex), because `crates/geom-brep/src/props/curved.rs`'s cone arm
is CONEARM's. The fix is left to this row, not to SPLITDOOR's B2.

## Closed (2026-11-02, SPLITDOOR)

Fixed in the B1 fix pass (`splitdoor/cone-apex-vertex-pairs`, PR 6345).
`cone_loop_winding` reads a loop's vertex azimuths from the edge that
leaves the apex, so a seam vertex's two copies are never compared across
it. The loop's verdict no longer depends on where the loop starts. Rows:
- `props::curved::apex_start_rows`: one measure from every start;
- `a_small_box_whose_side_holds_the_wrap_seam_builds_every_op`;
- on B2, the 60° turn-0 pose of
  `a_cone_apex_on_an_edge::an_edge_along_the_axis_holds_a_quarter_of_the_cones_top`,
  which builds now that the pin no longer accepts this refusal.

— SPLITDOOR

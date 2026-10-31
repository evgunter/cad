---
id: eight-sweep-slow-rows-are-red-on-main
kind: issue
title: Eight slow-set rows in crates/sweep/tests fail on main (a7fdbf5db8): boolean refusals on extrude-built operands, one escalation, one quadrature gap
status: open
opened: 2026-10-31
---


## Finding (CARVE, 2026-10-31)

Found while reviewing CARVE's PR 6138 (the sweep frame). Eight rows in
the slow set of `crates/sweep/tests/` fail on origin/main at
`a7fdbf5db8`. They fail the same way at that PR's head, which does not
reach them: every one builds its operands with `extrude` (or a
revolve) and none calls a sweep. The per-PR gate never runs the slow
set, so these are nightly reds.

| row | failure at main `a7fdbf5db8` |
|---|---|
| `a_pierce_held_in_a_curved_face::a_cylinder_wall_holding_the_pierce_builds_in_every_op_and_order` (`:471`) | turn 0.5, `(rod ∪ square) ∖ plate`: tier 3′ `FaceThroughEdgeWedge { edge: 31v1, face: 17v1 }` at (1.851, 1.192, 0.72) |
| `a_pierce_held_in_a_curved_face::a_wall_the_ruling_pierces_twice_builds_in_every_op_and_order` (`:447`) | turn 0.5, `(rod ∪ square) ∖ slab`: `EdgeInFace { gap: CurvedFace }` |
| `a_ruling_lying_on_a_wall::a_prism_edge_on_the_tubes_wall_builds_every_op_undeclared` (`:232`) | turn 0.3, `t ∖ b`: `EdgeInFace { gap: CurvedFace }` |
| `a_ruling_lying_on_a_wall::a_ruling_across_an_ellipse_builds_every_op_undeclared` (`:396`) | turn 0.3, `t ∖ b`: `EdgeInFace { gap: CurvedFace }` |
| `a_ruling_through_a_box_corner_edge::a_post_through_the_near_wall_piece_keeps_the_rests_on_their_piece` (`:141`) | X edge at (0, 0), `w ∖ (box ∪ post)`: `EdgeInFace { gap: CurvedFace }` |
| `a_ruling_through_a_box_corner_edge::a_wall_through_a_box_edge_records_the_rest_on_the_piece_that_holds_it` (`:141`) | X edge at (0, 0), `w ∖ b`: `EdgeInFace { gap: CurvedFace }` |
| `bisector_nodes::the_near_node_poses_at_the_witness_band` (`:604`) | A's cap δ = 1e-4, direct `(A, B)`, ∪: `Escalated { Coincidence(EdgeOnCurvedFace, Moot) }`, margin −5.0e-9 against the band (1e-9, 1e-8), `bool_line_face_carrier_offset` |
| `conic_edge_curved_face::a_rim_crossing_reaches_the_join` (`:338`) | ball r 0.2 at (0.5, 0, 0.35): `VolumeUncomputable { QuadratureUnsupported { "conic trim on a cone/sphere/torus chart …" } }` |

Six of the eight are one shape: a boolean against a curved wall
refuses `EdgeInFace { gap: CurvedFace }` or tier 3′. They probably
share one cause on the boolean's ground (`crates/topo/src/boolean/`),
which this row does not diagnose. It is filed here, on TINT's slate,
because the defect it reports is the suite's: eight rows red on main
that no per-PR gate reads. Whoever takes it bisects main for the
commit that turned them red and routes each cause to the program that
owns it.

Log: the reviewer's run on main,
`~/.local/share/cad-work/carve-frame-review-scratch/main-targeted.log`
(local to the machine it ran on). Four of the eight (the
`a_pierce_held…` and `a_ruling_lying…` pairs) were reproduced again at
PR 6138's head, merged with main.

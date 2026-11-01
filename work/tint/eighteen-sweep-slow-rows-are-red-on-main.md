---
id: eighteen-sweep-slow-rows-are-red-on-main
kind: issue
title: Eighteen slow-set rows in crates/sweep/tests fail on main (a7fdbf5db8, 80b4615683), none of them a sweep: boolean refusals, quadrature gaps, an escalation
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

## Ten more, on main `80b4615683` (CARVE, 2026-10-31)

PR 6138's fix pass ran the whole `sweep` suite with no profile
filter, in a release build. Ten more rows failed at its head. The same
ten fail at origin/main `80b4615683` (the PR's merge base), run in a
release build in a separate worktree, so they are inherited too. None
of them calls a sweep or a loft.

| row | failure |
|---|---|
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_centre_inside` (`:356`) | `A ∪ B`: `VolumeUncomputable { QuadratureUnsupported { "curved-cut face half-edge carries a pcurve with no harmonic form …" } }` |
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_centre_on_the_wall` | `A ∩ B`: the same |
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_centre_outside` | `A ∩ B`: the same |
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall` | `A ∩ B`: the same |
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim` | `A ∪ B`: `QuadratureUnsupported { "conic trim on a cone/sphere/torus chart …" }` |
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_2_across_the_rim` | `A ∩ B`: the same |
| `torus_doors_join::a_rod_across_the_inner_bound_stops_at_the_join` (`:152`) | an `assert_eq` on where the rod stops |
| `torus_doors_join::a_rod_off_the_axis_line_stops_at_the_join` | the same |
| `torus_doors_join::a_square_rod_on_the_outer_equator_stops_at_the_join` | the same |
| `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` (`:415`) | at ε 1e-9, β 0.5, spin 0: the refusal list reads `"ring volume"` / `"quadrature volume"` where the row expects germs |

Seven of these ten fail on volume quadrature (`QuadratureUnsupported`
on a curved-cut face's pcurve, or on a conic trim), which matches
`conic_edge_curved_face::a_rim_crossing_reaches_the_join` above. That
suggests one recent change to the quadrature's admitted faces, and it
is the first thing to bisect.

## 2026-10-31 — seven of the first eight on PR 5793 (CLEAVE)

PR 5793 (merged with main `aed0b727c8`) turns seven of the first eight
green. The `a_pierce_held…`, `a_ruling_lying…` and
`a_ruling_through_a_box_corner_edge` pairs are the boolean refusing
`EdgeInFace` (`CurvedFace`) where an edge lies in a curved face with
its material parted there (or, for one pierce row on main, shipping the
one-edge form that tier 3′ refuses): they are re-pinned to that
refusal, owed back on
`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`.
`bisector_nodes::the_near_node_poses_at_the_witness_band` escalates on
`bool_line_face_carrier_offset` with all three of the rim's samples in
band of the other rod's wall, none off: main raises that reading. PR
5793's gate (`slit::holding`) reads the edge's faces against the wall
first, and they leave it on opposite sides (`Crossed`), so the edge
does not lie in the wall and the reading is not raised.
`conic_edge_curved_face::a_rim_crossing_reaches_the_join` and the ten
below are not this PR's ground.

---
id: a-cell-holding-two-converging-walls-is-left-to-a-smaller-cell-that-never-comes
kind: issue
title: beside_curve's last arm reads Local where a cell holds two walls (two curves, or a curve and a line) that converge at a point, tangent or at a corner with a third edge, so no smaller cell holds one alone
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

Found by EMBED's lane `embed/trace-cylinder` (2026-11-21), second pass
over the cylinder buckets of the per-solid census
(`measure/gate/census-c2a0c41f40.json` on `embed/gate-measure`).
**11 tests** of the plane/cylinder, cylinder/plane and cylinder/cylinder
edge buckets end at `beside_curve`'s last arm (`crates/topo/src/injective.rs`,
`None => Unpresented::Local`). That arm is reached when the cell's local
boundary holds more than one wall, every curved one is a curve some
presentation reads, no corner at a shared vertex is unread, and no
corner presentation answered. Its doc's reason is "a smaller cell may
hold the curve alone, or the corner alone". Where the walls converge at
a point, every cell holding that point holds both, so the pair splits to
`Budget(Depth)`.

This is not `beside-curve-types-a-cell-by-its-first-unread-edge`: that
row is the arm before this one (`Some(&i) => unread(..)`, a curve no
presentation reads), and here every curved wall is read
(`reads_curve` true for each; the `CVBC` rows of
`embed/trace-cylinder-measure`).

Seven traced, with the walls printed at the refusing cell:

- **Two walls in reach with no vertex of theirs in reach (5).**
  - `tang_circle_cylinder::parallel_cylinders_that_pierce_build_at_the_closed_form`:
    two circles `8v1`, `10v3`, meeting at `9v1`, which no cell's ball
    holds. The circles are tangent there by the fixture.
  - `band_planar_oblique_fillet::a_brick_through_a_steep_elliptic_end_builds_in_every_op`:
    two ellipse arcs, both shared.
  - `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_splits_alike_under_either_normal`:
    an iso circle and an ellipse that touches it.
  - `bisector_lane_stubs::a_wall_stored_far_off_its_faces_builds_at_the_closed_form`:
    an ellipse and a ruling.
  - `band_turn_overrun_fillet::the_sweep_builds_every_convex_overrun_at_the_probes_closed_forms`:
    a line and an ellipse.
- **A corner plus a third wall (2).**
  - `band_turn_overrun_fillet::a_fillet_turn_builds_at_the_chamfers_band_and_escalates_in_its_own…`:
    a line and an ellipse at the shared vertex `13v1`, and a second
    ellipse ending at `14v1` in reach.
  - `band_turn_overrun_fillet_model::the_model_reads_the_box_corner_by_hand_and_every_witness`:
    two lines and a circle, their vertices in reach, none shared.
- **Not traced (4).** `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_at_its_seam_splits_alike…`
  prints a circle and two lines at two shared vertices but was not read
  further. Not printed: `reach_split_gate_window`, `split_section_rings`
  and the mesh row `winding_rows`.

## Valid or in band

Valid as built. Two-walls-converging includes tangent contacts inside a
face (two tangent circles, an ellipse touching a rim). Those faces have
a zero-angle corner or a tangency of two boundary edges by the fixture's
construction. Whether a zero-angle corner is owed a reading or is in
band is the question for the unit; the fixtures build them as finished
results.

## What a fix needs

A presentation for a cell holding two converging walls. Where they meet
at a vertex in the cell, it is the corner presentation (`present_corner`)
extended to a corner with a third edge in reach. Where they converge
with no vertex in reach (tangent), it is a two-sided graph: the face's
part of the cell between two graphs over one axis. Cost M; the tangent
case may be H.

## Evidence

- Cheapest reproducer: `sweep::all band_turn_overrun_fillet::a_fillet_turn_builds_at_the_chamfers_band_and_escalates_in_its_own…`
  (0.22 s).
- Checked: which walls the cell holds and that each is read, printed.
  The tangency of the converging pairs is read off the fixtures,
  unchecked.

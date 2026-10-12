---
id: cylinder-quadrature-integrates-loops-that-close-only-to-the-vertex-tolerance
kind: issue
title: The cylinder quadrature integrates pcurve chains that close only to the vertex tolerance, and pads no junction gap
status: open
opened: 2026-10-10
---


## What

`geom_brep::props::quad::cylinder_cut_face_rounds` sums `∮ u dv`
over each loop's stored harmonic pcurves, edge by edge
(`harmonic_edge_integral`). Green's form is exact for a closed chart
curve, and consecutive pcurves meet only to the vertex tolerance:
measured at the junctions' midpoints (each end read at its
`TrimEdgeQ::limits`), single-loop faces in
`pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound` miss by up
to `1.12e-9` (`R·|Δu| + |Δv|`) at ε = 1e-9. On the turned-bar rows of
`verbs_germarms` the midpoint gaps are `≤ 2.3e-15`; the `≈ 1.4e-9` first
recorded here for them was the endpoint brackets' WIDTH, not a gap
(review R2 of PR 4557, re-measured). The open chain's integral differs
from the closed region's by the gap's connector, `≈ ½(u₁ + u₀)·Δv`, and
no pad covers it: the endpoint-bracket pad covers a `t` bracket's width
and the map-residual pad a pcurve's distance from its carrier, neither
the distance between two carriers' ends.

`require_no_period_jump` (`props_quad_junction_period`, added by
`flux/ellipse-ring`) decides only that a gap is not an azimuth period;
it does not close the gap.

## Tried, and why it did not land

Closing each junction with its chart segment, `½(u₁ + u₀)(v₀ − v₁)` in
interval arithmetic plus a `g²/R` sliver pad, is the rigorous form. It
reds two rows: `one_segment_loop::a_split_through_the_seam_builds_as_the_two_arc_form_does`
(`props_quad_converged` escalates, margin 9.7e-9) and
`verbs_germarms::a_thin_bar_turned_about_two_axes_gets_through_the_join`
(`QuadratureBudget`, width 3.4e-6 against 1.0e-6). The cause is the
endpoint brackets, not the gap: a LINE edge's `trig0` is the
whole-circle bracket (`topo::props::quad_lane::trig_at_start`, "multiplies
away"), and its fitted pcurve carries trig amplitudes of ~3.5e-10, so
its endpoint `v` is a 7e-10-wide bracket, and the connector term carries
that width into every round at a constant floor.

## The shape of a fix

A line's endpoint read off its vertex point rather than off the
whole-circle trig bracket, so a junction's chart points are thin; then
the connector term and its sliver pad cost the enclosure what the gap
is, not what the bracket is.

## Since `flux/cylinder-band-quad`

The lane now reads `−∮ (v − v_a) du` (`v_a` the middle of the face's
edge-start levels, `quad::level_anchor`), and the per-junction
`require_no_period_jump` is gone: each loop's and the face's winding are
decided instead (`quad::loop_windings`). The residue is unchanged in
kind: an open chain's integral now differs from the closed region's by
`≈ −(½(v₁ + v₀) − v_a)·Δu` at each junction gap, which no pad covers.
The anchor keeps the lever at the face's own height rather than its
distance from the chart origin, so the term is no larger than the old
`½(u₁ + u₀)·Δv` lever on a face of comparable extent.

---
id: a-spline-span-against-a-spline-face-has-no-interior-arm
kind: issue
title: A spline span against a spline face has no interior arm: lily wall 8 refuses CurvedBooleanUnsupported there with a NURBS tip seam of the blade
status: closed
opened: 2026-10-27
closed: 2026-10-27
priority: P0
cost: H
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane, a-unions-flush-reading-cannot-follow-a-nurbs-carrier]
branch: splice/spline-span-arm
---

Filed by the SPLICE lane that built the analytic span × spline face
arm (`geom_brep::curve_face::spline_face`, row
`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`).

## What

The curve × face table (`geom_brep::curve_face::route`) routes
`(Spiric | Nurbs, Nurbs | Approx)` to `Door::SplineFace` unbuilt, and
the boolean's spline-face door (`topo::boolean::reduce::spline_face`,
then `spline_interior`) refuses such an edge whose boundary pass reads
`Clear` with `CurvedBooleanUnsupported { operand, face, kind: Nurbs }`.

**Lily wall 8 is this case, not the analytic one.** Measured on main
at `6117d399` with a probe in `spline_face`'s `Clear` arm: both
refusals `sweep`'s `a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`
pins are a `Nurbs` carrier on params `(0, 1)` (the blade's tip seams,
ends `(0, ±0.085, 0)`), against the sheath's ridge wall. The analytic
arm does not move it; `demos/tour/src/lily.rs` wall 8 and the sweep row
still pin `CurvedBooleanUnsupported { operand: B, kind: Nurbs }`.

## The likely shape

The analytic arm's state `(t, u, v)` on `C(t) − S(u, v)` carries over:
a spline span's point and tangent over a parameter window are its
control hull and its derivative's (each piece within one knot span,
none straddling a crease of the span's own knots, as the ring composite
already holds), where the analytic arm reads a closed form. The walk,
the Krawczyk test and the isolating boxes are unchanged.

## Closed

`geom_brep::curve_face::spline_face` reads a NURBS span as it reads an
analytic one: the curve side is one enum (`Path`), whose NURBS arm reads
the span's point and tangent over a parameter window off its ruled
sheet, `S(t, s) = C(t)`, through `NurbsBoxes` (the control hull and the
hodograph's quotient bound, cut to the window). The state is cut at the
span's creases as well as the chart's, through the one crease rule
(`Cuts::touch`, now over all three coordinates). The walk reads past an
end only as far as the carrier's domain runs; an end the walk cannot
read past, or one on the chart's domain edge, is accounted for by an
end's box, a box the walk reads that holds the end's state and on which
every solution lies within the band of the end
(`krawczyk::solution_reach`, the mean value theorem with a contraction
bound; no existence claimed). The table routes `(Nurbs, Nurbs | Approx)`
to the built arm; a spiric span stays unbuilt.

Lily wall 8 moves past every face reduction in both operand orders and
stops at the result's shared-rim seam, re-pinned in
`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim` and
`demos/tour/src/lily.rs` and filed as
`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`.

Witness rows (`curve_face::spline_face::rows`):
`a_nurbs_span_crosses_a_bicubic_patch_once`,
`a_nurbs_span_crosses_a_bicubic_patch_twice`,
`a_nurbs_span_tangent_to_a_dome_escalates`,
`a_nurbs_pass_within_the_band_escalates`,
`a_nurbs_pass_past_the_band_is_clear`,
`a_meeting_on_a_crease_of_the_span_refuses_and_one_beside_it_reads`,
`a_nurbs_meeting_on_a_crease_of_the_chart_refuses_and_one_beside_it_reads`,
`a_rational_span_crosses_a_rational_quarter_cylinder`,
`a_span_ending_on_the_chart_at_its_domains_end_is_the_ends`,
`a_crossing_past_the_band_of_a_domain_end_is_reported`,
`two_nurbs_meetings_within_the_band_of_each_other_refuse`; and in topo
`a_nurbs_edge_pierces_a_spline_face_inside_its_trim_and_passes_it_outside`.

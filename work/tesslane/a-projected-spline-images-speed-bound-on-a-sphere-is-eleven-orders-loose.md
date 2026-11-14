---
id: a-projected-spline-images-speed-bound-on-a-sphere-is-eleven-orders-loose
kind: issue
title: a projected B-spline image on a sphere chart bounds its speed at 1.4e12 where it moves at 21, so the sphere arm refuses its face at the chord budget
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by `one-chart-table-and-the-lane-takes-every-analytic-chart` (branch `tesslane/chart-table`), the first reader of `geom_brep::Pcurve::speed_bound` for a projected B-spline image on a sphere chart.

**Measured.** `crates/mesh/tests/reach_probes.rs::a_sphere_face_bounded_by_a_spline_section_refuses_in_the_chord_pass` (a ball of radius 0.3 centred `(0.5, 0, 0)`, `y`-poled, against a rod of radius 0.5 along `z`; `cylinder ∩ ball`): the sphere face's section edge (a `Curve3::Nurbs` carrier, its `Pcurve::Projected` image) has `speed_bound(t0, t1) = (1.4487e12, 1.4487e12)` over its span of 0.25, where a 20 000-sample finite difference of `Pcurve::eval` reads `(21.07, 6.32)`. Its `chart_box` over the span is `u ∈ [π/2, π]`, `v ∈ [−1.266, −2.8e−16]`: the image stays a latitude band away from both poles. At the trimmed sphere row's lattice step `h = 0.366` that bound asks `9.9·10¹¹` chords of the edge, past `trimmed::MAX_SPHERE_EDGE_CHORDS` (measured before unit 1's conic-only rule landed, when this face reached the rule through the chart table and refused `SphereTrimShape { ChordBudget }` at every δ).

**Where.** `crates/geom-brep/src/pcurve_cache/speed.rs`, `projected_speeds` (the projected arm of `Pcurve::speed_bound`, "`sup|C′|` over the chart's least lever arm"), and the lever `Levers::of` reads for a net carrier. The bound is safe-side and eleven orders loose; whatever sets the lever near zero on this image (the control net's hull reaching the axis, or a rational weight's derivative bound) is unmeasured.

**What it costs.** Today nothing reads it: `mesh::chords::chart_image` refuses a projected image of a spline carrier on every trimmed analytic arm ("the trimmed lane bounds the UV speed of a conic's image only"), so a sphere, cone or torus face a B-spline section bounds refuses there. Lifting that rule meets this bound first: the face would refuse at the chord budget instead of meshing.

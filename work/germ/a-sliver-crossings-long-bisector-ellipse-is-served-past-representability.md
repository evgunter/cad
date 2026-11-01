---
id: a-sliver-crossings-long-bisector-ellipse-is-served-past-representability
kind: issue
title: A sliver crossing of equal cylinders serves the long bisector ellipse past what f64 evaluates within the band
status: open
opened: 2026-10-19
priority: P2
cost: M
design: true
---


Found while landing `circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band`
(PR 5156).

`geom_brep::intersect::cylinder_cylinder_section`, equal radii, axes
meeting at an angle `φ` that `cc_axes_parallel` reads definite: the
bisector plane along both axes (normal `â₁ − â₂`) cuts the ellipse
with semi-axes `r/sin(φ/2)` and `r`. Its stored form
`c + u·a·cos t + v·b·sin t` evaluates near the axes' meeting point
with an error of about `a·u` (`u` the unit roundoff), so it is within
the band only while `r/sin(φ/2) ≲ ε/u`: at `r = 1` that needs
`φ ≳ 2e-4` rad at ε = 1e-12, `≳ 2e-7` at ε = 1e-9. Between the pose's
band (`φ·lever ≈ Kε`) and that, the table serves a carrier no
certificate can hold.

Measured: `sweep` `bisector_nodes::a_rod_glued_in_its_bore_answers_alike_in_either_order`,
the rod along `y` at `1e-7°` (φ ≈ 1.7e-9 rad, `a ≈ 1.2e9 m`) at
ε = 1e-12: ∩ refuses `Pcurves { Certify { ResidualExceeded { EnvelopeTerm(Radius) } } }`
in one operand order and `Merge(BooleanMerge(Pcurve { Certify { ResidualExceeded { MapResidual } } }))`
in the other. Sound (the certificate refuses), but by stage, so the
refusal's kind depends on the order and names no lever.

On main the same pose escalated `ellipse_axes_distinct` on the OTHER
bisector ellipse (the near-circular one, `a − b ≈ r·φ²/8`), alike in
both orders; PR 5156 retires that gate (Ev's decision on the item
above), which uncovers this. The range `φ ∈ (√(8ε/r), 2r·u/ε)` (about
`2.8e-6` to `2e-4` rad at ε = 1e-12, `r = 1`) reached this on main
already.

What PR 5156's fix pass settled, and what it did not: the long
semi-axis itself is no longer part of this. The table read the
half-angle's sine as `a₁·n₁`, off `1 − cos φ`, so it served `inf`
where that rounded to zero (`φ ≲ 1e-8`, any frame that keeps the axes'
products exact) and a semi-axis off by about `u/φ²` elsewhere
(`5.15e7` for `5e7` at `r = 1`, `φ = 4e-8`). It now reads the sine and
cosine as half the lengths of `â₁ ∓ â₂`, the shorter made ⊥ the longer,
so the semi-axis is `r / sin(φ/2)` to a few `u` of the stored axes'
angle at every `φ` the pose serves (`sweep`
`near_circular_sections::a_sliver_crossing_serves_its_long_semi_axis_to_a_few_u`),
and `Curve3::ellipse` refuses a non-finite semi-axis typed
(`EllipseInvalid::SemiAxisNotFinite`). What remains is the
evaluation error of an accurately stated long carrier, `a·u` against
the band, as above.

The question (`design: true`): where the long carrier's
representability is decided. The table could read it as a margin of
its own (the carrier's evaluation error against the band, levered at
the reach that consumes it) and refuse typed with the pose's lever
(turn the axes further apart, or together until parallel); or the
crossing lane could serve the long branch over the consumed reach as a
pair of near-rulings. Tier 3's representability margins
(`Curve3::representability_margins`) already meter the frame at the
major semi-axis at rest.

---
id: the-curve-pair-walk-exhausts-on-an-end-meeting-at-a-coarse-band
kind: issue
title: the curve-pair walk exhausts its piece budget on a spline span meeting a circle at a span's end when the band is coarse
status: open
opened: 2026-11-17
---


(Found by the IMPORTGATE implementer of `importgate/knot-place`, which reads the import's C⁰-knot line against each boundary edge through `geom_brep::curve_pair::meet`.)

## What

`geom_brep::curve_pair::meet` (`crates/geom-brep/src/curve_pair.rs`, `meet`; the walk in `curve_pair/walk.rs`) refuses `MeetFault::Unsettled(Unsettled::Exhausted)` on a plain end meeting of a spline span and a circle at the band `Band::new(1e-5, 1e-4)`, the chart band of a millimetre-scale STEP file (`tests/fixtures/wild/stepcode/dm1-id-214.stp`, ε_in = 1e-5 m):

- the circle: centre `(0.010055109324576903, 0.010000000000000005, 0.06500000000000034)`, axis `(0, -1, 0)`, radius `0.005000000000000101`, `u_ref` `(-1, 0, -6.1e-14)`, over `(0, 2π)`;
- the spline: degree 1, knots `[0, 0, 1, 1]`, controls `(0.012555109324577004, 0, 0.06066987298107802)` and `(0.012555109324577004, 0.010000000000000005, 0.06066987298107802)`, weights 1. Its stop lies on the circle at `t = 2π/3`, and it leaves the circle's plane square to it.

What was measured (one `found` meeting each time, the end meeting):

| pair | band (1e-5, 1e-4) | band (1e-9, 1e-8) |
|---|---|---|
| spline vs circle over `(0, 2π)`, either order | `Exhausted` | 1 meeting |
| circle cut at the meeting, `(2π/3, 5)` or `(0.5, 2π/3)` | `Exhausted` | 1 meeting |
| a degree-1 spline lying in the circle's plane, radial, ending on it | `Exhausted` | 1 meeting |
| the same spline as `Curve3::Line` (the table's LineConic arm) | 1 meeting | 1 meeting |
| the spline moved off the circle by its own length | 0 meetings | 0 meetings |

So the walk settles the pair at a fine band and when the spans are apart, and the closed form settles it at either band; at the coarse band it cannot clear the pieces about a meeting at a span's end, square or radial. A circle of radius 5e-3 m against a band of 1e-4 m is 2% of the radius, which is ordinary for a millimetre file.

## Where it bites

The import's C⁰-knot split (`crates/step-import/src/knot_split.rs`, `split_face_once`) reads the knot's iso curve against every boundary edge. It reads a degree-one iso of two controls as `Curve3::Line`, which is exact and takes the closed form, so dm1's straight cylinder ruling imports. A curved iso at a coarse band meeting a rim at its end still takes the walk, and its face refuses `C0KnotUnsplit` ("did not settle where they meet at the file's band") until the walk settles it.

## Reproduce

The table above was taken by a scratch test calling `meet` on these spans at both bands; build the two spans from the numbers above.

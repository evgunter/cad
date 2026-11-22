---
id: the-curve-pair-walk-exhausts-on-an-end-meeting-at-a-coarse-band
kind: issue
title: the curve-pair walk exhausts its piece budget on a spline span meeting a circle at a span's end when the band is coarse
status: open
opened: 2026-11-17
priority: P2
cost: M
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

In any crate that depends on `geom`, `geom-brep` and `geom-core`, as a test:

```rust
use std::sync::Arc;
use geom::{Curve3, NurbsCurve3};
use geom_brep::curve_pair::{Span, meet};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Vec3};

let q0 = Point3::new(0.012555109324577004, 0.0, 0.06066987298107802);
let q1 = Point3::new(0.012555109324577004, 0.010000000000000005, 0.06066987298107802);
let spline = Curve3::Nurbs(Arc::new(
    NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![q0, q1],
        vec![1.0, 1.0],
    )
    .unwrap(),
));
let circle = Curve3::circle(
    Point3::new(0.010055109324576903, 0.010000000000000005, 0.06500000000000034),
    Vec3::new(0.0, -1.0, 0.0),
    0.005000000000000101,
    Vec3::new(-1.0, 0.0, -6.106226635438239e-14),
);
let a = Span::new(&spline, (0.0, 1.0));
let b = Span::new(&circle, (0.0, core::f64::consts::TAU));
// Err(MeetRefusal { fault: Unsettled(Exhausted), found: [one meeting], .. })
let coarse = meet(&a, &b, Band::new(1e-5, 1e-4).unwrap());
// Ok(Meetings { points: [one meeting], .. })
let fine = meet(&a, &b, Band::new(1e-9, 1e-8).unwrap());
// The same spline as the line it is takes the LineConic arm: Ok, one meeting.
let line = Curve3::line(q0, q1 - q0);
let closed = meet(&Span::new(&line, (0.0, 1.0)), &b, Band::new(1e-5, 1e-4).unwrap());
```

The other rows of the table swap the spans; cut the circle at the meeting, `t = 2π/3` (`(2π/3, 5)` and `(0.5, 2π/3)`); replace the spline's controls by `[q1 + (q1 − c), q1]`, `c` the circle's centre (radial); or replace them by `[q0 − (q1 − q0), q0]` (apart).

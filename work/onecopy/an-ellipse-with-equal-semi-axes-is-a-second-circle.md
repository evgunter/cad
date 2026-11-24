---
id: an-ellipse-with-equal-semi-axes-is-a-second-circle
kind: issue
title: Curve3::Ellipse admits equal semi-axes, so a circle has two 3-D representations
status: open
opened: 2026-11-22
priority: P2
cost: M
---



Filed by PROFILE's cut of the ovoid build (PR 7289), from both of its
designers' "off the question" findings (fork log row 298), checked in
the tree on 2026-11-22.

## What

A circle has two 3-D representations, `Curve3::Circle` and a
`Curve3::Ellipse` whose semi-axes are equal, against `[one-way]` (few
kinds of thing, one way to do each).

- `geom::Ellipse::new` (`crates/geom/src/curves/conic.rs`) takes any
  two finite, not definitely negative semi-axes (`stored_size`), equal
  ones included.
- The deciding door `Curve3::ellipse` (`crates/geom/src/curves.rs`)
  decides only `ellipse_minor_positive` (the smaller semi-axis
  positive); nothing decides `semi_u − semi_v` clear of the band.
- So a reader that matches `Curve3::Circle` for a circle's closed forms
  (the circle-pair margins, `geom_core::Circles`) never sees an
  equal-axis ellipse as one, and two records of one circle can compare
  as different kinds.

The surface side already refuses the twin: `[surfaces]` makes a
spheroid's two radii "differ past the band", and the profile's ellipse
segment refuses a factor within the band of 1 (OVOID's
`a-profile-segment-is-an-ellipse-arc`). OVOID's plane × spheroid
section (`a-plane-cuts-a-spheroid-by-pulling-the-plane-back-through-the-stretch`)
decides a circle where the plane is normal to the axis, so it mints no
equal-axis ellipse; every other `Ellipse::new` site (about 20 files
under `crates/geom-brep/src/curve_face/` and `curve_pair/`, most of them
test rows) has not been audited.

## The fix, as far as it is written

A deciding door that refuses, or classifies as a circle, an ellipse
whose semi-axes are within the band of each other, and an audit of the
sites that mint an ellipse from computed sizes (GERM's
`conic-constructions-from-computed-sizes-unaudited` is the neighbouring
audit). Whether a computed equal-axis section should become a `Circle`
or refuse is a choice for the lane.

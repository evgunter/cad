---
id: conic-quadric-doors-choose-their-first-harmonic-arm-two-ways
kind: issue
title: The circle x cylinder and ellipse root doors pick their first-harmonic arm by two different predicates over one algebra
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [non-circle-conic-edge-refuses-against-every-curved-face]
---

Found by the REACH lane that gave the ELLIPSE carrier its root lane
(`work/reach/non-circle-conic-edge-refuses-against-every-curved-face.md`).

## What

Two root doors decide the same question over the same algebra by
different predicates:

- `boolean::circle_cylinder` takes its first-harmonic ("square") arm
  when the circle's TILT `ρ·|n̂ × â|` is in the zero band, and charges
  the dropped second harmonic to the noise;
- `boolean::ellipse_roots` takes its first-harmonic arm when the second
  harmonic's AMPLITUDE `A₂ = |(c₂, s₂)|` is in the zero band, charging
  the same.

Both read `geom_brep::ConicHarmonics` (one home for the algebra), and on
a circle `A₂ = ρ²·sin²(tilt)/4r`, so the two predicates select the same
poses up to the band's scaling — but they are two spellings of one
rule, with two row families. The `A₂` form is the general one: it is
what makes the first harmonic the residual to within the noise, on any
conic, and it also routes the poses the tilt form cannot see (an ellipse
whose projection off the axis is a circle).

## The shape of a fix

One conic × quadric door (`Circle | Ellipse` × `Sphere | Cylinder`)
deciding the arm on `A₂`, the circle × sphere door's first harmonic
falling out of it as the `A₂ ≡ 0` case. It moves the circle doors'
row names, so the circle rows' pins move with it. Waits on nothing but
a lane: the ellipse door landed beside `circle_cylinder` rather than
rewriting a door another lane had in review.

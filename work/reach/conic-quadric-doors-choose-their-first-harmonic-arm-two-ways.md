---
id: conic-quadric-doors-choose-their-first-harmonic-arm-two-ways
kind: issue
title: The circle x cylinder and ellipse root doors pick their first-harmonic arm by two different predicates over one algebra
status: closed
opened: 2026-10-02
priority: P1
cost: M
refs: [non-circle-conic-edge-refuses-against-every-curved-face]
branch: reach/conic-quadric-one-door
pr: 4042
closed: 2026-10-05
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

## Closed (2026-10-05)

Merged by PR 4042. One door, `boolean::conic_quadric::conic_quadric_roots`,
answers `Circle | Ellipse` × `Sphere | Cylinder` and decides its
first-harmonic arm on `A₂`; the tilt predicate is gone, the circle ×
sphere first harmonic is its `A₂ ≡ 0` case, and the ellipse × torus arm
keeps its own door. No suite or tour pose moved (6,479 identical door
calls over topo+sweep, re-measured by the reviewer). The admitted losses
(crossings the ladder certified and the arm refuses) are filed as
`conic-quadric-first-harmonic-arm-refuses-crossings-the-ladder-places`.
An independent verifier (`analysis/reach-verify/4042`) found 0 wrong in
the reviewer's fuzz at three ε; the arm's single rounding charge covers
the coefficients' rounding with about 3× room (max 0.316 of the charge
over 21,200 exact-rational trials, none over it).

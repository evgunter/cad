---
id: a-spline-span-against-a-spline-face-has-no-interior-arm
kind: issue
title: A spline span against a spline face has no interior arm: lily wall 8 refuses CurvedBooleanUnsupported there with a NURBS tip seam of the blade
status: open
opened: 2026-10-27
priority: P1
cost: H
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane, a-unions-flush-reading-cannot-follow-a-nurbs-carrier]
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
refusals `sweep`'s `a_sheath_on_its_blade_unions_past_the_shared_rim`
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

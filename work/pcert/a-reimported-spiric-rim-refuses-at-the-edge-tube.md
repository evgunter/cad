---
id: a-reimported-spiric-rim-refuses-at-the-edge-tube
kind: issue
title: A STEP re-import of a spiric rim refuses adoption at the edge's rung-3 tube (TubeStraddles, margin 0.0): the geometry's verdict or the tube scale's?
status: open
opened: 2026-10-08
priority: P2
cost: M
---


Found by `pcert/projected-image`, which moved the rung-3 uniqueness tube
(C2's limb 3) into the edge certificate
(`geom_brep::certify`, `certify_via`, `NurbsLane::analytic_rung3` →
`edge_nurbs::analytic_rung3`, which states limbs 1–2 against each
operand before the tube). It runs for an `Intersection` description
over a `Curve3::Nurbs` carrier between two analytic surfaces whenever
the scalar holds the lane.

## Measured

`crates/step-import/tests/spiric_roundtrip.rs`,
`a_spiric_rim_exports_as_a_spline_and_its_reimport_door_is_pinned`. The
vessel cavity's spiric rims export as cubic splines (uncertainty
1e-4 m). On re-import, the rim's `Intersection` reading refuses:

```
Certification { error: AnalyticRung3(TubeStraddles { verdict: Zero(Classified {
  margin: MarginDiag(Value(0.0, None)), band: (1e-9, 1e-8) }), boxes: 510 }) }
```

Limbs 1–2 hold against both operands; the tube is what refuses.

`TangentIntersection` refuses `ResidualExceeded { TangentParallel }`.
So the edge does not adopt, and the import refuses with `Adoption`.
Before the move, the reading adopted on its samples alone and the
import refused at tier 3's quadrature lane on the torus wall. Both
outcomes are refusals; the test is re-baselined to the new one.

## Open

Which is it?

- **The geometry's verdict.** The spline is the export's sagitta
  stand-in, so it lies off the true section between its samples.
- **An artifact of the tube's scale.** `analytic_rung3` takes
  the tube's extent as `carrier_diameter(carrier)`: the widest rung is the
  control-net diameter, and the ladder bottomed out at 510 boxes.

A margin of exactly `0.0` is suspicious either way. A fix owes one of
two things: a tube that certifies the true section's spline when it lies
within the band of it, or the statement of why this carrier has no tube.

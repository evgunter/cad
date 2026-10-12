---
id: a-reimported-spiric-rim-refuses-at-the-edge-tube
kind: issue
title: A STEP re-import of a spiric rim refuses adoption at the edge's rung-3 tube (TubeStraddles, margin 0.0): the geometry's verdict or the tube scale's?
status: closed
closed: 2026-10-12
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

## Closed

Neither. The 0.0 margin is a refused enclosure, not a measured
straddle: `ssi/enclose.rs`'s `implicit_gradient_enclosure` has no torus
(or cone) arm, so every box's `graph_margin` is NaI at every rung,
`zero_free_lower_bound` reads that as 0, and the narrowest rung reports
`TubeStraddles`. Its words then claim a sliver.

- **Geometry.** The export spline (513 controls, 510 spans) lies
  2.9e-13 m off the torus and 5e-18 m off the plane (20 000 samples),
  and limb 2 certifies it. It is within the band.
- **Scale and budget.** On the spiric, the ladder never bottomed out.
  All 20 rungs ran (1.4e-2 m down to 2.7e-8 m, above the 8e-9 m
  floor), and `boxes: 510` is the chain's span count, not a budget.
  But every rung was refused there, so whether some rung's margin
  would clear on the torus is unmeasured.
- **Does such a chain certify where it can be enclosed?** Yes, shown
  on plane × cylinder only: an export-shaped 512-node interpolant of a
  section at the vessel's scale certifies, chain and ladder included
  (`analytic_rung3_certificate.rs`,
  `a_dense_interpolant_of_a_section_certifies_its_tube`).

The edge now states why it has no tube:
`AnalyticRung3Refusal::NoTubeEnclosure { operand }`, raised once limb 2
holds. The roster is `ssi::enclose::has_implicit_enclosure`, pinned
against the enclosures.

A certifying spline on the same spiric waits first on the torus arms of
both enclosures, which is CHART's
`torus-certificate-runs-root-free-through-its-quartic` (evidence added
there). After those, it still needs a margin that clears at some rung
and the one-arc proof, neither of which is measured.

---
id: non-circle-conic-edge-refuses-against-every-curved-face
kind: issue
title: An ellipse (or spiric, or NURBS) edge refuses against a curved face it does not even meet - no clearance or root lane for the carrier
status: review
opened: 2026-10-01
refs: [line-edge-crossing-a-sphere-face-has-no-root-lane, sphere-union-sphere-refuses-though-the-section-is-closed-form]
branch: reach/conic-edge-curved-face
pr: 3805
---

Found by the `reach-snowman` lane's sweep of the crossing layer's
carrier × surface table.

## Measured

`reduce::curved_face_arm` dispatches on the edge's carrier before any
clearance test: `Line` and `Circle` go on to the enclosures and the
certified roots, and every other carrier takes
`_ => return Err(frontier())` — the comment above it says "Ellipse/NURBS
carriers keep the unconditional M5 door". So an `Ellipse`, `Spiric` or
`Nurbs` edge whose box meets a cylinder, sphere or torus face refuses
`CurvedPierceUnsupported` whether or not it comes near the face.

A real shape reaches it: the exact tilted cut of
`crates/sweep/tests/m5_pr5_tilted_cut.rs` (a radius-0.5 drum, height 1,
split by the plane through `(0, 0, 0.5)` with normal
`(sin 0.3, 0, cos 0.3)`), its lower part, against a ball:

| ball | ∪, ∩, ∖ |
|---|---|
| r 0.2 at `(0.5, 0, 0.35)` (straddles the cut rim) | `CurvedPierceUnsupported { operand: A, face: FaceKey(1v1), edge: EdgeKey(12v1) }` |
| r 0.3 at `(0, 0, 0.5)` (≥ 0.2 clear of the rim, only pokes through the tilted cap) | the same |

`EdgeKey(12v1)` is the cut's `Ellipse` rim (major 0.5234, minor 0.5).
The second row is the sharp one: the ellipse never comes within 0.2 of
the sphere, so a clearance verdict alone would let it through.

## What a fix has to supply

- **A clearance rung** for these carriers: the circle rung's sampled
  arc enclosure (`geom_brep::circle_arc_residual_range`, chord-dip
  charged) is the shape, needing a curvature bound for the carrier.
- **A root lane** for the definite crossings: an ellipse against a
  sphere is a degree-2 trigonometric polynomial in the ellipse's
  parameter (the residual is quadratic in the point, the point first
  harmonic in the parameter), so `circle_torus::half_angle_roots`,
  which is surface-generic over degree-2 harmonics, takes it as it
  stands; against a cylinder the same holds. Spiric and NURBS carriers
  have no such form.

Spiric (`spiric_rim`) and NURBS edges take the same line of code; they
are not measured here against a real shape.

## Outcome (2026-10-02, branch `reach/conic-edge-curved-face`)

The conic rung is the ellipse's too. `geom_brep::Conic` (a circle is its
`major = minor` instance) carries the residual algebra: the sphere and
cylinder residuals along it are exact degree-2 trigonometric
polynomials (`ConicHarmonics`, one home for both kinds and both
carriers), the arc's sampled enclosure reads its curvature bound off
the harmonics, and the torus arm's bound is stated at the semi-major
axis (`|C′|, |C″| ≤ a`). `reduce::curved_face_arm` takes `Circle |
Ellipse` through one rung, and `boolean::ellipse_roots` answers the
ellipse × sphere and ellipse × cylinder cells on the shared root cores
(first-harmonic arm when the second harmonic is in the zero band — the
ellipse whose projection off the wall's axis is a circle, a constant
residual — the half-angle ladder otherwise).

Measured on the two rows above, after: the rim crosses or clears
exactly, and each op stops at the next door — the ball straddling the
rim, past the crossing layer and the sector side, at the join's
cylinder × sphere germ frame
(`work/join/cylinder-sphere-germ-pair-has-no-section-frame.md`), the
ball through the cut face at `SectionNotPolar`
(`work/reach/tilted-sphere-pair-section-refuses-at-the-polar-gate.md`)
or, charted about the cut normal, at the at-infinity probe
(`work/contact/at-infinity-probe-measures-in-closed-form-only.md`).
What builds: a ball or a rod held inside the drum within reach of the
rim's box (`crates/sweep/tests/conic_edge_curved_face.rs`), which also
needed the extent scan's cylinder arm to read the wall's carrier
(residue: `sphere-straddling-a-cylinder-carrier-refuses-at-the-extent-scan`).
Rods across the rim needed the wall placement to read a wall bounded by
a planar section; they now stop at the join too (a wide rod's parallel
walls: `work/join/parallel-cylinder-germ-pair-has-no-join-arm.md`; a
narrow rod's ring: `work/tang/pierce-ring-has-no-join-arm.md`). Residue filed: `ellipse-edge-crossing-a-torus-has-no-root-lane`,
`conic-quadric-doors-choose-their-first-harmonic-arm-two-ways`; the
spiric and NURBS half is CLEAVE's
`boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule` (the
operand gate refuses them first).

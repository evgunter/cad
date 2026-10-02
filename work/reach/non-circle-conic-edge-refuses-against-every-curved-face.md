---
id: non-circle-conic-edge-refuses-against-every-curved-face
kind: issue
title: An ellipse (or spiric, or NURBS) edge refuses against a curved face it does not even meet - no clearance or root lane for the carrier
status: open
opened: 2026-10-01
refs: [line-edge-crossing-a-sphere-face-has-no-root-lane, sphere-union-sphere-refuses-though-the-section-is-closed-form]
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

## Also met: an engraved section face (SHOW, 2026-10-02)

The `tiltedcut` scene (`demos/tour/src/curvedcut.rs`, wall 1 of
`walls`) engraves a C, an annular sector of radii 0.25 and 0.15, into
the elliptical section face of a cylinder (r 1, height 2.5) cut by the
plane through `(0, 0, 1.25)` with normal `(sin 0.3, 0, cos 0.3)`: a
blind pocket 0.05 deep whose outline lies strictly inside the ellipse
(its leftmost point 0.30 short of the rim along the major axis). `subtract(lower half, tool)` refuses
`CurvedPierceUnsupported { operand: A, .. }` with the edge the half's
`Ellipse` rim and the face one of the tool's cylinder walls, an
ellipse × cylinder pair whose carriers never meet. The U and a disc
refuse the same way, on either half's section face, at every pose the
review tried (offsets (0, 0), (0.3, 0.2), (−0.2, −0.3); depths 0.02,
0.05, 0.2). Walls 1 (C, lower half) and 2 (U, upper half) assert the
rim is an `Ellipse` and panic when the subtraction builds or refuses
otherwise. Lines-only glyphs get past this door and are
pose-dependent: `work/contact/at-infinity-probe-measures-in-closed-form-only.md`.

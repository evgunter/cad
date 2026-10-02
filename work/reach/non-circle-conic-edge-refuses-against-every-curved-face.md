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
rim's box (`crates/sweep/tests/conic_edge_curved_face.rs`); the
extent scan hands their sphere × wall pairs to the section pass
(PR 3801's route, merged in).
Rods across the rim needed the wall placement to read a wall bounded by
a planar section; they now stop at the join too (a wide rod's parallel
walls: `work/join/parallel-cylinder-germ-pair-has-no-join-arm.md`; a
narrow rod's ring: `work/tang/pierce-ring-has-no-join-arm.md`). Residue filed: `ellipse-edge-crossing-a-torus-has-no-root-lane`,
`conic-quadric-doors-choose-their-first-harmonic-arm-two-ways`; the
spiric and NURBS half is CLEAVE's
`boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule` (the
operand gate refuses them first).

### Fix pass (dual review of PR 3805)

The review found the ellipse ladder's roots certified off the wall (its
`τ` metric is not arc length along an eccentric ellipse), tangencies
certified as misses in the shared half-angle ladder, a frame premise
the mint does not keep, and an at-end decision metered at the least
speed. The degree-2 root doors now answer by certified subdivision in
the residual's metres (`circle_roots::certified_subdivision`, TANG's
core); `geom_brep::Conic` reads semi-axis magnitudes in any stored
order and sign; a root's gap from an end is metered at the carrier's
speed there.
Residue filed: `work/hone/half-angle-ladder-escalates-in-its-own-metric.md`.

### Second fix pass

The harmonics' rounding is charged at the semi-axes' magnitudes (a
negative `major` under-charged it), and so is `replace_face`'s
`pose_reach`; the subdivision charges each derivative of `F` its
Bernstein share of the noise, and every guard it has carries a row red
without it. PR 3801 landed first; its section-pass route replaced this
branch's carrier arm for sphere × cylinder pairs
(`reconcile-sphere-cylinder-scan-arm-with-3801`), and the straddling
pose that arm refused now builds.

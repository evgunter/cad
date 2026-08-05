---
name: step-curved-subset
description: The STEP writer's curved subset (M5 PR 13) — every elementary surface and conic exports as an EXACT native AP214 entity, never the rational-quadratic form; OCC is blind to face sense on curved geometry too, so orientation acceptance is text-level
metadata:
  type: reference
---

Since M5 PR 13 `crates/step-export` covers the whole kernel geometry
vocabulary, and every arm is a **native, exact** AP214 entity:

- surfaces: `PLANE`, `CYLINDRICAL_`, `CONICAL_`, `SPHERICAL_`,
  `TOROIDAL_SURFACE`
- carriers: `LINE`, `CIRCLE`, `ELLIPSE`,
  `B_SPLINE_CURVE_WITH_KNOTS` (rational complex instance when weighted)

**Conics do NOT take the rational-quadratic road** (NURBS Book §7.3–7.4,
shape factor `k = w₀w₂/w₁²`), even though CURVED-DESIGN names that as
the kernel's export form for conics and it is exact. AP214 HAS `CIRCLE`
and `ELLIPSE`, so the rational form would be an equally exact and
strictly worse encoding — it discards the axes/centre readers consume
and reparameterizes for nothing. No kernel curve kind needs it. If a
future schema target lacks conics, that is when §7.4's
infinite-control-point machinery becomes relevant.

The mapping is an **identity**, not an equivalence: each kernel frame
`(origin, axis, u_ref)` is `axis2_placement_3d` field for field, so the
parameterizations agree, not just the point sets. The acceptance suite
compares emitted reals to the body's stored reals with `==` (the float
printer round-trips to identical bits). The single exception is
`Cone`: STEP's `v` is axial, the kernel's is slant arc length, so they
differ by a fixed cos α — invisible while no pcurves are emitted, and
the thing to carry when pcurves land. Cones use the **apex placement**
(`radius = 0.0`), legal under 10303-42's `radius >= 0` WHERE rule and
the encoding that invents no offset constant.

**One live refusal** (of PR 13's original two: the NURBS-face
refusal RETIRED at M6-3/#192 — the loft-assembly unit minted NURBS
faces at rest and the export side now emits
`B_SPLINE_SURFACE_WITH_KNOTS` on both arms):
`CurvedShellClassification` — the outward/void classifier for
MULTI-shell solids never grew curved closed forms, so it is now
narrower than the emitter. Its divergence-theorem reduction is a
planarity identity, and its output is a material-vs-void sign, so an
approximation there would be a silent lie. Only S12's two-stub
`boss ∖ plate` complement hits it.

**OCC is blind to face sense on curved geometry too.** Re-measured at
PR 13: `revert(ball)` and `revert(washer)` — inside-out solids, every
face `same_sense = .F.` — import as `valid: True` with the SAME
positive volumes as the un-reverted bodies. ShapeHealing rectifies
silently, exactly as M4's review found on `cube.step`. So curved
orientation acceptance is text-level: `orientation_oracle.rs` gained
**edge-use coherence** (every edge traversed once each way per shell —
curved-agnostic, needs no planarity), whose negative control is the
double-composition bug itself. FreeCAD volumes ARE trustworthy for
magnitude: the committed fixtures (ten at the PR 13 measurement; the
corpus is now 15 solids + `nurbs_wireframe`, oracle-checked in CI)
match their closed-form analytic volume to ≤ 4e-15 relative, because
surfaces cross the wire as surfaces.

**Trap.** Suites matching emitted records to kernel entities must walk
the WRITER's traversal (`tests/common/mod.rs::walk_order`), never
`Body::faces()`/`edges()`: arena order coincides on simple extrusions
and diverges on boolean results, so the wrong pairs get compared on
precisely the most interesting bodies.

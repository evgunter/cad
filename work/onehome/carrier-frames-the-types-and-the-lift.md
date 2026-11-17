---
id: carrier-frames-the-types-and-the-lift
kind: unit
title: Carrier frames (i): the variants hold UnitVec3 / RigidMotion behind accessors, and the lift re-mints a frame at the target scalar
status: dispatched
opened: 2026-11-17
priority: P1
cost: H
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
branch: onehome/carrier-frames
---


The prerequisite of the parent row's split (orchestrator, 2026-11-17).

- **The types.** `geom::Frame` is backed by a `RigidMotion`. The
  axisymmetric variants (cylinder, cone, sphere, torus, spindle, circle,
  ellipse, spiric) and the plane hold their `axis`/`u_ref` frame as
  that witness, and a line holds its `dir` as a `UnitVec3`.
- **Accessors.** The variants give `.axis()`, `.u_ref()` and
  `.frame()`, and readers are migrated onto them, so a later field
  change does not touch every reader again.
- **Tests.** Tests build carriers through a small f64 fixture helper.
- **The lift.** `Curve3::map_scalar` and `Surface::map_scalar`
  re-mint a frame field at the target scalar by a decision-free
  renormalisation, exact in the reals: the axis renormalised and kept,
  the reference's residual perpendicular to it renormalised. Every
  other field is mapped structurally. The f64 entries are the rounding
  of the exact rotation the frame's mint names, so the lift reads the
  carrier the f64 body stands for. It follows `[scalars]` ("built at
  the lane scalar, from its own inputs and those choices") and
  `RigidMotion`'s "No door carries a frame up from f64". It is rule A,
  provisional until the measurement on this branch.
- **`moved_by`.** It changes only where the type change forces it.
  The sibling row is
  `the-boolean-reads-each-operand-in-its-own-coordinates`.

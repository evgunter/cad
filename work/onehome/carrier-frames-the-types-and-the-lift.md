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
  that witness. A line's `dir` stays a `Vec3` here: it is also an
  affine utility whose parameter is in units of `|dir|`, so it is its
  own row, `carrier-frames-a-lines-dir-is-a-unitvec3` (orchestrator,
  2026-11-17).
- **Accessors.** The variants give `.axis()`, `.u_ref()` and
  `.frame()`, and readers are migrated onto them, so a later field
  change does not touch every reader again.
- **Tests.** Tests build carriers through a small f64 fixture helper.
- **The lift.** `Curve3::map_scalar` and `Surface::map_scalar` carry
  a frame's entries within one `Real::WITNESS` kind, and re-mint it
  across kinds (`RigidMotion::map_scalar`). At a plain enclosure
  (`Interval`) the re-mint encloses the polar factor of the stated
  entries to second order (`Real::enclose_rotation`, the bound argued
  in its `Interval` doc); a frame whose `η` is not below ½ or whose
  determinant is not certified positive poisons. At a target that also
  carries derivatives or forms (`Dual<Interval>`, `Sym<Interval>`) the
  re-mint squares (`RigidMotion::squared`), since an enclosure of the
  value does not reach those channels; a computation runs at one
  scalar, so the two never meet. Every other field is mapped
  structurally. It follows `[scalars]` ("built at the lane scalar,
  from its own inputs and those choices") and `RigidMotion`'s "No door
  carries a frame up from f64" (orchestrator rulings A and polar (a),
  2026-11-17/18).
- **Witness-preserving mints where the rows need them** (orchestrator
  ruling A): sweep's walls, caps, revolve frames, swept stations and
  placed rims, topo's motion of a surface frame and the split's section
  plane are minted from the frame they are built on, not squared. The
  rest of the production mints are row (ii).
- **`moved_by`.** It changes only where the type change forces it.
  The sibling row is
  `the-boolean-reads-each-operand-in-its-own-coordinates`.

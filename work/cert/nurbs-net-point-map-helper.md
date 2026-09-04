---
id: nurbs-net-point-map-helper
kind: issue
title: map_points on NurbsSurface/NurbsCurve landed from fix/ — the control-point map behind transform_rigid
status: review
opened: 2026-09-04
branch: fix/transform-rigid-nurbs
pr: 1742
---


**A fence-crossing record, not a request.** `crates/geom/src/*` is S-CERT's
territory glob. FIX's `keep_out` line says the NURBS point-map helper is
filed here as a row if S-CERT is live when the unit is cut; S-CERT is open,
so this is that row.

## What landed

Two methods, both modelled on the `map_scalar` that sits beside each:

- `NurbsSurface::map_points` (`crates/geom/src/surfaces/nurbs.rs`)
- `map_points` in the `nurbs_curve!` macro, so `NurbsCurve2` and
  `NurbsCurve3` both carry it (`crates/geom/src/curves/nurbs.rs`)

Each carries every control point through the caller's closure and hands the
knot vectors and the weight channel over verbatim, constructing through the
existing private `from_validated_parts` (a pointwise map cannot change a
count, so no re-validation runs — the argument is already written there and
the `debug_assert` re-derives the count agreement).

## Why the weights are untouched, stated at the door

The nets are stored **Euclidean** — `Point3`/`Point2` control points with
the weights in a separate `f64` channel — so evaluation forms the
normalized ratio `sum(N w P) / sum(N w)`, an AFFINE combination of the
control points. An affine map commutes with an affine combination, so
mapping the net alone reproduces the image exactly. Both doc comments say
this, and both say the converse: were the net stored WEIGHTED
(homogeneous `w*P`), the translation limb would have to be scaled by `w`
and the same call would bend the geometry.

Neither method decides anything, meters anything, or touches a certificate;
no k-lint row is added. The caller owes affineness and nothing checks it,
which is why the obligation is written at the door rather than assumed.

## Consumer

`topo::transform_rigid`, which now refuses only the NURBS **placeholder**
and maps described nets — `work/fix/transform-rigid-refuses-described-nurbs.md`.

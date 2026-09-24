---
id: public-ring-names-spell-the-retired-type
kind: issue
title: CurveRingData, SurfaceRingData, ring_coords and apply_ring name the retired ring type's role in the public API
status: open
opened: 2026-09-24
---


## Finding

Split out of `certification-value-hygiene-has-no-gate` (item 4) when
RING-5 closed that row's other items. RING-3 kept "ring" as the algebra
adjective ("ring quotient"); these public names instead name the
retired certification type's ROLE, and now spell a type that no longer
exists:

- `geom_core::spline::compose::CurveRingData` (`spline/compose.rs`, near `:127`)
- `geom_core::spline::compose::tensor::SurfaceRingData`
  (`spline/compose/tensor.rs`, near `:143`)
- `NurbsCurve2::ring_coords` / `NurbsCurve3::ring_coords`
  (`geom/src/curves/nurbs.rs`, near `:1638`, `:1647`) and
  `NurbsSurface::ring_coords` (`geom/src/surfaces/nurbs.rs`, near `:1373`)
- `CurvePlan::apply_ring` (`spline/algebra.rs`, near `:262`)

Each hands out or consumes certification `Interval`s (the
`Certification` trait's values, `geom_core::interval::certification`).

## Fix

A rename to say what they are now — certification coefficients
(`CurveCertData`/`certified_coords`/`apply_certified`, or whatever reads
best) — across `geom`, `geom-brep`, `mesh`, `step-import` and `topo`.
A public-API change with no behaviour in it; nothing in RING-5's surface
needed it.

---
id: two-reversal-policies-for-a-spline-net
kind: issue
title: NurbsSurface::reversed_u/v keep the domain and refuse an asymmetric vector, while geom_brep::reversed_column negates the domain and never refuses: two reversal policies
status: open
opened: 2026-10-10
priority: P3
cost: E
---

(NURBS lane, from the delta review of PR 4479. Not a defect in either
door; a split in policy.)

## Finding

Two doors run a spline net backwards, under different rules:

- `NurbsSurface::reversed_u` / `reversed_v` (`crates/geom/src/surfaces/nurbs.rs`)
  keep the chart's domain and are DEFINED only on a mirror-symmetric
  knot vector (`KnotVector::mirror_symmetric`), refusing every other
  one. Their docs say why reflecting the knots about the domain is not
  offered: `lo + hi − k` is not exact in `f64`.
- `geom_brep::reversed_column` (`crates/geom-brep/src/nurbs_iso.rs`,
  since PR 4479) reflects through 0 (`KnotVector::negated`, exact for
  every vector) and never refuses; its result lives on `[−b, −a]`.

Both are exact, so neither is unsound. But a caller choosing between
"run this backwards" doors meets two answers: one that keeps the
parameterization's domain and refuses most vectors (by the predicate's
own docs, equally spaced loft sections refuse from 7 on), and one that
always succeeds on a negated domain.

## Shape

Decide whether the surface doors should offer the negated reflection
too (a reparameterization `v ↦ −v` onto `[−hi, −lo]`, exact for every
vector), or say in their docs why a chart keeps its domain where an
edge carrier need not. Not changed in PR 4479.


---
id: an-on-surface-edge-is-not-tied-to-its-curves-surface
kind: issue
title: OnSurface::edge takes a bare carrier and span, so nothing ties an edge to the SurfaceCurve's surface it is read against
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-09
pr: 6624
---


Filed from `co-surface-carriers-have-no-home-of-their-own-in-curve-pair` on closing it (PR 6567). It was found by PR 6566's FULL review.

## What

Nothing in the types ties an `OnSurface` to its `SurfaceCurve`'s surface.
- `OnSurface::edge` (`crates/geom-brep/src/curve_pair/on_surface.rs`) takes a bare carrier and span. Its documented precondition, held by the caller, is that the edge lies on the curve's surface.
- A `SurfaceCurve` is tied to a `geom::Surface` value, not to a face's record. `apply_cut_ins` charts the face's sphere on its own `u_ref`, and a region door rebuilds its surface from its fields.

## What closing it takes

Build an `OnSurface` from a face (its record and edge), and give a `SurfaceCurve` that record's key, so that `across` refuses a mismatch. Check first whether every caller has a face record to hand. A caller that synthesises its surface has none, and either keeps the precondition with its reason or carries the key it charted from.

## Closed (2026-11-09, PR 6624)

- A `SurfaceCurve` is charted from a `Charted` (a record's key and the value charted on it) and holds the key. `OnSurface::edge` takes its face record's key, and `across` refuses a pair on two records (`AcrossRefusal::OffRecord`).
- Every caller has a record to hand:
  - The region doors hold their face's record and chart their re-built surface under it. `RegionEdge` carries its face's key.
  - `apply_cut_ins` charts the meridian under the sphere face's key.
- That the charted value is the record's surface stays each re-charting caller's premise, documented at `Charted`.
- The loop walk's `u_line` (PR 6609, open at closing) takes its key when the two meet.

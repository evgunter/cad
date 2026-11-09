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

- A `SurfaceCurve` is charted on a `Charted`, which owns the record's key and surface. The surface is one of:
  - as stored (`Charted::record`);
  - turned about its own axis onto another seam (`Charted::turned`);
  - a sphere on a frame of the caller's (`Charted::framed`, the sphere door's, which keeps the chart's poles where its rows were measured).
- `OnSurface::edge` takes its face record's key. Both readings of a pair, `across` (the edge across the curve's plane) and `under` (the curve across the edge's own plane), refuse a pair on two records (`AcrossRefusal::OffRecord`).
- Every caller reads the key and the surface off one face:
  - Each region door holds its face's `Charted`, and its geometry is read off it. The sphere door no longer takes a centre and radius from its callers. `landings` keys each edge by the door's record.
  - `apply_cut_ins` turns the face's own sphere onto the cut's meridian.
- A mismatch in either caller is a fault in its wiring (one record per door; every cut-in piece inherits the face's record), so it panics naming both keys.
- The premise left to a caller is that the key and the surface it hands `Charted::record` are one record's.
- The loop walk's `u_line` (PR 6609, open at closing) is keyed when the two meet. Its ray plane is synthesised, not read off the face's record, and its premise is named at `conic_crossings`.

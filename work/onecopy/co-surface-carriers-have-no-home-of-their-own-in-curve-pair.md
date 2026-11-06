---
id: co-surface-carriers-have-no-home-of-their-own-in-curve-pair
kind: issue
title: Two carriers on one surface have no co-surface reading in curve_pair; a lane can hand them to meet, whose 3D gap is identically zero ([ev]: C5 sentence + OnSurface type)
status: open
opened: 2026-11-06
priority: P2
cost: M
refs: [region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet]
---


From the off-surface landings fork (2026-11-06). The two designers agreed on this. The draw is on `analysis/design-fork/onecopy-landings-off-surface`.

## What

Two curves known to lie on one surface record have a 3D gap that is identically zero. So `curve_pair::meet`'s contact stage decides nothing well-posed for them: what it reads is the construction's rounding against the band.

- At Interval it escalates once the coordinate scale nears 1e3 m at ε 1e-12. The enclosure gap is about 3.4e-15 × scale, measured on a probe.
- Nothing in the types stops a caller from handing such a pair to `meet`. PR 6491's first route did exactly that.

The region doors now read a plane-section crossing instead (the row it refs).

## What closing it takes

1. **A type for a curve on a surface.** `OnSurface<'s>` or `Section` holds a span plus the surface record it lies on. It never converts implicitly to a bare `Span`.
2. **A co-surface arm in `geom_brep::curve_pair`.** It shares `transverse`'s harmonic generator and reads the surface's component in place of `contact`. Then the region doors and the sphere cut-in (`apply_cut_ins`) read it by type.
3. **The rule in the design doc.** This extends C5's table, so it is an `[ev]` PR. Until then, add a clarifying sentence to `[one-way.readings]` or C5 (also `[ev]`): two carriers on one surface meet as a crossing of a plane section, or in the chart, never as a 3D curve-pair gap.

It earns its place with a second consumer. The sphere cut-in is one.

## Implemented: items 1 and 2 (2026-11-06, `onecopy/meet-pose`)

1. **The type.** `geom_brep::curve_pair::on_surface` (`crates/geom-brep/src/curve_pair/on_surface.rs`):
   - `SurfaceCurve`: a curve on a surface, built only from a `geom::Surface` and chart coordinates. Its families are a parallel (cylinder, cone, torus), a meridian (torus, sphere), a ruling (cylinder, cone), a cone's generator from its apex, and a sphere's great circle and circle about a pole. Its centre, radius and direction are the surface's own at those coordinates, so a curve off its surface cannot be built as one: the reviewer's 6491 note on `Round::through` and `Line::from`. A family the surface does not have is refused (`NoSuchFamily`).
   - `Section`: which curve of its plane's section of the surface it is (moved from `topo`'s `LegSection`).
   - `OnSurface`: an edge span on the same surface. It has no conversion to `Span`.
2. **The arm.** `OnSurface::across(&SurfaceCurve, row, band)` reads the edge across the curve's plane through the plane door (`span_plane_meet`, with its `span_place`), and `SurfaceCurve::section_of` reads a root on the curve's section through the chart's channels, where `meet` has `contact`. `SurfaceCurve::across_plane` reads the curve across an edge's plane, for the torus's spiric and parallel legs. The plane door's sinusoid is now `curve_pair::pose::height`, the generator `transverse` reads, bit for bit.
   - `region::landings` and the three doors' `Round` and `Line` legs read through it. `Round::through`, `Line::from` and `LegSection` are gone.
   - `apply_cut_ins` reads the cut's half-meridian as a sphere meridian through it. `FirstHarmonic::conic_off_plane` and the `bool_sphere_cut_roots_*` rows are retired (`docs/K-REPORT.md`, ONECOPY addendum on the cut-in).

Item 3, the C5 rule, is the `[ev]` PR 6567. This row closes there.

**Type gap left (review of 6566).** Nothing in the types ties an `OnSurface` to its `SurfaceCurve`'s surface: `OnSurface::edge` takes a bare carrier and span, and that the edge lies on the curve's surface is its documented precondition, held by the caller. A `SurfaceCurve` is tied to a `geom::Surface` value, not to a face's record: `apply_cut_ins` charts the face's sphere on its own `u_ref`, and a region door rebuilds its surface from its fields. Closing it would mean an `OnSurface` built from a face (its record and edge) and a `SurfaceCurve` carrying that record's key, so `across` could refuse a mismatch.

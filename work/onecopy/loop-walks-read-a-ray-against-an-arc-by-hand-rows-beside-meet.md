---
id: loop-walks-read-a-ray-against-an-arc-by-hand-rows-beside-meet
kind: issue
title: The planar loop walk and the sphere cut-in read a ray or a half-meridian against a boundary arc by hand rows beside curve_pair::meet
status: open
opened: 2026-11-03
priority: P1
cost: M
refs: [region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet, circle-plane-first-harmonic-has-three-hand-built-copies]
---

Found by the sweep of `onecopy/meet-pose`, which moved the four region `landings` (`topo/src/boolean/{region,ruled_region,sphere_region,torus_region}.rs`) onto `geom_brep::curve_pair::meet`. Two sites outside those doors still answer "where does this curve meet that boundary edge" with rows of their own:

- `splitting/containment.rs` `conic_crossings` (called from the planar loop walk, `carrier_walk` ~2064): the ray `q + d·t` against a conic arc, by the conic's unit-circle chord (`point_in_arc_loop_conic_disc`) and its own span rows. A ray and an arc are a `LineConic` pair of `meet`.
- `boolean/ops.rs` `apply_cut_ins` (~7232): a sphere face's boundary arcs against the cut's half-meridian, by `circle_roots::first_harmonic_roots` over `FirstHarmonic::circle_off_plane` against the meridian plane, then its own `bool_sphere_cut_span` rows. Two circles on one sphere are a `CircleCircle` pair of `meet`, and `meet` now reads the conic that crosses the other's plane the less obliquely, so the plane premise buys it nothing (the landings decision, `region-landings-...` "Decided").

Both can decide one meeting differently from `meet` in the band (DESIGN `[one-way.readings]`, C5).

Fix: read each against `meet` (the region skeleton's `region::landings` shows the shape: a `Span` per course, `Landing` from `Meeting`, the shared run as a run along), after checking what each caller needs that a `Meeting` does not carry (the loop walk counts crossings and reads a tangency as a graze; the cut-in needs each arc's root parameter for a split).


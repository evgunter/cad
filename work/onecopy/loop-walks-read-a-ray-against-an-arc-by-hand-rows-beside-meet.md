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

Found by the sweep of `onecopy/meet-pose`, which moved the four region `landings` (`topo/src/boolean/{region,ruled_region,sphere_region,torus_region}.rs`) onto one reading, `region::landings`: one curve across the other's plane through the plane door, then the root on its plane's section curve. Two sites outside those doors still answer "where does this curve meet that boundary edge" with rows of their own:

- `splitting/containment.rs` `conic_crossings` (called from the planar loop walk, `carrier_walk` ~2064): the ray `q + d·t` against a conic arc, by the conic's unit-circle chord (`point_in_arc_loop_conic_disc`) and its own span rows. A ray and an arc are a `LineConic` pair of `meet`.
- `boolean/ops.rs` `apply_cut_ins` (~7232): a sphere face's boundary arcs against the cut's half-meridian, by `circle_roots::first_harmonic_roots` over `FirstHarmonic::conic_off_plane` against the meridian plane, then its own `bool_sphere_cut_span` rows. Two circles on one sphere are co-surface, so they meet as a crossing of a plane section and never as a 3D gap (the landings row's 2026-11-06 Decided section). This is the plane door's reading the region landings now take, and the second consumer `work/onecopy/co-surface-carriers-have-no-home-of-their-own-in-curve-pair.md` waits for.

Both can decide one meeting differently from `meet` in the band (DESIGN `[one-way.readings]`, C5).

Fix:
- **The loop walk's ray against a conic arc** is not co-surface (a ray in the face's plane against an arc of it is: check which before choosing). A transverse pair reads `meet`; a co-surface one reads the plane door, as `region::landings` does.
- **The cut-in's arc against the half-meridian** reads the plane door (`span_plane_meet`, with its roots at an end reported), or the co-surface arm once it exists.

Before either move, check what each caller needs: the loop walk counts crossings and reads a tangency as a graze, and the cut-in needs each arc's root parameter for a split.


## 2026-11-06: the cut-in half is done

`apply_cut_ins` reads its arcs against the half-meridian through the co-surface arm, `geom_brep::curve_pair::on_surface` (`OnSurface::across`, the plane door with each root's place on its arc reported, then `SurfaceCurve::section_of`). Its root parameters still feed the split. `FirstHarmonic::conic_off_plane` and the `bool_sphere_cut_roots_*` rows are gone. What is left is the planar loop walk's ray against a conic arc (`splitting/containment.rs` `conic_crossings`).

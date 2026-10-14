---
id: point-to-carrier-distance-is-spelled-seven-times
kind: issue
title: A point's distance to a carrier is spelled seven times over different kind subsets, and the on-carrier decision reads it exact in one door and linearized in another
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

A point's distance to a carrier (and the on-carrier decision built on it) is spelled in:

- `geom_brep::implicit_residual` (`crates/geom-brep/src/implicit.rs:165`): exhaustive, but Nurbs and Approx return poison rather than refusing typed;
- `carrier_eq::distance_to` (`crates/topo/src/boolean/carrier_eq.rs:1027`): plane, sphere, cylinder and torus;
- `carrier_touch`'s own distance;
- `pcurve_cache::chart_distance` (`crates/geom-brep/src/pcurve_cache.rs:8207`): `unreachable!` for Nurbs and Approx;
- `blend/reach::surface_distance` (`crates/sweep/src/blend/reach.rs:490`);
- `offset_axial::surface_residual` (`crates/topo/src/offset_axial.rs:2556`): `_ => 0`, filed separately on SHELL;
- `solid_contain::torus_elevation` (`crates/topo/src/boolean/solid_contain.rs:2673`).

The on-carrier decision is not one reading either:
- `contain` decides on the exact ρ−r;
- `solid_contain` decides on the linearized (ρ²−r²)/2r;
- `vtxfac` and `reduce` decide on `implicit_residual`, and the placement then decides again on the exact form, under the same predicate name `bool_curved_contain_carrier`.

## Owed

One point-to-carrier residual: one arm per surface kind, a typed refusal for the kinds with no closed form, and one decision form per predicate name. The test oracles keep their own copies by design.

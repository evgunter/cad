---
id: point-to-carrier-distance-is-spelled-seven-times
kind: issue
title: A point's distance to a carrier is spelled seven times over different kind subsets, and the on-carrier decision reads it exact in one door and linearized in another
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

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

Re-aimed to D3 "One reading per question". A point's residual, distance and on-carrier decision are a reading of one carrier against a point. One such reading, with one arm per surface kind, which every spelling above reads.

- `implicit_residual`'s Nurbs and Approx poison is a total evaluator's poison, which D3 says is not a decision: no reading reads it without first refusing on kind. The reading refuses those kinds typed, naming what it lacks.
- One decision form per predicate name: the exact ρ−r and the linearized (ρ²−r²)/2r are two readings of one fact, which D3 forbids. `bool_curved_contain_carrier` decides once.
- D3 places one-carrier-against-a-point readings in `geom`, the lowest crate that can state them.

The test oracles keep their own copies by design (D3 exempts them).

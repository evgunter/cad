---
id: curve-surface-crossing-has-no-kind-pair-table
kind: issue
title: Curve-surface crossing has no kind-pair table: wildcard arms, fallback ladders, and circle-plane solved in six places
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

"Where does this curve cross this surface" has no `(CurveKind, SurfaceKind)` table. Today:

- **Planar faces:** `reduce::sweep_direction` (`crates/topo/src/boolean/reduce.rs:1046`).
  - Line uses interpolation and conic uses `splitting::conic_plane_candidates` (`crates/topo/src/splitting/classify.rs:563`).
  - Spiric and spline are refused `CrossingCarrierUnsupported`.
  - Dispatch is an Option gate, not a match.
- **Walls:** `reduce::curved_face_arm` (`:1918`) → `conic_clearance` (`:2737`) → `wall_crossing` (`:3320`) → per-pair root finders (`line_wall_roots_of`, `line_cone_roots`, `conic_quadric_roots` at `crates/topo/src/boolean/conic_quadric/mod.rs:197`, `circle_torus_roots`, `ellipse_torus_roots`).
  - Wildcards: `wall_crossing` `_ => Unsettled` (twice), `line_wall_roots_of` `_ => Uncertain`, `conic_quadric_roots` `_ => {}`.
  - Runtime fallbacks: enclosures first, then roots, then `carrier_touch::off_face`; the dip-bound clearance, then roots; `implicit::conic_residual_extremes` harmonic, else sampled.
- **Circle×plane is solved in about six places:**
  - `carrier_cross::circle_meets_plane` (`crates/topo/src/boolean/carrier_cross.rs:507`);
  - `conic_plane_candidates`;
  - `ops::apply_cut_ins` (`crates/topo/src/boolean/ops.rs:6053`);
  - `sphere_region::ray_roots` (`crates/topo/src/boolean/sphere_region.rs:414`);
  - `ring_path::plane_roots` (`crates/topo/src/ring_path.rs:1520`);
  - `pcurve_cache` incidence.
- **Line×cone** is solved twice, with different rows (`line_cone_roots`, and `solid_contain::cast_ray`'s own quadratic at `crates/topo/src/boolean/solid_contain.rs:5239`).
- **Census confirm:** `census::point::edge_face` (`crates/topo/src/census/point.rs:253`) serves only (circle, plane) and (line, sphere), and refuses the rest typed.

## Owed

One total `(CurveKind, SurfaceKind)` crossing table:
- closed forms where they exist;
- the certified walk for spiric and spline;
- each consumer reads it.

`carrier_cross`'s curve×plane parts and the census edge-face cut (`Uncrossable`) are in `fuse/the-curve-pair-reader`'s callers and are left to it. This row is the rest.

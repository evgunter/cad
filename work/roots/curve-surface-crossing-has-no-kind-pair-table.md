---
id: curve-surface-crossing-has-no-kind-pair-table
kind: issue
title: Curve-surface crossing has no kind-pair table: wildcard arms, fallback ladders, and circle-plane solved in six places
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

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
  - `carrier_cross::circle_meets_plane`: retired (stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227));
  - `conic_plane_candidates`;
  - `ops::apply_cut_ins` (`crates/topo/src/boolean/ops.rs:6053`);
  - `sphere_region::ray_roots` (`crates/topo/src/boolean/sphere_region.rs:414`);
  - `ring_path::plane_roots` (`crates/topo/src/ring_path.rs:1520`);
  - `pcurve_cache` incidence.
- **Line×cone** is solved twice, with different rows (`line_cone_roots`, and `solid_contain::cast_ray`'s own quadratic at `crates/topo/src/boolean/solid_contain.rs:5239`).
- **Census confirm:** `census::point::edge_face` (`crates/topo/src/census/point.rs:253`) serves only (circle, plane) and (line, sphere), and refuses the rest typed.

## Owed

Re-aimed to D3 "One reading per question". A curve's crossings of a surface are a pair *outcome*. One crossings reading, one exhaustive match over `(CurveKind, SurfaceKind)`, which each consumer reads. Where the crossing depends on the pair's pose (a curve lying in the surface, a tangent touch), it takes the pose reading's answer and decides no pose of its own.

- Closed forms where they exist. The certified walk for spiric and spline is those arms' one method, fixed at write time, not a method reached because a closed form declined.
- The wildcards (`wall_crossing` `_ => Unsettled`, `line_wall_roots_of` `_ => Uncertain`) become listed arms with typed refusals naming what they lack. `conic_quadric_roots`' `_ => {}` answers a default value (no roots), which D3 forbids.
- The ladders: a stage before the method is allowed only if it is total in kind, decides the same question soundly at the same band, and cannot change the verdict by which stage decided (a box cull before exact roots). Enclosures may meet that bar. `carrier_touch::off_face` after roots, the dip-bound clearance then roots, and harmonic-else-sampled are fallbacks.
- A consumer that serves fewer pairs (`census::point::edge_face`) matches on the reading's answer and refuses the rest typed.

`carrier_cross`'s curve×plane parts and the census edge-face cut (`Uncrossable`) are in `fuse/the-curve-pair-reader`'s callers and are left to it. This row is the rest.

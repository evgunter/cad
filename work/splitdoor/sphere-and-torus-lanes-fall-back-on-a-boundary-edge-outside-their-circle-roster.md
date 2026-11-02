---
id: sphere-and-torus-lanes-fall-back-on-a-boundary-edge-outside-their-circle-roster
kind: issue
title: Sphere and torus face lanes read only circles (and spirics) on a face's boundary and return Ok(None) or admit nothing past them: audit each fallback for a silent narrowing
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found on review of PR 5940 (`splitdoor/tilted-plug-rosters`). Its
sweep's first pass matched only refusing fallthroughs; these sites
fall through to `Ok(None)` ("not in this class") or admit a fixed
kind set, so a face with an edge past the roster takes the caller's
fallback silently, or never reaches the lane.

## The sites

- `crates/topo/src/boolean/solid_contain.rs` `sphere_chart_trim`: a
  boundary edge that is not a circle (`let geom::Curve3::Circle … else
  { return Ok(None) }`) takes the face out of the sphere-chart class;
  the caller's typed refusal stands.
- `crates/topo/src/boolean/surface_group.rs` `unmated_boundary`: an
  unmated edge that is not a circle reads `Ok(None)`. Whether that
  reaches the closed-group reading's own `Ok(None)` ("definitely NOT
  closed", per its doc) is not read here.
- `crates/topo/src/boolean/torus_region.rs`: `torus_face_region` admits
  boundary kinds `[Circle, Spiric]` only, and `circle_roots` reads a
  circle edge's crossings with a meridian leg's plane.
- `crates/topo/src/boolean/sphere_region.rs`: `sphere_face_region`
  admits circles only (`unreachable!("sphere_face_region admits circles
  only")` at each read past it).
- `crates/topo/src/boolean/ops.rs` `sphere_pair_cut` / `pair_seam`:
  read the sphere pair's radical-plane section as a circle and refuse
  any other `SphereSphereSection`. Two spheres meet in a circle or not
  at all, so this one is likely sound; listed for the audit's
  completeness.

## Direction

For each, decide whether the `None` is honest ("this lane does not
read this face, and the caller's fallback is right for it") or a
narrowing that turns a buildable input into a refusal or a wrong
reading (whether `unmated_boundary`'s `None` becomes "definitely not
closed" is the one to check first). A sphere or torus face trimmed by a cylinder or cone holds a
quartic edge, which is how each is reached. Sibling:
`a-sphere-cut-in-refuses-a-sphere-face-with-a-boundary-edge-that-is-not-a-circle`.

---
id: the-face-covers-sphere-arm-reads-the-booleans-chart-trim-and-face-placement
kind: issue
title: topo::cover's sphere arm reads boolean's sphere_chart_trim and curved_face_placement: two body readings the [topo.homes] gate wants at the root
status: open
opened: 2026-11-08
priority: P2
cost: M
---


Found by `cover-topo-faces`, which moved topo's face and edge readings
to the crate root (`crates/topo/src/cover.rs`) under `[topo.homes]`
(`crates/topo/README.md`; gate `scripts/gates/engine-leaves.sh`).

Two arms of `topo::cover::face_cover` still read readings the boolean
engine hosts, so `cover.rs` carries a cited `names-boolean` pin
(`scripts/gates/engine-leaves.counts`):

- the sphere arm's chart window (`cover.rs` `sphere_window`) reads
  `boolean::solid_contain::sphere_chart_trim` and its
  `PointInSolidError`;
- the pole-side cut along an aim (`cover.rs` `pole_side`) reads
  `boolean::contain::curved_face_placement`, `CurvedPlacement` and
  `FaceContainment`.

Both are readings over a `Body` that name no engine type in their
answer (a latitude/azimuth window, a point's place on a face), so by
the signature test they belong at the crate root, not in `boolean/`.

**Fix**: move `sphere_chart_trim` (with the refusal it answers in, as a
root type) and the curved face placement to root modules, repoint the
boolean's own readers, and drop `cover.rs`'s `names-boolean` line.

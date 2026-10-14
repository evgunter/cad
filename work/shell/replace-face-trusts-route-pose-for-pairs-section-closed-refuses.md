---
id: replace-face-trusts-route-pose-for-pairs-section-closed-refuses
kind: issue
title: replace_face reads route_pose as serving sphere-sphere and cylinder-sphere, then section_closed refuses both
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

`replace_face` reads `route` and `route_pose` first (`crates/topo/src/replace_face.rs`, ~l.2924-3013), then picks its own lane. The general lane is `offset_derive::section_closed` (`crates/topo/src/offset_derive.rs:518`).

`route_pose` returns "every pose served" for (sphere, sphere|cylinder) and (cylinder, sphere) without a pose check (`crates/geom-brep/src/intersect.rs:494`). But `section_closed` serves only plane×{plane, cylinder, cone, sphere, torus} and cone×cylinder, and is `_ => Unsupported` for the rest.

So sphere×sphere and cylinder×sphere pass the table as implemented and are then refused by a second table. Plane×plane is answered by `section_closed`'s own `plane_plane`, not the C5 arm.

## Owed

`section_closed` reads `route`'s arm for every pair `route` implements. Where the offset lane is a deliberate narrowing, `route_pose` (or the lane) says so before the door claims the pair is served.

---
id: replace-face-trusts-route-pose-for-pairs-section-closed-refuses
kind: issue
title: replace_face reads route_pose as serving sphere-sphere and cylinder-sphere, then section_closed refuses both
status: open
opened: 2026-10-14
priority: P1
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

`replace_face` reads `route` and `route_pose` first (`crates/topo/src/replace_face.rs`, ~l.2924-3013), then picks its own lane. The general lane is `offset_derive::section_closed` (`crates/topo/src/offset_derive.rs:518`).

`route_pose` returns "every pose served" for (sphere, sphere|cylinder) and (cylinder, sphere) without a pose check (`crates/geom-brep/src/intersect.rs:494`). But `section_closed` serves only plane×{plane, cylinder, cone, sphere, torus} and cone×cylinder, and is `_ => Unsupported` for the rest.

So sphere×sphere and cylinder×sphere pass the table as implemented and are then refused by a second table. Plane×plane is answered by `section_closed`'s own `plane_plane`, not the C5 arm.

## Owed

Re-aimed to D3 "One reading per question": which pairs a reading serves is derived from the reading, never kept beside it. `route_pose` claiming sphere×sphere and cylinder×sphere while `section_closed` refuses both is the two sources of truth that rule removes.

- `section_closed` reads the one section outcome reading, which takes the pose reading's answer, for every pair that reading serves. That includes plane×plane, which today is `section_closed`'s own `plane_plane` rather than the C5 arm.
- Where the offset lane is a deliberate narrowing, it matches on the reading's answer and refuses the rest typed. Then the door's claim and the lane's refusal come from one reading, and the door asks `route` (the reading's kind-level dispatch) rather than a list of its own.
- `route_pose`'s "every pose served" for (sphere, sphere|cylinder) and (cylinder, sphere) is no pose classification. The pose reading classifies these pairs too, so the served claim is what the reading decides rather than a blanket.

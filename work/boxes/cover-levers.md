---
id: cover-levers
kind: unit
title: The levers read the cover: the split and join reaches, the carrier doors, the tangent witness span, the containment selection reach and side reader, and geom_brep::Reach built from cover queries
status: parked
opened: 2026-11-02
priority: P1
cost: H
blocked_on: [cover-topo-faces, cover-edge-doors]
---


Fourth unit of the cover build (`cover-geom-readings` has the order and the ratified text).

Each lever that bounds a region reads the cover's `distance_range`, `support` or `ball` and keeps no kind match:
- `splitting::rules::{face_reach_from, face_reach_round_from, face_axial_range, cone_face_reach}`;
- the `join.rs` frame extents;
- `boolean::carrier_pair::face_ball` (`a-sphere-or-torus-pair-extent-skips-the-face-box-rule`);
- `boolean/mod.rs` `witness_span`/`conic_reach`;
- `solid_contain::selection_reach`, `containment::loop_extent_from`, `side_reader::beyond`;
- `chart_region::face_boundary_points` where it is read as an extent.

**Seam with `crates/geom-core/README.md` "Float rounding".** That section was ratified on PR 6329 the same day as PR 6334. It names `geom_brep::Reach` as the one type for the consumed extent a verdict levers over, built only from the geometry consumed, with `ExtentBall` its ball payload. This unit builds `Reach` from cover queries of the consumed geometry: `Reach::Span`/`Ball` become a cover, and `Measured` stays only if something still measures. If the two texts turn out to disagree in practice, that is a revision to propose to Ev, not something to settle locally.

Verdicts move here: whole-carrier levers tighten. Each reader that moves gets its own differential.

Also closes, by reference (they are filed on other programs):
- APEX `face-reach-misses-an-interior-far-point-on-a-sphere-torus-or-cone-face`;
- TANG's whole-turn conic reach row;
- LEVER `germ-takes-the-span-bounded-face-reach-alone`;
- HONE `splitting-face-extent-lever-reads-vertices-not-arcs`;
- TWICE `extent-ball-and-reach-are-two-statements-of-one-extent`.

Coordinate with their owners in the PR.

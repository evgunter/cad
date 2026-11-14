---
id: circle-torus-plane-height-refuses-a-graze-the-pose-places-as-a-touch
kind: issue
title: circle_torus_plane_height refuses Tangency where a plane grazes the tube, a sibling of the pose's extreme-in-the-gap read, unmeasured
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6642
---


Noted, not measured, by the needle-tilts lane in PR 6623's sweep.

In `crates/geom-brep/src/curve_face/circle_torus.rs`, `circle_torus_plane_height` refuses `Tangency` when a plane grazes the torus tube. PR 6623 handles the same case in `curve_pair::pose`: a height extreme in the gap, with the other extreme definite past it, places a touch, and the window and then the polish decide it. The tube graze may be the same question: a screen-and-locator read refusing where a touch should be placed and certified.

**To close it:**
- Measure whether a pose that grazes the tube refuses where the touch is really there. A row showing the refusal is enough.
- If it does, decide whether the grazing read should go through the same home `pose.rs` now has (`height_extremes`/`Extremes`), or explain why the question differs here.

**Related programs:** germ, honeroot and splice also work on circle × torus. Check their slates before starting.

## Resolved (PR 6642)

Measured. At a graze (depth in the zero band or the gap), circles 0.7 m and more from the tube's top ring refused `Tangency` or escalated, where the answer is a miss.

The depth is now a screen. At a graze, the door's own extreme placements decide: both extremes outside the tube on one side of the centre circle is a miss, since the residual grows away from the ring the plane touches. Otherwise the graze's refusal stands. Pinned by `circle_torus::tests::a_plane_grazing_the_tube_is_a_miss_off_its_ring`.

The question is the pose's, but the home differs: a curve against a surface has no window, and the door's residual at the extremes is what reads the distance. Germ, honeroot and splice were checked; none covers it.

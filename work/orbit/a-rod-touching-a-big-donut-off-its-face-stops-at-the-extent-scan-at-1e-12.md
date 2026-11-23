---
id: a-rod-touching-a-big-donut-off-its-face-stops-at-the-extent-scan-at-1e-12
kind: issue
title: A rod touching the donut's outer equator past its end refuses FallbackExtentUnsupported at ×100–×250 at ε 1e-12
status: open
opened: 2026-11-10
priority: P2
cost: M
refs: [a-torus-near-a-tilted-cut-stops-at-the-extent-scan]
---


Found by ONEHOME's session-box fixtures lane (`onehome/session-box-fixtures`, main at `75243665`).

`crates/sweep/tests/torus_touch_off_faces.rs`'s `the_off_face_touches_build_at_every_scale` scaled the donut and its partners ×1e3. That reaches 3.5 km, past the session box, so the lane moved the large scale to ×250 (875 m). At ε = 1e-12 the donut ×250 is now a finished body. The 1e3 donut used to refuse at its finish by escalation, so before this the row never reached a Boolean at that row. The brick and the ball build every op, in both orders. But the rod (`top_rod`: its wall touches the outer equator past its end, the pose `a_rod_whose_wall_touches_the_outer_equator_past_its_end_builds` builds at unit scale) refuses every op in both orders with `FallbackExtentUnsupported { operand: A, face: … }`: "a curved face and a face of the other solid have tangent or near-tangent carriers and no crossing layer saw an event". Measured:
- ×10 builds at 1e-12;
- ×100, ×150, ×200 and ×250 refuse at 1e-12;
- ×250 builds at 1e-9 and 1e-6.

The rod lies 1.5 units clear of the donut except at the touch, and the touch is off the rod's face. So this looks like the extent scan's tangency reading at a scale where ε is a few dozen ulps of the coordinates, not a near-tangency the band cannot decide. That is the regime `[tolerance.units]`'s admission floor says is admissible.

The row pins the refusal at that ε and scale (`torus_touch_off_faces::ROD_SCAN_STOPS`, keyed to the measured band), so it goes red when this is fixed. Then remove the pin.

Signed: (ONEHOME session-box-fixtures lane)

**Since PR 7131 (GREEN U12, 2026-11-23).** `torus_touch_off_faces::the_off_face_touches_build_at_every_scale` now stops before the scan this row pins: operand B is not a finished body (`props_cylinder_circle_off_axis`, hi 1.568e-12, nightly 44989234943). Cause: the polar read. 7131's mutant A alone turns the row green, and F1X or F1F alone does not. The refusal is counted in ONEHOME's `pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted` (PR 7402's branch until it merges), with probe runs 45083192761 (green at `e4d679d37e`), 45083200534 (red at `9bc9d6517b`) and 45083184695 (mutants). GREEN pins nothing until Ev rules on that count.

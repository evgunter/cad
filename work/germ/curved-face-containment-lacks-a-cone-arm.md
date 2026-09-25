---
id: curved-face-containment-lacks-a-cone-arm
kind: issue
title: curved_face_containment has no cone arm (the torus half rides germ/torus-doors)
status: open
opened: 2026-09-25
priority: P0
cost: D
refs: [curved-face-containment-lacks-cone-torus, VERBS-CONE]
---


The cone half of `curved-face-containment-lacks-cone-torus`, split off when
that row's torus half was dispatched with `germ/torus-doors` (2026-09-25).
`contain::curved_face_containment` answers `Ok(None)` for a cone face while
`point_in_solid` answers cones. The torus arm that lands first is the
template. Best taken with `VERBS-CONE`'s operand lanes.

---
id: curved-face-containment-lacks-a-cone-arm
kind: issue
title: curved_face_containment has no cone arm (the torus half rides germ/torus-doors)
status: closed
opened: 2026-09-25
priority: P0
cost: D
refs: [curved-face-containment-lacks-cone-torus, VERBS-CONE]
branch: germ/cone-containment-and-pose-gate
closed: 2026-09-28
pr: 3322
---


The cone half of `curved-face-containment-lacks-cone-torus`, split off when
that row's torus half was dispatched with `germ/torus-doors` (2026-09-25).
`contain::curved_face_containment` answers `Ok(None)` for a cone face while
`point_in_solid` answers cones. The torus arm that lands first is the
template. Best taken with `VERBS-CONE`'s operand lanes.

## Closed (PR 3322, 2026-09-28)

Landed with the cone lane (single full review, APPROVE-WITH-FIXES; fix pass taken whole). See the PR body.

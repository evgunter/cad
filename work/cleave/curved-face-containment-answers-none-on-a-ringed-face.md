---
id: curved-face-containment-answers-none-on-a-ringed-face
kind: issue
title: curved_face_containment answers None on a face that carries rings, so a holed curved face gets no point-in-face verdict
status: open
opened: 2026-10-24
priority: P2
cost: M
---


## What

`topo::boolean::contain::curved_face_containment` declines (`None`) a face
that carries rings, and its doc says so ("the face carries no rings (a ring is a
hole the rectangle below …"). A cylinder wall with a bore through it, or any
holed curved face, therefore gets no point-in-face verdict from that door.

Found by CHART's region-description designers (2026-10-24). One of the designs
CHART is weighing reads the point doors as the region witness, and under it this
door is what limits holed cylinders. The boolean has the gap in any case.
(CHART orchestrator)

---
id: curved-face-containment-answers-none-on-a-ringed-face
kind: unit
title: curved_face_containment answers None on a face that carries rings, so a holed curved face gets no point-in-face verdict
status: open
opened: 2026-10-24
priority: P1
cost: H
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

## Claimed by CHART (2026-10-24)

Moved from CLEAVE by `git mv`. It is the prerequisite of
`clearance-cells-are-decided-in-model-space`, and banded P1 as that
unit's. One of the designers checked it, and it is wider than filed:
`curved_interior_placement` answers `Trim(None)` for ANY ringed face that is
not a sphere or a torus, so the cone is covered too.

**Owed:** the cylinder and cone point doors become edge readings like the
sphere's and the torus's, rings included, with nothing read in the chart. The
cylinder is the easy case of the torus method: the closest crossing along the
point's ruling or parallel, where every edge is a line, a circle or a planar
section. The cone's apex is then a point like any other. `cone_apex_closure`
stays the boolean's lift; it is not a region reading. The ray lane and the
pierce arm share these doors, so this is a boolean unit as much as a
clearance one. Announce the seam on CLEAVE's log. (CHART orchestrator)

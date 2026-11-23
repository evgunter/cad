---
id: point-touch-reading
kind: issue
title: The census reads a point touch: a circle on a plane, a line or plane on a sphere, two spheres, found on the carriers' closed-form lane and certified there
opened: 2026-10-13
status: closed
priority: P1
branch: contacthold/point-touch-reading
pr: 4956
refs: [a-carried-edge-face-record-the-census-cannot-read-refuses-every-later-op, an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane]
closed: 2026-10-13
---


The census's point-touch reading, C3's point-touch bullet (`[ev]` PR
4951 carries the text; its mint-time confirm and lineage carry wait
for Ev and are not this unit).

## What

`census::point` (`crates/topo/src/census/point.rs`) finds the touch
point on the closed-form lane of the two cells' carriers and confirms
the record there when `κ_rel` is definitely positive (sagitta door,
levered by the curved carrier's radius), the carriers coincide within
ε, and the point is strictly inside both cells (the curved edge-interior
door, the face containment door). Apart or crossing reads stale; a
circle lying in a plane away from a conventional vertex is a curve
touch, refused typed; an edge–face pair outside the lanes refuses
typed, and a face pair outside them goes on to the tangent-line lane.

- edge–face: circle on a plane, line on a sphere
  (`census::confirm_edge_face`, after the line-in-plane and
  conventional-vertex lanes);
- face–face: sphere on a plane, sphere on a sphere outside or inside
  (`census::confirm_face_pair`, ahead of the tangent-line lane);
- edge–edge: two circles kissing, or a line on a circle, are already
  read by `census::curved` (the radical line's chord, discriminant in
  band); a test now pins it.

## Measured

Rod on plate (`sweep` `face_face_touch`): `plate ∪ rod`'s 3′ reads both
cap records (only the cross-solid curved search stays undecided). ∪ a
far brick and ∖ a hole in the plate, carrying `carried()`, answer with
all three records (`editor-core` `face_face_touch_carry`). The bridge
carrying every record answers one solid with the face–face and bottom
cap records, 3′ Ok. A step glued under the bottom cap divides its circle;
the record lands on the piece holding the touch point only.

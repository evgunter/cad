---
id: a-pierce-point-inside-a-curved-face-of-the-piercing-operand-refuses-unbuilt
kind: issue
title: A pierce point inside a curved face of the piercing operand, or a plane face holding one over a curved pierced face, refuses unbuilt
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [an-edge-lying-in-a-face-at-a-pierce-refuses-ring-meets-ring]
---

## What

`vtxfac::faces_holding_pierce` (`crates/topo/src/boolean/vtxfac.rs`)
gives a ring vertex to a face of the piercing operand that holds a
pierce point inside it, so the face's section with the pierced face
passes a vertex there. It builds that vertex on two planes only, and
refuses `BooleanError::HeldPierceUnbuilt` before anything is built in
three cases:

- the point lies inside a curved face of the piercing operand
  (`placed: true`);
- a curved face's box holds the point but its trim cannot say whether
  the face does (`placed: false`): a ringed wall, a NURBS face, an
  outline the chart door does not read;
- the face holding the point is a plane but the pierced face is curved.

Before that function, all three built a body whose section passed the
pierce with no vertex. Only the result's tier-3 gate caught it, if
anything did.

## The shape to give

All three want what the plane case has. The face takes a ring vertex at
the point. Its one Out run reads the face's half above the pierced face,
with germs along the two faces' section through the point. On a curved
face, that section is a conic or a ruling, not a line. The germs are its
tangents (`pierce_germ_dir` reads the tangent plane already), and the
run is levered for the face's curvature as `pierce_side` levers it.

No row reaches either case yet. A witness: a cylinder standing on a
plate, with a prism whose edge lies along one of the cylinder's
rulings, folded so the cylinder and the prism are one operand when the
plate joins them.


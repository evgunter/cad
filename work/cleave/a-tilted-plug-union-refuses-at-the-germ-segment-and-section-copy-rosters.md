---
id: a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters
kind: issue
title: The tilted plug's union refuses at sectors::one_segment and chord_join's section copy, which read lines and circles only
status: open
opened: 2026-10-12
priority: P2
cost: M
---


Found by the two designers on `a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims` (2026-10-12), each measured on a prototype that reads ellipses in `carrier_cross::boundary_crossing`.

With the interior question reading ellipses, the tilted plug's ∩ and both ∖ build at their closed forms. ∪ still refuses, at two lines-and-circles rosters further along:

- `sectors::one_segment` refuses `GermEdgeCarrierUnsupported { operand: A, edge }`. It reads two edges leaving a vertex pair along a germ, with paired far ends, as one segment only for lines and circles. As a property of curve kind that is right: two ellipses tangent at a point with a shared far end can differ between. Here, though, the identity is structural and already decided. Each rim is the meet of a top face and a wall face, and the glue door decided both face pairs to be one carrier (walls `SameOpposite`, tops `SameOriented`). A suggested direction is to read the glue's one-carrier verdicts on the two edges' parent face pairs before the kind rule, i.e. D10's "same construction" identity. Not measured.
- Once that is relaxed, `chord_join`'s copy of a section segment ("only lines and circles are copied") refuses an ellipse arc. Measured by one designer, up to here and no further.

The issue's closed form (the union is the slab, 12) stays unreached until both land.

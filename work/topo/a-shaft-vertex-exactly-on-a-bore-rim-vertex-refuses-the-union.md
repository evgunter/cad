---
id: a-shaft-vertex-exactly-on-a-bore-rim-vertex-refuses-the-union
kind: issue
title: A shaft vertex exactly on a full-turn bore's rim vertex refuses the union
status: open
opened: 2026-10-23
---


`crates/editor-core/tests/a_mate_on_a_full_turn_bore_names.rs` puts a
shaft (a circle of the bore's radius, extruded along its own `z`) in a
collar revolved a full turn about its own `y`, the collar turned a
quarter turn about `x`, then `−a` about `z`. At azimuth `a = 0` the
shaft's circle vertex lies on the collar's seam, so on the "through"
span it meets a rim vertex.

Written as the exact quarter turn (`turn/4`, D′2's placement of the
move its author meant), every coordinate is exact and
`topo::union(collar, shaft)` refuses:

```
azimuth 0, through, collar ∪ shaft: the mate does not union:
Some(NodeUnbuilt { a_face: FaceKey(3v1), b_face: FaceKey(3v1), why: OnBoundary(A) })
```

Written in radians (`ang(FRAC_PI_2)`, cosine 6.1e-17 — what the
retired `place_against` helper wrote), the same row unions and names
cleanly at every azimuth and span. The row is pinned to the radian
form for now, with a comment pointing here
(`a_mate_on_a_full_turn_bore_names.rs:114`).

The exact coincidence is the case the row's own header describes (a
shaft vertex meeting the collar at a rim's vertex), so the boolean
should build it rather than refuse it. Done when the row writes
`turn/4` and passes.

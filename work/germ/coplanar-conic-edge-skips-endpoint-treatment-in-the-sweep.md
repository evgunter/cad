---
id: coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep
kind: issue
title: A conic edge lying in a plane face's plane gets no endpoint treatment in the sweep, so premise S holds for it only through its neighbours
status: open
opened: 2026-09-28
priority: P1
cost: E
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
---

## What

`reduce.rs` `sweep_direction`, the conic × plane lane: when
`conic_plane_crossing_roots` returns `Ok(None)`, the lane `continue`s
with no endpoint treatment. `Ok(None)` has two causes, and one of them
is not "never meets": the PARALLEL-frame gate (`classify.rs`, the
`split_conic_plane_parallel` `Zero` arm) returns it for a conic whose
plane is parallel to the face's plane, which includes a conic lying IN
it. The code comment ("a conic that definitely never meets the plane …
Zero endpoints are impossible here") is false for that case.

A conic edge lying in `G`'s plane and running inside `G` is then neither
recorded, nor certified a miss, nor refused, at this arm — premise S of
the section certificate fails per arm. What evidences such a contact
today is the neighbours:

- a line edge of `G` crossing the conic is a pierce of `F`'s carrier
  landing on `F`'s boundary (`OnEdge`, recorded), or a sphere frontier
  refusal;
- the conic's endpoints are endpoints of other edges (a meridian seam,
  a line edge), whose endpoint rows record them;
- a planar face beside the conic and coplanar with `G` is refused
  `UndeclaredCoincidence`.

Measured (section-certificate fix pass, 2026-09-28): a cylinder with
its cap coplanar with a box's top, the box's boundary crossing the rim,
refuses `UndeclaredCoincidence`; a donut revolved with its seam
parallels at the equators, a box top in the equator plane holding an
arc of the outer equator, refuses `CurvedSectorSideUnsupported` under
∪ (and at the ∖/∩ roster). No wrong answer found. A vertex whose every
incident edge is a conic in `G`'s plane (a valence-2 vertex between two
coplanar arcs) would have none of those neighbours; no constructor in
the tree is known to build one.

## Fix

In the `Ok(None)` arm, give the endpoints the line lane's
`(Zero, Zero)` posture: each endpoint on `G`'s plane through
`vertex_on_face`. Then correct the comment, and the section
certificate's premise-S audit line for the lane.

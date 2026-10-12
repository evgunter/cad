---
id: a-ring-on-a-sphere-face-has-no-island-winding
kind: issue
title: A section passing through a sphere face as a ring refuses RingOffCylinderChart: the ring lane winds its island on a cylinder wall's chart only
status: closed
opened: 2026-10-03
priority: P2
cost: M
closed: 2026-10-07
branch: tang/sphere-ring-island-winding
---


Found by REACH's `reach/arc-from-pairing` lane, which gave a pierce
ring's chord its arc on every carrier (the chord takes the arc the
pierce germs' directions name, `chord_join::Leave`).

## Measured

`crates/sweep/tests/m5_s13_review_probes.rs`,
`probe_edge_escape_refuses_typed_before_the_scan`: the slab against
`ball_poled_y(0.5)` at `(0.3, 2.0, 1.2)` under ∪. The slab's top edge
pierces the ball's face, the pierce lands as a ring of that face, and
the join refuses

```
Join(RingOffCylinderChart { face: FaceKey(2v1), kind: Sphere })
```

raised in `chord_join::chart_island_winding` (its `wall_chart` reads a
cylinder only, `no_wall_chart`), which `boolean::join::choose_roles`
asks for the role order of a same-face ring lane on a wall
(`RingClosure::Wall`). `chord_join::chart_ring_side`, the ring
re-homing's chart reading, has the same cylinder-only `wall_chart`.

A sphere ring elsewhere builds: the tilted sphere pairs whose pierce
lands inside a half-band (`crates/sweep/tests/tilted_sphere_pair.rs`,
`a_pierce_off_the_seam_plane_builds_under_every_boolean`) never ask
for an island winding, since their ring closes on the other loop.

## What a fix owes

The island winding of a ring lane on a sphere face's chart, or a
chart-free statement of it (the side of the section plane the island
lies on, read from the germs), with this probe's pose under every op
against its closed form.

## Evidence (2026-10-05, `reach/trimmed-sphere-escape`)

A chart-free sphere arm was written and measured on that branch and
taken back out of it (the unit found a cut that makes no ring); the
code is at `cd76b9c3a3`, `boolean::join::sphere_island_ccw`. The
statement, for a ring lane of a sphere face whose chord lies in the
section plane `Π`:

- the run lies in one closed cap of `Π` (side `σ`, read off every
  charted run edge's midpoint), so run ∪ arc bounds one region inside
  that cap and one holding the other; the region LEFT of the closing
  arc is the inner one exactly when `N × t` at the arc's midpoint
  points to `σ`;
- the ring is a hole, so the two candidate runs bound the two pieces
  the arc cuts off the face's side of the ring, and the outer loop lies
  in one: an outer-loop vertex `w` off `Π` on the side opposite `σ`
  says the run's patch is the island exactly when it is the inner
  region; `w` on `σ` is read against the other run, which must lie on
  the opposite side.

"The side of the section plane the island lies on" alone does not
decide it: both candidates' left regions lie inside their own caps,
and only the outer loop tells them apart. On this item's probe pose it
built ∩ in both orders and slab ∖ ball, tier 3 clean; ∪ and ball ∖ slab
then refuse `ResultInvalid { VolumeUncomputable { source:
RingOnCurvedFace } }`, the ringed sphere face the result carries
(`sphere-face-with-a-hole-has-no-closed-form`).

## Evidence (2026-10-05, PR 4046's dual review)

PR 4046 lets a carved sphere body (one with a face bounded by a tilted
circle) be a later boolean's operand. Wherever a new operand's edge then
crosses a sphere face, the join stops here. Measured on
`reach/carved-sphere-classify`, every case under ∪:

- the lens union `ball(1, (2, 2, 0.5)) ∪ ball(1, (3.4, 2, 0.5))` against
  `ball(0.5, (2.7, 2, 1.3))` refuses
  `Join(RingOffCylinderChart { face: FaceKey(4v1), kind: Sphere })`;
- `ball(1, origin)` against the box `[0.3, 2] × [0.2, 2] × [0.1, 2]`
  (a corner inside) and against `[−0.2, 2] × [−2, 2] × [0.4, 2]` (an edge
  through) both refuse `Join(RingOffCylinderChart { face: FaceKey(1v1), kind: Sphere })`;
- `ball(1, origin) ∪ ball(0.6, 1.2·(0.6, 0, 0.8))` refuses the same.

The reviewers report the box cases under every op
(`analysis/reach-dual/4046-r1`, NOTE 4; `-r2`, NOTE 9).

## Closed (2026-10-07, `tang/sphere-ring-island-winding`)

A sphere face's ring lane winds its island without a chart,
`chord_join::sphere_island_winding`, closed by the chord's arc (so
`boolean::join::choose_roles` waits on the segment's curve for a
sphere, `RoleLane::SphereRing`, as the planar lane does).

- The run lies in one closed cap of the section plane: the side its
  arcs' ends, midpoints and in-span extremes decide
  (`chord_join::arc_probes`), or, for a run on the section circle, the
  side the arc leans to.
- Its left region is the inner one when the arc's left normal points
  into that cap.
- An outer-loop reference (a vertex, or an edge midpoint) decides which
  region is the island: one on the far side directly, one on the run's
  side by the crossing parity of the great-circle path to the far cap's
  pole (`chord_join::sphere_path_parity`).

Ring re-homing on a sphere, `chord_join::sphere_ring_side`, reads the
same parity from a ring vertex to a reference of the old face's outer
loop: a vertex, or the midpoint of an edge the run does not share. A
path with a zero-band reading says nothing and the next pair is asked,
so a ring on the run at every vertex refuses there as on a wall's chart
(`a-chorded-ring-on-the-run-at-every-vertex-has-no-homing-reading`).

`RingOffCylinderChart` is renamed `RingIslandUnread`. A sphere reaches
it where a ring run reaches both sides of its section plane or is
bounded by an edge that is not a circle.

The probe pose builds ∩ in both orders and slab ∖ ball at the slice
integral; ∪ in both orders and ball ∖ slab keep the ring as a hole of
the ball's face and refuse at the result gate
(`work/flux/sphere-face-with-a-hole-has-no-closed-form.md`). The rows
are `crates/sweep/tests/a_ring_on_a_sphere_face.rs`, the review's
far-pole, bar and edge-midpoint poses among them. Filed: the cone kind,
`a-ring-on-a-cone-or-torus-face-has-no-island-winding`.

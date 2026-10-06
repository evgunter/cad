---
id: a-ring-on-a-sphere-face-has-no-island-winding
kind: issue
title: A section passing through a sphere face as a ring refuses RingOffCylinderChart: the ring lane winds its island on a cylinder wall's chart only
status: open
opened: 2026-10-03
priority: P2
cost: M
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

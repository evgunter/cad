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

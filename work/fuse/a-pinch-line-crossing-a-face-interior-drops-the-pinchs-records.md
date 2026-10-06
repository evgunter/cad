---
id: a-pinch-line-crossing-a-face-interior-drops-the-pinchs-records
kind: issue
title: A pinch line crossing a face's interior drops the pinch's records at the new pinch end
status: open
opened: 2026-10-03
priority: P1
cost: M
---


## What

A pinch (`union_with(q1, q3)`, `q1 = brick((0,1),(0,1),(0.5,1.5))`,
`q3 = brick((-1,0),(-1,0),(0.5,1.5))`, records carried) whose line
crosses the interior of a face of the other operand: the boolean
splits the pinch's two coincident edges where they pierce the face,
and a result that keeps both pieces there holds two vertices at that
point with no v-v record. 3′ refuses `UndeclaredContact`
`VertexVertex` there, plus the `EdgeEdgeOverlap` beside it. The
volumes are right.

Two witnesses, both in
`a_pinch_line_through_a_face_drops_its_records_at_the_new_end`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`), which pins the
failure as it stands:

- **The sheared wedge**: the 80°–190° wedge over z ∈ (0.5, 1) mapped
  by (x, y, z) → (x + 0.3(z − 0.5), y − 0.3(z − 0.5), z). Its corner sits
  at the pinch's lower end and the pinch line leaves through its top
  face at (0,0,1). `pinch ∖ C`, `pinch ∩ C` and `C ∩ pinch` fail 3′
  at (0,0,1); both unions and `C ∖ pinch` pass. Reached only since
  `fuse/shared-vertex-crossings`, whose corner arm builds it (on main
  the corner refused `SharedVertexCrossings`). Found by that PR's
  review.
- **A brick**: `brick((-0.5,0.5),(-0.5,0.5),(0,1))`, which holds the
  axis, so its top face crosses the pinch line at (0,0,1) with no
  shared vertex. `block ∩ pinch` builds at 0.25 and fails 3′ at
  (0,0,1). The same failure is on main. The pinch-first ops refuse
  `Join(RingHomingAmbiguous)`, which is
  `work/tang/a-pierce-strut-at-a-pinch-has-no-vertex-off-the-run.md`'s.

Not `a-subtract-through-a-pinch-line-drops-the-pinch-row-at-its-cut`:
there the new end was a v-v group whose kept corner was a null-edge
copy, and the group remap reaching copies fixed it. Here the new end
is a pierce: the pinch edges meet a face, so the reduction records
vertex-on-face rows (`contacts.a_on_b`/`b_on_a`), not a v-v pair, and
`remap_contacts` (`crates/topo/src/boolean/ops.rs`) records no v-v
row from them.

## Owed

Record the two pinch pieces' vertices where their split edges pierce
one face, the pierce's split vertices being coincident by
construction, as a v-v row of the result. Then flip the pinned test to
3′ passing in all six ops.

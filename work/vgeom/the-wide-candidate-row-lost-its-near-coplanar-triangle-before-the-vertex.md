---
id: the-wide-candidate-row-lost-its-near-coplanar-triangle-before-the-vertex
kind: issue
title: index_memo's wide-but-informative row has no near-coplanar candidate before the ring's aimed vertex once the ring's caps chart from their sketch frame
status: open
opened: 2026-11-14
priority: P3
cost: E
---

Found by BOXFINISH's `boxfinish/sweep-frames`. On that branch a sweep's
cap plane is its station's frame: its chart origin is the sketch
origin carried, where before it was a point fitted to the cap's corners.
The geometry is unchanged, but a planar face is triangulated in its
chart, so the bumped ring's mesh moved.

`crates/viewer/tests/index_memo.rs`
`a_wide_but_informative_candidate_answers_before_the_rings_aimed_vertex`
needs a near-coplanar triangle on the `reach = 1.48` ray that answers
`RING_WIDE_CANDIDATE_T = 1.4487652724897624`, 0.031 short of the aimed
vertex. On the new mesh the ray admits seven candidates:

- two near-coplanar ones answering `t = 1.48` (conditioning 1.47e-16
  and 1.28e-16, `|det|` 3.3e-20; the first has span
  `[1.46508, 1.49492]`);
- four transversal ones at `t = 1.48` (conditioning 0.112, `|det|`
  2.8e-5);
- one near-coplanar one answering `t = 1.5112347275102376`, 0.031
  BEYOND the vertex (conditioning 6.85e-16, span `[1.50672, 1.51575]`).

None is short of the vertex, so the class the row carries (a
near-coplanar candidate whose interval wholly precedes the transversal
one) has no fixture on this ray. The row's own note says a move here is
a change in that class, not a baseline to restore, so the branch pins
the row at its premise's stop at every ε row rather than re-aiming it.

What closes it: a fixture whose ray meets a near-coplanar triangle
before the aimed vertex, chosen for the class and not for a mesh
accident (a fixture that states the triangle would not move with a
chart's origin), and the pin lifted.


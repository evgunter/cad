---
id: the-wide-candidate-row-lost-its-near-coplanar-triangle-before-the-vertex
kind: issue
title: index_memo's two wide-candidate rows lose their near-coplanar triangle once a partial revolve's far rim circles take the turned station frame's u_ref
status: open
opened: 2026-11-14
priority: P3
cost: E
---

Found by BOXFINISH's `boxfinish/sweep-frames`. On that branch a
partial revolve's far rim circles take their `u_ref` from the arc's
sketch start radial carried by the turned station frame (the arc arm
of `swept::placed_segment_spec_about`), where before they took the
normalised vector from the circle's placed centre to its placed start
vertex. The two agree to one to three
ulps. The geometry is unchanged, but the rim's chord points are
sampled from the carrier, so the mesh near that rim moved.

**The cause, by value** (every stored surface, carrier and point of
the bodies dumped on the merge base and on the branch and diffed):

- The bumped gallery ring: the far cap's four rim circles' `u_ref`
  moved from `(0.9807852804032304, 0, 0.19509032201612872)`,
  `(-0.9807852804032304, 0, -0.19509032201612872)`,
  `(0.9807852804032303, 0, 0.19509032201612878)` and
  `(-0.9807852804032304, 0, -0.19509032201612875)` to
  `±(0.9807852804032303, 0, 0.19509032201612872)`.
- `tube_arc`: the far cap's two rim circles' `u_ref` moved from
  `±(0.5403023058681397, 0.8414709848078965, 0)` to
  `±(0.5403023058681398, 0.8414709848078965, 0)`.
- On both, the caps' stored planes moved too (origin to the station
  origin, and on `tube_arc` the far cap's normal by an ulp), but the
  planar mesher builds its chart from the boundary's own chord points
  and reads no stored plane (`crates/mesh/src/planar.rs:42-60`). Every
  other surface, carrier and point is bit-identical.

So the only triangulator input that changed is the rim chord points,
moved by the carriers' rounding. The ring row's aimed vertex is on its
far cap's inner rim.

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
one) has no fixture on this ray.

Its sibling
`a_wide_candidates_interval_reaching_the_aimed_vertex_refuses_with_both`
(`tube_arc`, the certified-tie side of the same line) stops the same
way: no admitted candidate answers `TUBE_ARC_WIDE_CANDIDATE_T =
1.4759048527723095` on its `+y` ray. Both rows' notes say a move here
is a change in the class, not a baseline to restore, so the branch pins
both at their premise's stop at every ε row rather than re-aiming them.

What closes it: fixtures whose rays meet a near-coplanar triangle short
of the aimed vertex, chosen for the class and not for a mesh accident (a
fixture that states the triangle, so it does not move with a carrier's
rounding), and both pins lifted.

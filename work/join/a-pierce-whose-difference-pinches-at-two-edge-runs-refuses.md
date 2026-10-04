---
id: a-pierce-whose-difference-pinches-at-two-edge-runs-refuses
kind: issue
title: A pierce with two lone-edge Out runs refuses cube minus prism, which pinches at the pierce point (Euler SelfLoopEdge)
status: open
opened: 2026-10-04
priority: P0
cost: H
refs: [a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op]
---


## What

Found by the sweep of `a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op`.

The pose is `join_pierce_strut_facing.rs`'s L-prism and `cube_beyond(m)`,
tilted so that only the +x and +y edges of the reflex top corner
`v = (1, 1, 1)` read Out. That gives two Out runs, each a lone edge
(a fan). Examples are `m = (−1, −0.9, −1.3)`, `(−1, −0.6, −1.2)` and
`(−0.8, −0.7, −1)` (the row
`two_edge_runs_build_their_union_and_intersection`). In
`join_pierce_runs_sweep.rs`, the face placement's poses `i=6..8 j=0..2`
are this family.

The union and the intersection build `SOUND` in both orders, and so
does prism ∖ cube. Its two runs' copies stay apart on one point, since
no face's boundary runs through both and `finish::weld_pierce_copies`
leaves them as unwelded pierces are left (one point, two vertices:
the shared-point ruling, PR 3813). Cube ∖ prism refuses
`Euler(SelfLoopEdge)` from `zip::zip_seam`: both of its seams pass
the pinch vertex twice.

With the ring struts faced the other way, every op of this family
refuses (main's hard-coded facing).

**The family is wider than the lone edges** (PR 4026's review r1, m3):
cube ∖ prism refuses `SelfLoopEdge` in all 30 L-corner poses whose +x
and +y edges both read Out, including runs widened by a side face's
bisector, and not only where each run is a lone edge.

## The shape to give

This is the same question as `a-pierce-whose-wide-run-pinches-its-intersection-refuses`:
which face carries a pinch's crossed corner when neither operand's
kept faces run through both copies. Diagnose the two rows together.

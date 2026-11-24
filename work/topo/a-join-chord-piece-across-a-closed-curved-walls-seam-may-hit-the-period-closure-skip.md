---
id: a-join-chord-piece-across-a-closed-curved-walls-seam-may-hit-the-period-closure-skip
kind: issue
title: A join's chord pieces on a closed curved wall that run past its seam may meet the coplanar merge's PeriodClosure skip and stay two faces
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [the-closed-run-skip-is-described-as-unfinished-and-as-the-design, tier-3-does-not-check-maximal-faces]
---

Filed by ORBIT's `orbit/transient-chart-cut` lane. Latent: not measured.

## What

The sphere cut-in's surviving meridian was skipped by the coplanar merge
with `MergeCoplanarError::PeriodClosure` (C12(5)): the cut's two pieces
met across the cut *and* across the ball's own seam, so gluing them
would close the sphere's period, and the merge refuses the whole run
rather than kill the cut and keep the seam. That branch now closes the
cut itself before the merge (`topo::boolean::slit::close_cuts`), so the
merge is never asked.

The same shape can arise without a cut-in. The join divides a closed
curved wall (a full-turn cylinder, cone, sphere or torus face) along
chords, and the result can keep two of its pieces on one surface that
meet across a chord *and* across the wall's own seam. The merge then
meets the same same-face duplicate and skips the run with
`PeriodClosure`. Neither naming nor tier 3 checks maximal faces
(`work/restfront/tier-3-does-not-check-maximal-faces.md`), so such a
result ships non-maximal.

## To do

Measure one: a closed wall whose kept region, after a boolean, is one
region of its surface spanning the seam, built from two join fragments
(for example, a full-turn cylinder wall cut by a box across its seam and
another chord elsewhere). Then decide, with the closed-run skip's row,
whether the merge should kill the chord and keep the seam, as
`close_cuts` does.

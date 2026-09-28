---
id: an-infinite-nurbs-net-passes-check-1-where-an-infinite-analytic-datum-does-not
kind: issue
title: Check 1 refuses an infinite analytic datum and passes a NURBS net of infinities - one poison rule, two readings of it
status: review
opened: 2026-09-24
priority: P4
cost: E
refs: [ATREST-6, S330]
parent: ATREST-13
---

## What

Check 1 now refuses an analytic surface whose stored datum is `±∞`
(`ValidationError::PoisonedSurfaceDatum`, via
`geom_core::is_finite_length`, `crates/topo/src/validate.rs`
`poisoned_datums`): an infinite radius or origin is no more a locus
than a NaN one.

The same check passes a NURBS net whose control points are infinite.
`geom`'s `NetState` reads the net through `Real::is_poison`, and `+∞`
is not `f64` poison, so such a net is `Described` and earns no check-1
verdict — pinned deliberately by
`the_net_state_ladder_decides_check_1s_verdict`'s "infinite x at every
point" rung (`crates/topo/src/tier3_tests.rs`).

Both are one rule — `geom`'s totality-and-poison rule — read two ways
at one check. Which reading is right for the net is a `NetState`
question (whether "poisoned" should mean non-finite rather than NaN
for a stored control point), not a check-1 one.

## Fence

Track P for check 1; the `NetState` discriminator is `props` ground
(`crates/geom/src/net.rs`).

## Answered (ATREST-13)

One posture: check 1's `Nurbs` arm reads a `Described` net's control
points through the same `is_finite_length` read an analytic datum
takes (`crates/topo/src/validate.rs`, `net_is_finite`), and a net
carrying a `±∞` is refused as `PoisonedSurfaceDescription`, the net's
poison verdict. `NetState` is untouched: its discriminator answers
"placeholder or not", which every consumer that treats "no description
yet" benignly must ask on the scalar's own poison, and changing it
would move the merge's and the box constructors' answers too. Weights
need no read: every door into a net refuses a non-finite one
(`SplineError::NonFiniteWeight`).

The ladder's "infinite x at every point" rung is re-cut to the new
verdict and a "negative infinity at one point" rung is added
(`the_net_state_ladder_decides_check_1s_verdict`). Corpus (CI run
36154432046): no described net outside that test carries a
non-finite control point.

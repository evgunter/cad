---
id: an-infinite-nurbs-net-passes-check-1-where-an-infinite-analytic-datum-does-not
kind: issue
title: Check 1 refuses an infinite analytic datum and passes a NURBS net of infinities - one poison rule, two readings of it
status: open
opened: 2026-09-24
priority: P4
cost: E
refs: [ATREST-6, S330]
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

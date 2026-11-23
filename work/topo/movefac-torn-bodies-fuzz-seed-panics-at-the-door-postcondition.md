---
id: movefac-torn-bodies-fuzz-seed-panics-at-the-door-postcondition
kind: issue
title: movefac's torn-bodies fuzz panics at the door postcondition on seed 3482798898476541996 (ops_ring_bridge)
status: open
opened: 2026-10-07
priority: P2
cost: M
---


## What

Found by the review of PR 4240 (`join/tier3-pinch-checks-review`,
`REVIEW.md` NOTE-4) and unrelated to that PR. The fuzz row
`movefac::tests::tears::movefac_on_a_few_torn_bodies`
(`crates/topo/src/movefac/tests/tears.rs`) panics on seed
`3482798898476541996`, on the `ops_ring_bridge` fixture, at the door
postcondition in `crates/topo/src/surgery.rs` (`sweep_if_outermost`'s
`debug_assert_eq!(validate(body), Ok(()))`, "door postcondition:
result is not tier-1 valid (kernel bug)").

A torn input should be refused typed, or panic naming the torn record
(D2 row 4), before `movefac` writes. A tier-1-invalid result reaching the
door's closing sweep means some tear got through to the write.

Not reproduced by the filer. Replay with the fuzz's seed override.

## The shape to give

Replay the seed, name the tear `movefac` failed to prove, and pin it as
an ordinary deterministic row beside the fix (discipline §8).

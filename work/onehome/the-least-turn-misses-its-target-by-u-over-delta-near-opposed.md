---
id: the-least-turn-misses-its-target-by-u-over-delta-near-opposed
kind: issue
title: the least turn carries a onto b only to about u/δ near opposed, so a mate admitted close to its pole lands off its target by that much
status: closed
opened: 2026-11-10
priority: P1
closed: 2026-11-14
pr: 6962
---


Found by PR 6717's delta review (m2), and measured on `onehome/rigid-motion`. It predates this PR: the mint reproduces the chart's old arithmetic bit for bit.

`RigidMotion::least_turn` (`crates/geom-core/src/linalg/rigid_motion.rs`) is the mate chart's least turn `S`, with `S·v = c·v + k × v + k (k·v)/(1 + c)` for `k = a × b`. It is rigid within rounding at every input, pinned by `every_construction_mint_is_rigid_within_rounding_on_the_whole_sphere`. Near opposed, though, `k`'s components carry about `u` of rounding against a length `δ`, the angle short of opposed. So the turn's axis is off by about `u/δ`, and `S·a` lands off `b` by about that much.

Measured by the pin `the_least_turns_carry_near_opposed_is_its_pinned_bound`: 2000 drawn directions per `δ`, the mint's own arm, the worst `|S·a − b|`:

| δ | worst miss | miss / (u/δ) |
|---|---|---|
| 1e-1 | 1.2e-15 | 0.5 |
| 1e-3 | 1.1e-13 | 0.5 |
| 1e-5 | 1.4e-11 | 0.6 |
| 1e-7 | 1.3e-9 | 0.6 |
| 1e-8 | 1.4e-8 | 0.6 |
| 1e-9 | 1.2e-7 | 0.5 |

The pin asserts `2u/δ + 4u`.

`mate_chart_pole` admits pairs about 1e-7 from opposed. At a 1 m arm a mate there lands about `1.3e-9` m off, about `1.3ε` at the default ε. The review measured 6e-8 at 1e-7 and 4e-6 at 1e-9, about 50 times this. That reading is consistent with `1 + c` taken directly near the pole, an arm no public path can reach now that the mint chooses its own (PR 6717). Re-measure the mate path end to end before sizing the fix.

What is owed (the mate lane):
- Bound the carry where the chart admits a pair: refuse at the pole verdict a pair whose carry at the bundle's arm would exceed ε. Or take the near-opposed turn in a form whose axis does not come from a short `k`: a half turn about an axis across `a`, then the small least turn.
- Then tighten the pin.

## Closed

Closed by PR 6962 (`onehome/least-turn-axis`). The least turn's arithmetic has one home, `geom_core::LeastTurn` (`crates/geom-core/src/linalg/rigid_motion.rs`), which `RigidMotion::least_turn`, the mate chart's pole verdict and `turn_onto` all read. Where `c` reads negative its axis is `a × (a + b)`, `a × b` over the reals, whose rounding is relative to its own length, so `|S·a − b|` is at most 2.7 u at every δ from 1e-1 to 1e-9, where `a × b` gave `0.6·u/δ` (`5.4e8 u` at 1e-9). The pin above is replaced by `the_least_turn_carries_from_onto_to_within_a_few_ulps_near_opposed` (≤ 6 u, and the direct spelling shown to miss) and `the_least_turn_in_the_near_hemisphere_keeps_the_direct_bits`. No ratified text changes: D10 `[placement.values]`'s "least turn" is the same turn, computed to its rounding (geom-core W1).

What the fix does not reach, the least turn's own conditioning near its pole (a mate's carried axes, its spin and its slides along them moving by about `u/δ` with its directions' rounding, inside the ε band the chart admits), is `work/onehome/a-mates-carried-axes-near-the-least-turns-pole-are-conditioned-u-over-delta.md`, a design fork.

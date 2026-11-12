---
id: the-least-turn-misses-its-target-by-u-over-delta-near-opposed
kind: issue
title: the least turn carries a onto b only to about u/δ near opposed, so a mate admitted close to its pole lands off its target by that much
status: open
opened: 2026-11-10
priority: P1
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

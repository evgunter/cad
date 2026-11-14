---
id: a-mates-carried-axes-near-the-least-turns-pole-are-conditioned-u-over-delta
kind: issue
title: Near the least turn's pole a mate's carried axes, and so its spin and its slides along them, move by about u/δ when its directions move by their rounding, inside the ε band the chart admits
status: open
opened: 2026-11-14
priority: P1
cost: M
---


Found by ONEHOME's least-turn-axis lane, while it fixed `work/onehome/the-least-turn-misses-its-target-by-u-over-delta-near-opposed.md`. That fix makes the least turn carry `a` onto `b` within a few `u` however near opposed; it does not touch what this row records, which no arithmetic of the turn reaches.

**What D10 says.** `[placement.values]` (`docs/DESIGN.md`, "Values") reads every value beside a direction-fixing mate through **the least turn** `S` carrying where the orientation puts the copy's mated direction (`a`) onto the mated direction (`b`):
- a slide runs along one of the target's axes as `S` carries it;
- the spin is the angle about `b` "left after the least turn" (`crates/editor-core/src/mate/value.rs`, `spin` and `carried`, which read `S`'s columns);
- "within the band about that one pole the chart refuses".

**The conditioning.** Near opposed, `δ` the angle short of opposed, the least turn is itself ill-conditioned in its inputs: its axis is `a × b` normalized, of length `sin δ`, so moving `a` or `b` by `u` (the rounding every direction evaluated from geometry carries) turns the axis, and with it `S`'s carried axes, by about `u/δ`. Measured on the fix's branch: the two spellings `k = a × b` and `k = a × (a + b)`, one vector over the reals, give turns whose columns differ by `0.48–0.62 · u/δ` at every `δ` from `1e-1` to `1e-9` (300 drawn pairs per `δ`). `S·a = b` holds within a few `u` either way; what moves is the spin about `b`.

**The landing error.** So a point of the copy at reach `r` off the mated axis lands about `0.5·u·r/δ` from where the exact pose at the exact directions puts it, whatever the stored spin and slides say. At `r = 1` m:

| δ | landing error | at the default ε (1e-9) |
|---|---|---|
| 1e-6 | 1.1e-10 m | 0.1ε |
| 1e-7 | 1.1e-9 m | 1.1ε |
| 1e-8 | 1.1e-8 m | 11ε |

**The admitted band edge.** `mate_chart_pole` (`crates/editor-core/src/mate/value.rs`, the chart's pole verdict) decides the chord from the pole, `√(2(1 + c)) ≈ δ`, levered by the bundle's arm, against the run's band; it admits a pair whose levered chord is definite, at or past `escalate = K·ε` (`K = 10` by default). At a 1 m arm and the default ε that is `δ ≥ 1e-8`, where the landing error is about `11ε`. At ε = 1e-12 the edge is `δ ≥ 1e-11` and the error there about `1.1e-5` m.

**The conditioning bound.** The landing error stays within ε where `δ ≥ 0.5·u·arm/ε`: `1.1e-7` rad at a 1 m arm and the default ε, `1.1e-4` rad at ε = 1e-12. The chart's pole band admits `δ ≥ K·ε/arm`, which meets the bound only where `arm ≤ ε·√(2K/u)`: about 0.3 m at the default ε, about 0.3 mm at ε = 1e-12.

**Options** (a design fork; `[placement.values]` is ratified):
- Widen the pole's refusal to the conditioning bound: refuse, typed as the pole is, a pair whose `δ` is under `0.5·u·arm/ε` (or its stated form), so every admitted chart carries its axes within ε.
- Keep the chart's band and its meaning, and state the conditioning near the pole: where a placement's directions are within the conditioning bound of opposed, its pose is certified only to `0.5·u·r/δ`, said where the chart is documented and where the evaluation reports.
- Change the chart's reference near the pole: a half turn about a direction across `a`, then the small least turn onto `b`, whose axis is well conditioned. That changes what a stored spin and a slide along a carried axis mean for a direction past the switch, so a document's values would read differently on either side of it.

**Measure first.** None of the three is built. The row's figures are the turn's own; the mate path end to end near the pole (a placement whose mate is written about `1e-8` from its orientation's pole, its pose read at `f64` and at `Interval`) is the reproduction to add before a choice is sized.

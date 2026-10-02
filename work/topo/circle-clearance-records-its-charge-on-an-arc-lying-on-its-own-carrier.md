---
id: circle-clearance-records-its-charge-on-an-arc-lying-on-its-own-carrier
kind: issue
title: topo: bool_circle_curved_clearance records the sampled enclosure's -charge as a margin on an arc lying on the very sphere it is tested against, which k-lint reads as a micrometre feature below its floor
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## Measured

Found by the `reach/tilted-sphere-pair` lane. In that lane, lily wall 7's carve (the lantern minus a ball of r 0.16 at
`(−2.80, 0, 0.90)`) runs at the probe scalar (`demos/tour/src/lily.rs`,
`wall_probes::<Probe>`), so the K sweep's `demo/lily_walls` file now
records it. 36 `bool_circle_curved_clearance` rows per ε; the name has
no row in the committed M7 era. Instrumenting `circle_clearance`
(`crates/topo/src/boolean/reduce.rs`) printed every flagged row's pair.
Each one is a circle whose centre and radius ARE the sphere it is tested
against, so it lies on that carrier and its true residual is
identically zero:

| sphere | arc span `Δt` (rad) | recorded margin (m) |
|---|---|---|
| lantern zone, c `(−2.367, 0, 0.794)`, r 0.44 | 1.044 | −2.5497e-6 |
| same | 0.726 | −1.2325e-6 |
| same | 0.329 | −2.5352e-7 |
| ball, c `(−2.8, 0, 0.9)`, r 0.16 | 3.120 | −3.0099e-6 |
| same | 2.685 | −2.2287e-6 |
| same | 0.457 | −6.4602e-8 |
| same | 0.0218 | −1.4725e-10 |

The margins are bit-identical at ε 1e-6, 1e-9 and 1e-12, and they scale
as `Δt²` (1.044² / 0.726² = 2.07 = 2.5497 / 1.2325): this is the arc
enclosure's chord-dip charge, `|F″|·(Δt/ARC_RESIDUAL_SAMPLES)²/8`, read
as `−charge` about a zero residual. The harmonic carrier enclosure is
looser still (−0.0799 and −0.1271). The code's own comment at the
carrier-identity rung already says so: "the sampled one is `±charge`
about an identically-zero residual and reads definitely negative by
construction". The mechanism is the one
`germ/torus-coincident-pair-cannot-reach-the-covered-rung` measured on
the torus.

k-lint (`tools/k-lint`) reads these rows as follows:
- **1e-6:** 8 rule-1 in-band rows (2.2e-6 … 3.0e-6 lie in `(ε, Kε)`) and
  4 rule-2 zero-near rows.
- **1e-9:** 12 rule-3 rows below `BASELINE_FLOOR_MARGIN`, 4 rule-2
  escalation-near rows and 2 rule-2 zero-near rows.
- **1e-12:** 14 rule-3 rows and 2 rule-2 rows.

## Why no decision is fragile here

The pairs are undeclared, so the carrier-identity rung does not apply,
and the arcs are uncovered on a sphere. In that arm `Ok(Zero | Negative)`
and `Err(_)` all take the same path: the certified carrier × wall roots
(`wall_crossing`). Only `Positive` ("definitely clear") forks. A true
margin of 0 can never read definitely clear, so whether a row lands
negative, zero or in-band moves nothing. The carve builds at all three ε
(wall 7's row in PR 3817's table).

The flags are real readings of a recorded margin that is not a model
distance: it is a margin of the enclosure, not of the geometry. That is
why neither k-lint remedy fits. The margin is ε-independent, so the
ε-coupled roster does not apply. And no geometry or lever of the reach
lane produced it, so there is nothing to fix there.

## What a fix owes

One of the following:
- Do not record a margin that cannot change the outcome. The arm
  already knows that only `Positive` forks, so the clearance could
  decide "definitely clear or not" and record nothing on the side that
  does not fork.
- Rule the name in `tools/k-lint` as a pre-filter whose negative and
  in-band readings are non-decisions, with the measurement above.
- Or say why the metre rules are right for it.

Until one of these lands, the K baseline cannot be re-cut cleanly over a
scene that carves a sphere along a tilted section.

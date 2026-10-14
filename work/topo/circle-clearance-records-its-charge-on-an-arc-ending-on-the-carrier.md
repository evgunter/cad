---
id: circle-clearance-records-its-charge-on-an-arc-ending-on-the-carrier
kind: issue
title: topo: bool_circle_curved_clearance records the sampled enclosure's -charge as a margin on an arc that ends on the sphere it is tested against, which k-lint reads as a micrometre feature below its floor
status: closed
opened: 2026-10-02
priority: P3
cost: M
closed: 2026-10-02
pr: 3817
---


## Measured

Found by the `reach/tilted-sphere-pair` lane. In that lane, lily wall 7's
carve (the lantern minus a ball of r 0.16 at `(−2.80, 0, 0.90)`) runs at
the probe scalar (`demos/tour/src/lily.rs`, `wall_probes::<Probe>`), so
the K sweep's `demo/lily_walls` file now records it. It recorded 36
`bool_circle_curved_clearance` rows per ε; the name has no row in the
committed M7 era.

Instrumenting `circle_clearance` and its caller
(`crates/topo/src/boolean/reduce.rs`) showed what every flagged row
is: a FRAGMENT of one operand's meridian, split at the section, tested
against the OTHER operand's sphere. The lantern's zone meridians
(circle centre `(−2.367, 0, 0.794)`, r 0.44) are tested against the
ball, and the ball's meridians (centre `(−2.8, 0, 0.9)`, r 0.16)
against the zone. Each fragment ends at a cut vertex, which lies ON the
sphere it is tested against. Its residual is therefore exactly zero at
that end, and its true one-sidedness margin is at most zero.

| meridian | fragment span `Δt` (rad) | recorded margin (m) |
|---|---|---|
| lantern zone | 1.044 | −2.5497e-6 |
| lantern zone | 0.726 | −1.2325e-6 |
| lantern zone | 0.329 | −2.5352e-7 |
| ball | 3.120 | −3.0099e-6 |
| ball | 2.685 | −2.2287e-6 |
| ball | 0.457 | −6.4602e-8 |
| ball | 0.0218 | −1.4725e-10 |

The margins are bit-identical at ε 1e-6, 1e-9 and 1e-12, and they scale
as `Δt²` (1.044² / 0.726² = 2.07 = 2.5497 / 1.2325). This is the arc
enclosure's chord-dip charge, `|F″|·(Δt/ARC_RESIDUAL_SAMPLES)²/8`,
widening the sample hull past the zero end and read as `−charge`. The
unsplit meridians, whose ends are off the other sphere, read −0.0799
and are not flagged.

k-lint (`tools/k-lint`) read the rows as follows:
- **1e-6:** 8 rule-1 in-band rows and 4 rule-2 rows.
- **1e-9:** 12 rule-3 rows and 6 rule-2 rows.
- **1e-12:** 14 rule-3 rows and 2 rule-2 rows.

No decision was fragile. The arcs are uncovered on a sphere, and in that
arm `Zero`, `Negative` and an escalation all fall through to the
endpoint arms. Only `Positive` returns early, and an arc with a zero end
cannot read it.

## Closed by

PR 3817 (`reach/tilted-sphere-pair`). `curved_face_arm` decides an
uncovered circle's endpoint sides first, for sphere, cylinder and torus
faces. An arc with an end ON the carrier is never asked its clearance;
it goes straight to the endpoint arms, which receive the sides rather
than deciding them a second time.

The behaviour is unchanged. A held side escalation surfaces in those
arms exactly as before. It is dropped only when the clearance reads
definitely clear, and an in-band end cannot allow that.

`demo/lily_walls` now records 21 clearance rows per ε, all with
`|m| ≥ 7.4e-3`. The row is
`crates/sweep/tests/tilted_sphere_pair_k_rows.rs`, at the probe scalar,
rostered in `scripts/gates/probe-suite-census.sh`. It is red without
the change (union of the equal pair: a clearance of −1.6e-6) and green
with it.

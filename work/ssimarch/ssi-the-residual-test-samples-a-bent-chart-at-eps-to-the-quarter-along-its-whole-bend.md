---
id: ssi-the-residual-test-samples-a-bent-chart-at-eps-to-the-quarter-along-its-whole-bend
kind: issue
title: ssi/march: the residual test samples a flat wall whose chart bends at an ε^−¼ density along the whole bend, against main's certificate-local density: 3.5–5.3× main's samples at ε 1e-12
status: open
opened: 2026-10-05
priority: P3
cost: M
---


(SSI implementer on `ssi/neighbour-cap`, PR 4034, from the r3 delta
review's measurement.)

## What

A flat wall `z = y − 0.3` whose chart is straight for `u ≤ ½` and
bends after a C¹ knot (`y = v + β·ψ(u)`, `ψ = (u − ½)²` past the knot):
the intersection is a straight line in ℝ³ carried by a bent pcurve.
PR 4034's march keeps a step only where its predicted state's residual
is within ε plus the settle tolerance (`ssi/march.rs`, the residual
test). The carrier rungs read no curvature on a straight carrier, so
the residual test sets the density along the whole bent stretch, as
`ε^−¼`. Main's march (the crossing cap) samples coarsely and its
certificate adds samples only where it refuses, near the knot. Samples
on the PR's head against main's, β = ±8 and the review's bent-chart
family (`probe2_straight_on_bent_charts`,
`probe_flat_wall_chart_bends_after_a_knot` on
`analysis/neighbour-cap-review/4034-r2`):

| ε | head ÷ main |
|---|---|
| 1e-6 | 0.74–0.93× |
| 1e-9 | 1.3–2.0× |
| 1e-12 | 3.5–5.3× (β = 8: 1081 against 307; β = −8: 1552 against 294) |

Head samples grow as about `ε^−0.23`, main's as about `ε^−0.09`.
Turning the state rungs off moves nothing; turning the residual test
off returns main's counts (β = 4, ε 1e-12: 944 against 313). A cost, not
a correctness defect: every such branch certifies, paired right.

## Done when

The march's density on a bent chart under a straight carrier follows
what the certificate asks (for example, pricing the predictor against
the fit's error rather than ε, or letting refinement add the samples
the residual test now adds everywhere), and β = ±8 at ε 1e-12 takes no
more than about twice main's samples; or a measurement shows the
density is what the certificate needs.

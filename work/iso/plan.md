# ISO — the plan

the trimmed and rational lane: iso derivation, composite rounds, and the loft seam's exact compare

Opened 2026-09-20 by CHART's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**24.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `loft-seam-carrier-exact-knot-compare` | H | loft_body refuses at degree ≥ 2 on generic parameterizations: pcurve_cache compares the seam carrier's knots/weights by exact equality |
| P1 | `iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain` | H | nurbs_iso_derive's rim arms map an edge's whole interval onto the chart's whole u domain, so mint_pcurves refuses on any body whose spline-chart wall edge has been split |
| P1 | `non-separable-rational-interior-column` | H | An interior column of a chart whose weight net varies along both parameters has no exact class - the composite bound without a tube |
| P1 | `rational-gates-test-unit-weights-not-constancy` | E | The cap class's 'is this rational' gate tests weights == 1.0 where any constant weight vector is polynomial - over-strict by TRIM-1's own insight |
| P1 | `trimmed-quadrature-composite-rounds` | H | The trimmed quadrature fences the Newton-Cotes window at p_u + p_v <= 4; the composite fallback is unbuilt |
| P3 | `S351` | E | nurbs_iso placement rule is now cited from two places in geom; a move must re-aim both pointers |
| P3 | `curved-trim-e2e-fixture-waits-for-a-producer` | D | No end-to-end body exercises the trimmed lane's chord machinery; the only General image at rest is a 2e-16 rectangle |

## Order

`loft-seam-carrier-exact-knot-compare` first and alone if need be:
`loft_body` REFUSES at degree >= 2 on generic parameterizations because
`pcurve_cache` compares knots exactly. A loft that refuses on ordinary
input is the only P0 row here and the only one a user meets.

Then `iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain`,
the premise the rest of the lane rests on, and
`non-separable-rational-interior-column` behind it.
`curved-trim-e2e-fixture-waits-for-a-producer` is blocked on geometry
nothing mints yet — read it before planning around it.

## Review posture

OPEN, for this program's first dispatch. CHART inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.

---
id: boolean-door-tier-3-waits-on-the-description-gap
kind: issue
title: The boolean door gates tiers 1-2 only: 55 of 687 topo-corpus results it ships fail tier 3, so gating tier 3 there needs the description-gap decision
status: open
opened: 2026-10-02
priority: P1
cost: H
design: true
---


The half of
`boolean-door-passes-a-geometrically-open-result-the-backstop-cannot-see`
that the volume backstop cannot close. A result missing a face whose
zip glues the two free edges is topologically closed (tiers 1 and 2
pass) and can enclose a short POSITIVE volume. No inequality over
`vol(A)`, `vol(B)` and the result bounds `vol(A ∩ B)` from below: that
lower bound is `vol(A) + vol(B) − vol(A ∪ B)`. Tier 3 sees such a body
(an arc edge in a plane face, `PlanarBoundaryResidual`), but
`boolean::ops::gate` runs tiers 1 and 2 only. Its doc says why: "tier 3
is an at-rest posture with the PR 3 description gap".

## Measured

The instrument is topo's `door-tier3-meter` feature
(`crates/topo/src/boolean/door_meter.rs`), run by
`python3 scripts/door-tier3-meter.py`. It runs `AtRestPolicy::gate_at_rest`
on every result `boolean_op_recut` builds, after `gate` and the volume
backstop, and on both operands. It times the op, tier 3 and the backstop,
and changes no result. Measured on `reach/door-backstop` (`ci` profile,
default ε):

| corpus | results | tier 3 refuses | of those, shipped | shipped with tier-3-clean operands | tier 3 / op time | median / p90 / max per result | backstop / op |
|---|---|---|---|---|---|---|---|
| topo | 693 | 55 | 55 | 3 | 15 % | 11 % / 27 % / 79 % | 6 % |
| sweep | 1168 | 42 | 0 (the backstop refuses the same bodies `VolumeUnmeasured`) | 0 | 13 % | 13 % / 23 % / 226 % | 25 % |

(On `cd49025f`, before the backstop's interval re-derivation, the same
probe found 52 of the 55 topo refusals shipped.)

- Every result the door ships from verb-built operands (the `sweep`
  corpus) passes tier 3.
- Of the 55 shipped topo results that fail tier 3, 52 come from
  operands that fail it too. The tier-3 errors are `ScaffoldAtRest` and
  `TransverseNotIntrinsic` on the hand-built `review_m3_pr55` fixtures,
  `surgery::tests`, and the two `refusal_routes::offer_rows`
  far-origin rows, which also carry `PlanarBoundaryResidual` and
  `PlanarFaceResidual`.
- The other 3 are `contact9_side_codes` slivers whose operands pass
  tier 3. The result carries a door-minted `ScaffoldAtRest` edge, and
  two of them also carry `LaminaWedge`.

So gating tier 3 at the door refuses bodies it ships today. Some have
tier-1/2-only operands, and some are slivers whose minted edge keeps a
scaffold description. That is the description-gap decision the gate's
doc names: whether the door's currency is tier 3, and what it does with
operands below it. The decision is not taken here. The cost of tier 3
is about 15 % of op time on both corpora.

## What would close it

Decide the description gap (designers first,
`memories/orchestration-model.md`). Then either gate tier 3 at the door,
or gate the part of it that sees an off-carrier edge (check 3's
planar vertex residuals and check 5's planar boundary containment).
With the decision taken, measure again what the corpus ships.

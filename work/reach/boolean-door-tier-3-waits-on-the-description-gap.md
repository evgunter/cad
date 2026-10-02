---
id: boolean-door-tier-3-waits-on-the-description-gap
kind: issue
title: "The boolean door gates tiers 1-2 only, citing a description gap closed since M3 PR 6a: what a door owes for what it ships and consumes (the finished-body contract)"
status: open
opened: 2026-10-02
priority: P1
cost: H
design: true
needs_ev: true
branch: reach/ev-door-finished-body
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

## Designers (2026-10-02)

Two designers weighed this (`docs/DESIGN-FORK-LOG.md`, row 46). Both
found the description gap closed: M3 PR 6a mints honest seam
descriptions, and the gate's doc is stale. The open question is the
door contract.

After two reconciliation rounds, the first of which crossed, they
recommend one final state:
- the finished body is a type (`AtRestBody`);
- every verb door takes it and returns it (tier 3, or tier 3′ with
  contacts), so each body is gated once, at the door that built it;
- Euler operators hand back construction state;
- D1's tier-2 sentence is changed;
- check 7 takes the backstop's interval re-derivation before or with
  the gate, and the positivity arm retires after.

Put to Ev on the `[ev]` PR from `reach/ev-door-finished-body`.


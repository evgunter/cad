---
id: the-blends-g1-and-coaxiality-verdicts-are-unrecorded-coincidences
kind: issue
title: The blend battery's chain-G1 and support-coaxiality Zero verdicts are coincidences decided from values that no stage-4 unit records
status: open
opened: 2026-10-08
priority: P2
cost: M
---


## What

Found by INTENT stage 4 PR B's sweep of the value decisions that make
cells one (the PR body's hit list). PR B records the battery's
isosceles turn (`fillet3_turn_isosceles`, `turn_at`) as a
`topo::Coincidence`, as the spec's §3 names. The same battery decides
two more coincidences from values, and nothing records them:

- `chain_g1` (`crates/sweep/src/blend/battery.rs`, `fillet3_chain_g1`):
  a Zero `sin θ` between two links' tangents at a joint is read as
  tangent continuity, and one band runs through the joint. That is a
  tangency decided from values, the class D10 says is constructed
  ("tangency is constructed") and stage 4 §14 Q6 says is recorded and
  left to rung 3.
- `support_coaxiality` (`fillet3_support_coaxiality`): a Zero departure
  reads two curved supports as sharing one axis or ruling, and the band
  is minted as an exact torus on it. That is coaxiality decided from
  values; D10: "Coaxiality is one `Axis` read twice".

`cap_transverse` (circle vs ellipse at a cap) is a kind decision on one
band end, not a coincidence of two cells, and is not listed.

## What closing it takes

Each Zero arm records a row (`Relation::Tangent { aligned }` and a
coaxiality relation the record does not have yet; `DecisionSite`
arms of their own), carried on `Blended::coincidences` beside the turn
rows. The cells are the two links' edges, or the two supports. No unit
of `docs/INTENT-STAGE4-SPEC.md` names these sites: G covers profile
junctions, E the boolean's. The orchestrator places it.

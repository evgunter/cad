---
id: the-blends-g1-and-coaxiality-verdicts-are-unrecorded-coincidences
kind: issue
title: The blend battery's chain-G1 and support-coaxiality Zero verdicts are coincidences decided from values that no stage-4 unit records
status: closed
opened: 2026-10-08
priority: P2
cost: M
closed: 2026-10-10
branch: intent/blend-g1-coaxiality-recorded
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

## Closed

Both Zero arms record a row, carried on `Blended::coincidences` after
the coaxiality rows and before the turn rows
(`BatteryVerdict::coincidences`):

- `fillet3_chain_g1` at a junction: `Relation::Tangent { aligned }`
  between the two links' edges, at `DecisionSite::BatteryJoint`
  (`battery.rs` `chain_turns`, rows on `BatteryVerdict::joints`).
  `aligned` reads the two tangents, both heading into the junction,
  as opposed. A self-closed link's own seam is one cell, so it
  records nothing.
- `fillet3_support_coaxiality`: `Relation::Coaxial`, new, between the
  link's two support faces, at `DecisionSite::BatterySupportAxis`
  (`resolve_link`, on `Link::support_axis`). The planar rows decide
  none.

The rows are named by the document like the turn rows, and the lint
reports each one unproven. A filleted disc reports both kinds. Pins:
`sweep/tests/blend_value_decided_rows.rs` and `editor-core`'s
`coincidence_door::a_filleted_disc_records_its_coaxial_supports_and_tangent_joints`.

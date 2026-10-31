---
id: tier-3-refuses-a-carrier-with-an-interior-c0-knot
kind: issue
title: tier 3 refuses a face chart or edge carrier with an interior C⁰ knot, read exactly from its knot structure
status: parked
opened: 2026-10-28
priority: P1
cost: E
refs: [loft-of-three-or-more-sections-at-degree-one-is-unmeshable]
blocked_on: [the-skin-cuts-an-arc-wall-at-each-sub-arc-joint]
---


## Ruling

Ev's ruling on [ev] PR 5913 (2026-10-28, Decision 2: "S, then"), now DESIGN [topology]: every face chart and edge carrier is C¹ by its knot structure strictly inside its cell; a construction whose carrier is not puts an edge or vertex there; import splits at its door; no reader carries a crease arm.

## Owed

Tier 3 checks that `KnotStructure::crease_runs` is empty on every face chart and edge carrier, and refuses typed, naming the face or edge and the knot. The check is structural, with no tolerance band.

**Sequencing:** it lands after `the-skin-cuts-an-arc-wall-at-each-sub-arc-joint` and `import-splits-a-foreign-face-or-edge-at-each-c0-knot`. Before they land, valid native and imported bodies would fail it.

Filed by the TESS orchestrator at the ruling.

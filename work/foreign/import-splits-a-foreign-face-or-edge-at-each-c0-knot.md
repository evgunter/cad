---
id: import-splits-a-foreign-face-or-edge-at-each-c0-knot
kind: issue
title: STEP import splits a foreign face or edge at each interior C⁰ knot at its door, so nothing imported carries a crease inside a cell
status: open
opened: 2026-10-28
priority: P1
cost: M
refs: [loft-of-three-or-more-sections-at-degree-one-is-unmeshable]
---


## Ruling

Ev's ruling on [ev] PR 5913 (2026-10-28, Decision 2: "S, then"), now DESIGN [topology]: every face chart and edge carrier is C¹ by its knot structure strictly inside its cell; a construction whose carrier is not puts an edge or vertex there; import splits at its door; no reader carries a crease arm.

## Owed

At the import door, split each foreign face chart and edge carrier at every interior knot of multiplicity at least its degree. The split is exact: the knot vector already separates the sub-nets. Mint an edge (or a vertex on a curve) at each knot, carrying its pcurves through the split. The joint is a smooth seam where the foreign surface was smooth there, and a crease edge where it was not. No numeric verdict decides which, because both are edges.

**Sequencing:** land it before the tier-3 rule.

Filed by the TESS orchestrator at the ruling.

---
id: import-splits-a-foreign-face-or-edge-at-each-c0-knot
kind: issue
title: STEP import splits a foreign face or edge at each interior C⁰ knot at its door, so nothing imported carries a crease inside a cell
status: open
opened: 2026-10-28
priority: P1
cost: M
refs: [loft-of-three-or-more-sections-at-degree-one-is-unmeshable]
design: true
---


## Ruling

Ev's ruling on [ev] PR 5913 (2026-10-28, Decision 2: "S, then"), now DESIGN [topology]: every face chart and edge carrier is C¹ by its knot structure strictly inside its cell; a construction whose carrier is not puts an edge or vertex there; import splits at its door; no reader carries a crease arm.

## Owed

At the import door, split each foreign face chart and edge carrier at every interior knot of multiplicity at least its degree. The split is exact: the knot vector already separates the sub-nets. Mint an edge (or a vertex on a curve) at each knot, carrying its pcurves through the split. The joint is a smooth seam where the foreign surface was smooth there, and a crease edge where it was not. No numeric verdict decides which, because both are edges.

**Sequencing:** land it before the tier-3 rule.

Filed by the TESS orchestrator at the ruling.

## How import satisfies the rule (FOREIGN designer pair, 2026-10-29)

Measured on the unit's branch (draft PR 6013): the standard rational-quadratic conic (equal arcs, weights 1 : cos(θ/2) : 1) is C¹ as a locus at every arc joint, not C², and C⁰ by its knot structure, which tests the homogeneous spline: the weight function's slope flips sign at the joint, so the double knot never comes out. Every torus, sphere, cone or swept circle a file writes as a spline is therefore cut at every joint, at the exporter's span count: the QUAD acceptance quarter torus goes from 2 faces to 162 and its import from 5.3 s to 125–146 s (release).

Import satisfies the rule in this order:
1. **Promote** a conic net to its analytic kind (torus, sphere, cone; open circular arcs for curves) on the whole patch, before any split (D7, ruling #256).
2. **Re-represent** a spline that stays NURBS by a carrier one degree higher whose interior knots are C¹ by structure (each span scaled linearly in homogeneous coordinates; a joint that is only G¹ re-knotted so the one-sided speeds match), adopted where its distance from the file's carrier certifies at ε_in, as promotion's is.
3. **Split** at every joint the first two leave C⁰, as the branch does. A real corner always splits.

Separately, the at-rest quadrature's round-zero schedules are per face (`AREA_CELLS_PER_AXIS`, `AXIS_INIT_PIECES`, `HULL_BLOCKS_PER_AXIS`, `REFINED_SPANS_PER_AXIS`), so a tile gets a whole patch's grid; they become per knot span (`work/quad/the-at-rest-gates-cost-grows-with-the-face-count-of-a-split-conic-patch.md`).

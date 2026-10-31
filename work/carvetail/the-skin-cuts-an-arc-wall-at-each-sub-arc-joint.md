---
id: the-skin-cuts-an-arc-wall-at-each-sub-arc-joint
kind: issue
title: the skin cuts a spline wall at each sub-arc joint (a double knot inside an arc wider than MAX_SUB_ARC), so a lofted arc wall carries no C⁰ knot inside it
status: open
opened: 2026-10-28
priority: P1
cost: H
refs: [loft-of-three-or-more-sections-at-degree-one-is-unmeshable]
---


## Ruling

Ev's ruling on [ev] PR 5913 (2026-10-28, Decision 2: "S, then"), now DESIGN [topology]: every face chart and edge carrier is C¹ by its knot structure strictly inside its cell; a construction whose carrier is not puts an edge or vertex there; import splits at its door; no reader carries a crease arm.

## Owed

`sweep::skin` writes an arc wider than `MAX_SUB_ARC` (about 95.6°) as rational-quadratic sub-arcs joined at a double knot. A lofted or swept wall over such an arc therefore carries a C⁰ knot where the surface is geometrically smooth.

The skin, the loft and the sweep should mint one wall per sub-arc, with a smooth seam edge (a 180° wedge) at each joint, and name the new walls and seams. Ev accepted that the face count of a lofted arc wall then follows the sub-arc ceiling: a lofted circle is four walls, and a lofted one-segment loop is no longer one wall.

- **Also closes:** the residue of `loft-of-three-or-more-sections-at-degree-one-is-unmeshable`. The integral arm of `patch_bound` refuses a crease at every degree, and after this row no wall reaches it with one.
- **Re-baseline** what moves (names, goldens, tour pins), naming the cause.
- **Sequencing:** land it before the tier-3 rule, `tier-3-refuses-a-carrier-with-an-interior-c0-knot`.

Filed by the TESS orchestrator at the ruling.

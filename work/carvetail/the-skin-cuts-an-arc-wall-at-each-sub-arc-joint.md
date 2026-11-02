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

## Evidence: klein wall 3 stops on this crease (2026-10-31)

From SPLICE's `splice/rim-sector-clearance`, at ε 1e-9, 1e-6 and 1e-12 alike. With the neck seam cleared at the loop's rim, the union of the bulb and the loop (klein wall 3, `demos/tour/src/klein.rs` `wall3_pinned`) refuses `SplineCrossingUnsettled { edge 4v1, face 3v3, why: Crease }`. Face 3v3 is a half tube of the swept loop, its section two quarter arcs joined at a double knot `u = ½` (`crease_values() == [0.5]`) in the seam plane `y = 0`. The flare cone's seam line, edge 4v1 from `(0.312, 0, 2.443)` along `(½, 0, −√3/2)`, crosses the wall transversally on that joint, near `t ≈ 0.577`, chart `(0.5, 0.819)`. The spline-face arm refuses any meeting on a crease (`geom_brep::curve_face::spline_face`, the crease rule). Once the sweep mints an edge at the joint, as this row owes, the crossing is a meeting on a boundary edge, which the boundary pass reads.

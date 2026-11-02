---
id: second-order-side-readings-the-stage-1-sweep-left-beside-the-reading
kind: issue
title: bends_into_material, tangent_sector_osculation and slit's wall-against-floor descent read a second-order side beside bound_side
status: open
opened: 2026-11-01
priority: P2
cost: M
parent: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
---


Item 8 of
`the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading`
(found by PR 6217's review), not built in stage 2
(`tangrim/sector-passes-descend`). Each reads a second-order side of a
face beside `bound_side`:

- `geom_brep::bends_into_material`: a wall's curvature sum against its
  tangent plane;
- `splitting/rules.rs` `tangent_sector_osculation`;
- `boolean/slit.rs` `bool_rider_wall_side` and `walls_inside`'s descent:
  a wall's first-order side against the floor, then the parting.

Stage 2 folded the two readers PR 6219 added (`recl`'s `curving`, now
`flank_key`'s descended key; `vtxfac`'s `TangentRead`, now the entries'
own descended classes), which the review counted with these.

Also left: `sectors::FaceShape::of_face`, read by callers that hold a
face but no sector (`boolean/mod.rs`'s partner shape, `slit.rs`); every
sector caller now reads `BoolSector::bend`.

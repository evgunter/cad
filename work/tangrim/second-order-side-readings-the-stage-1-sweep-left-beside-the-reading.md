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

## `tangent_relative_side`, a bound reader with its own ladder (PR 6253's review, 2026-11-02)

`boolean::sectors::tangent_relative_side` still reads a bound's side of
a declared-tangent partner one order down, beside `bound_side`: the two
carriers' graph accelerations along the bound, differenced, through
`geom_brep::parting_side`. Its two production callers:

- `recl::recl_sectors`'s per-bound descent on a coplanar record whose
  faces are glued `Tangent` (each bound of both sectors against the
  other's carrier);
- `sectors::tangent_lump`, each bound of a tangent sector against the
  pierced or partner carrier.

The ladder differs from `Tie::descend`'s: a parting in the gap refuses
there (`of_lever(LeverArm::SectorCurving, …)`), where `Tie::descend`
reads it `Unresolved::Gap` and its consumers `On`; and it reads the
carrier's normal section along the bound, never the bound's own bend
(for an edge on the carrier, equal by Meusnier only at the tangency).
Routing it through the bisector arm of `Tie::descend` changes a
declared tangency's refusal into a fallback, which stage 2 did not
measure, so it was left.

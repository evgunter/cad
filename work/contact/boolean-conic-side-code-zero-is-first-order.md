---
id: boolean-conic-side-code-zero-is-first-order
kind: issue
title: A curved edge's side code at a boolean vertex reads its departure to first order, so an arc tangent to the other face at the vertex reads On while it curves off
status: open
opened: 2026-09-28
priority: P3
cost: M
---


Filed by CONTACT-9. Unverified: no fixture has reached it.

`boolean::sectors::side_code` (`crates/topo/src/boolean/sectors.rs`,
`Reach::Extent`) reads a conic or fitted edge's side of a face plane
at a boolean vertex as its departure direction levered at the edge's
own extent. A definite sign is exact. A Zero is a first-order tangency:
the departure lies within the band of the plane over the extent, to
first order. An arc that departs tangentially and then curves off by
more than the band reads On, and `vtxfac`'s on-edge resolution (or
`recl_edges`) hands it its neighbours' side.

The splitting twin descends one order at the same Zero
(`splitting/neighborhood.rs`, `split_conic_departure`, then
`geom_brep::enters_material_order2`). The boolean has that descent only
for declared `Tangent` pairs (`tangent_lump`).

If this is reached, a wrong germ at the vertex refuses at the join
(`SplitJoinError::UnpairedLooseEnds`, whose message names this cause).
The fix's shape is the splitting lane's: descend to second order at
the Zero, and keep On only for a second-order tie.

**Reached, in its declared form (TANG m9-3 residues, 2026-10-01).**
`crates/sweep/tests/m9_3_zip.rs`'s
`a_tangent_curved_sector_on_a_face_lumps_whole` puts a quarter round's
arc, tangent at its end vertex, on a face containing that vertex: the
arc bound reads On at first order exactly as described. Declared
`Tangent`, the wall sector descends through `tangent_lump`; undeclared,
the curved on-carrier sector refuses C8 (`CurvedBooleanUnsupported`)
first, so the undeclared wrong-germ path is still not reached. The same
fixture shows the arc split at the band's edge
(`work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`).

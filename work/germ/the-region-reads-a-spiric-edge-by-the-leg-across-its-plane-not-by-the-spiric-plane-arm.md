---
id: the-region-reads-a-spiric-edge-by-the-leg-across-its-plane-not-by-the-spiric-plane-arm
kind: issue
title: The region reads a spiric edge by its leg across the edge's plane: the (Spiric, Plane) arm is unbuilt, and refusing the edge instead reds ten plane × torus rows
status: closed
opened: 2026-11-21
closed: 2026-11-22
pr: 7431
priority: P1
cost: M
rides_with: co-surface-direction-table-and-spiric-plane-door
---


Filed by GERM's `germ/crossing-consumers` lane (U3b of
`docs/GERM-CROSSING-U3-SPEC.md`).

## Finding

U3b retires `region`'s across-then-`under`: `under` becomes the
`(Spiric, Plane)` arm's method if it is a correct general reading of a
spiric span against a plane, and otherwise a spiric edge is the unbuilt
route's window, refused typed.

**`under` is not that reading.** It reads the *leg* (a circle on the
torus) across the spiric edge's cutting plane `P`, and keeps a meeting
by the torus door's placement (`TorusFaceRegion::on_edge_plane`,
`crates/topo/src/torus_region.rs`): `P` meets the torus in the edge's
oval and its mirror, so a point of the leg on `P` is on one of them. It
needs a third curve on the torus. A spiric span `S` against a general
plane `Q` meets it on `S ∩ (P ∩ Q)`, a line against the torus; no curve
in `Q` on the torus is given.

**The refusal branch reds GERM's own plane × torus rows.** With a
spiric edge refused typed in `region::landings` (a blocked path,
`RegionRefusal::Unreadable`), ten sweep rows red on point-in-solid
`EdgeCarrierUnsupported { carrier: Spiric }`:
`germ_plane_torus_spiric::{a_plane_near_the_inner_equator_builds_its_true_bodies,
every_point_of_the_spiric_box_on_the_one_face_torus_reads_its_side,
the_donuts_spiric_poses_build_their_true_bodies,
the_fat_corner_bars_lens_is_the_true_body,
the_one_face_and_seamed_tori_s_spiric_poses_answer_as_the_donuts}`,
`germ_plane_torus_frame::{a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name,
two_one_site_loops_on_one_face_pair_pass_the_dispatch}`,
`germ_torus_doors::subtract_and_intersect_reach_the_doors_union_does`,
`a_ray_beside_a_shallow_spiric_edge_is_read_by_the_span_walk`,
`spline_spiric_operands::a_poke_into_the_cavitys_torus_band_refuses_typed`.
`route`'s note ("no producer reaches a boolean with one") holds of
operand edges, not of a torus face's own boundary mid-boolean.

## What U3b did instead

A spiric edge keeps the leg read across its plane (`ReadAcross::LegOverEdge`,
as main's torus door chose), now through `cross`: `OnSurface::under` reads
the leg's circle against the edge's plane, a built arm, with the record
check, and the torus door places each meeting on the oval
(`TorusFaceRegion::on_edge_plane`). A parallel leg against a circle edge
keeps the same direction, as on main.

## To decide

Whether the `(Spiric, Plane)` arm is built (its pose and a method on
`P ∩ Q` against the torus, then the oval's own loop and span), so
`spiric_landings` reads the edge across the leg's plane like any other;
or the leg-across-the-edge reading is ratified as the co-surface arm's
own spiric reading (two curves on one torus), in `curve_pair::on_surface`
beside `across`.

## Closed (2026-11-22, PR 7431)

Ev ratified the design in PR 7301: the `(Spiric, Plane)` arm is built
(`curve_face::spiric_plane`, by section reduction), and which curve is
read across which plane is the co-surface arm's direction table
(`curve_pair::on_surface::direction`), which reads any leg against a
spiric edge across the edge's plane. The region reads the pair through
the arm (`OnSurface::meet`), which places each meeting on the edge;
`Legs::across`, `Legs::on_edge_plane` and the torus door's placement are
retired. The ten rows above pass on the branch.


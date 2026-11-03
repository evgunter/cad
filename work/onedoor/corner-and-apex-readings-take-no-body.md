---
id: corner-and-apex-readings-take-no-body
kind: issue
title: "[topo.homes] arm 1: sector_shape.rs's apex reading takes no Body and stays in topo, waiting on sector_face::ConeApex"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep` (2026-11-03), which moved `sector_shape`'s rungs, and the corner reading over them (`CornerBounds`, `CornerShape`, `FullTurn`, `TurnOrder`, `corner_shape`, `reads_slit_tip_turn`), to `geom_brep::sector_shape`, and left the apex half.

In `crates/topo/src/sector_shape.rs`, only `SlitTips::of` (and `of_operands`) read a `Body`; `SlitTips` holds `(VertexKey, FaceKey)` pairs, so it stays. The apex half takes no `Body` and no arena key: `ApexShape`, `apex_shape`, `apex_corner_shape`, and the `sector_apex_tilt` / `sector_apex_window` rows. It stays because it reads `crate::sector_face::ConeApex` (`crates/topo/src/sector_face.rs`), a carrier datum with no `Body`, which has to move to `geom-brep` first; the apex half then follows it beside `geom_brep::sector_shape`. Each crate's suite guards its own sources for the rung names (`the_rungs_are_decided_in_one_place`, in both `geom_brep::sector_shape` and `topo::sector_shape`), so that row needs no move.

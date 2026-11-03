---
id: corner-and-apex-readings-take-no-body
kind: issue
title: "[topo.homes] arm 1: sector_shape.rs's corner and apex readings take no Body and stay in topo"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep` (2026-11-03), which moved `sector_shape`'s rungs to `geom_brep::sector_shape` and left these.

In `crates/topo/src/sector_shape.rs`, only `SlitTips::of` (and `of_operands`) read a `Body`. These take none and no arena key: `CornerBounds`, `CornerShape`, `FullTurn`, `TurnOrder`, `corner_shape`, `reads_slit_tip_turn`, `ApexShape`, `apex_shape`, `apex_corner_shape`, and the `sector_apex_tilt` / `sector_apex_window` rows. The apex half reads `crate::sector_face::ConeApex` (`crates/topo/src/sector_face.rs`), a carrier datum with no `Body`, which would move first. `SlitTips` holds `(VertexKey, FaceKey)` pairs, so it stays; `corner_shape` takes only the `CornerBounds` it computes. The anti-re-fork guard (`the_rungs_are_decided_in_one_place`) stays beside `SlitTips` or moves to `geom-brep` walking both crates.

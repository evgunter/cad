---
id: crossing-doors-leave-topo-for-geom-brep
kind: unit
title: "[topo.homes] arm 1: the Body-free curve x face doors (plane_crossing_lane, circle_torus, ellipse_torus, circle_roots, section_cert, carrier_eq) move to geom-brep beside curve_face::route, and their K names take the deciding module's prefix"
status: open
opened: 2026-11-03
priority: P1
cost: H
---


Unit 2 of `[topo.homes]` (PR 6426), arm 1: code that takes no `Body` lives in `geom-brep`.

`geom_brep::curve_face::route` is the one table of how a curve span meets a face, and it marks `Door::PlaneCrossing` built. Yet that door is `topo::splitting::classify::plane_crossing_lane`, two crates up. The same holds for the other Body-free doors and readings below.

The move:
- `plane_crossing_lane`, `PlaneCrossingLane` and `ConicPlaneMeet` become the `PlaneCrossing` door in `geom-brep`, beside `route`;
- `boolean/{circle_torus, ellipse_torus, circle_roots}` go with their doors;
- `section_cert` and `carrier_eq` go too, each only if it reads no `Body` (check item by item);
- `bound_side`'s reading and `sector_shape`'s pure `Vec3`/`Band` rungs move by the same test (the designers were unsure on the rungs alone).

The callers inside topo then read the doors from `geom-brep`. `topo::validate::decide` is a passthrough to `geom_core::k_stats`, and `geom-brep` already decides through that, so K recording holds nothing in `topo`.

**K names:** the six `split_conic_*` names `plane_crossing_lane` decides take the crossing door's prefix. Record old → new in the K-REPORT census note, as #661 did. This is a rename, not a pooling question: each name is one site and one population.

The guard's allowlist must shrink by exactly the lines this unit clears.

Parked on the guard so the move list exists first. Review tier: dual, since it moves shared code several programs read and renames K populations.

**Scope addition (from PR 6458's review, 2026-11-03):** `sector_shape.rs`'s Body-free functions also take no `Body`, so arm 1 sends them to `geom-brep`: `sector_shape`, `plane_offset`, `point_segment_distance`. Its deleted placement paragraph said they "would compile in geom-brep today". When `boxes` moves, `boolean/boxes.rs`'s `PINNED` per-file census of calls into `boxes` must move with the guard's allowlist.

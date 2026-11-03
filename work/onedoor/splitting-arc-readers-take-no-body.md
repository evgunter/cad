---
id: splitting-arc-readers-take-no-body
kind: issue
title: "[topo.homes] arm 1: splitting's arc readers (pieced_arc, spiric_arc, spline_arc) and order take no Body and stay in an engine"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep`'s shape sweep (2026-11-03): files under `boolean/` and `splitting/` none of whose `fn` signatures (test modules aside) names `Body`, an arena key or a boolean record type.

- `crates/topo/src/splitting/pieced_arc.rs`, `spiric_arc.rs`, `spline_arc.rs`: an arc read against a region by its pieces (`PiecedArc`, `PieceBall`, `Ray`, `SpiricArc`, `Oval`). `boolean/torus_region.rs` reads them across the engines (`engine-leaves`' `boolean-names-splitting` line for it), which is the cross-engine shape `[topo.homes]` refuses.
- `crates/topo/src/splitting/order.rs`: `exact_band`, `in_plane_frame`, `lex_cmp`, `sort_indices_by_point`, `sort_along_line`, `sort_along`; its one engine type is `SplitPlane` (an origin and a unit normal).

Each is a reading of carriers or points, so arm 1 sends it to `geom-brep`; `ConicRows`/`ArcRows` in `splitting/containment.rs` would be checked by the same test when they go.

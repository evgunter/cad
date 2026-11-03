---
id: bound-side-reading-waits-on-link-metric
kind: issue
title: "[topo.homes] arm 1: bound_side's reading takes no Body but reads link::metric and link::Round, which live inside link.rs"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep` (2026-11-03), which was to move `bound_side`'s reading beside `geom_brep::enters_material` if it took no Body.

`crates/topo/src/bound_side.rs` names no `Body` and no arena key (`read`, `read_plane_side`, `Tie`, `BoundSupport`, `FaceBend`, `ConicArc`, `FaceDatum`, `sagitta_charge`, `rise`). But it decides through `crate::link::metric` (`Reading`, `Departure`, `Distance`, `settle`) and reads `crate::link::Round` (`FaceBend`'s round), both declared inside `crates/topo/src/link.rs`, a module that reads Bodies. `link.rs`'s own tests read its source text for `mod metric` (`body("\npub(crate) mod metric")` and the `Self(` census beside it), so `metric` has to leave `link.rs` with those guards re-pointed before `bound_side` can follow it to `geom-brep`. Order: `metric` (and `Round`) out of `link.rs` to `geom-brep`, then `bound_side`.

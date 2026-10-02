---
id: edge-side-surfaces-have-no-door
kind: issue
title: An edge's two side surfaces have no public door; the read is spelled at least nine times
status: dispatched
opened: 2026-10-02
priority: P3
cost: M
branch: tquery/edge-side-door
---


## What

`topo::query::rim_of` now defines a rim by an edge's two side surface
keys, but no public door answers "this edge's side surfaces" (or side
faces). The walk `he → parent_loop → face → surface` is spelled
privately wherever it is needed:

- `crates/topo/src/query.rs` `edge_sides` / `surface_across` (the rim
  door), and `face_kind_across` under `edge_adjacent_matches`.
- `crates/topo/src/replace_face.rs` `edge_faces` (`pub(crate)`), and
  `crates/topo/src/validate.rs` `edge_adjacent_faces`.
- `crates/sweep/src/blend/battery.rs` `edge_surfaces`, and
  `is_seam_vertex`'s parallel co-surface rule.
- `crates/sweep/src/test_support.rs` `arcs_at`'s `surface_of` closure.
- `demos/tour/src/bodies.rs` (the bud's mouth-seed finder) and
  `demos/tour/tests/common/rim_select.rs`: the tour reaches past the
  API for it.
- `crates/editor-core/src/names/emit_union.rs` `edge_faces` (faces, not
  surfaces); editor-core's selectors otherwise delegate to
  `topo::query` and spell no copy of their own.
- Test closures: `crates/sweep/tests/rim_of_rows.rs` (the bead row),
  `crates/topo/tests/r2_rim_probes.rs` (`query_face`),
  `crates/sweep/tests/rim_of_structural_review_probes.rs` (`surface`,
  `pair`).

## The shape to give

One read-back door (`readback::edge_sides` or a `query` twin) returning
both side faces or surfaces with a typed refusal, the private copies
routed through it, and the tour seed finder spelled on it.

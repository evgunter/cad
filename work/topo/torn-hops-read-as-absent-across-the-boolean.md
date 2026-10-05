---
id: torn-hops-read-as-absent-across-the-boolean
kind: issue
title: Record hops past a resolved face, half-edge or edge read a torn link as absent across boolean/
status: dispatched
opened: 2026-10-05
priority: P3
cost: M
refs: [torn-records-read-as-absent-in-the-rest-lane-and-the-split-gate, torn-body-refusal-families-beyond-the-six-doors]
---

## What

Found by the sweep of the PR that closed
`torn-records-read-as-absent-in-the-rest-lane-and-the-split-gate`: a
lookup through a key read out of a record this call resolved (a face's
surface or loops, a half-edge's start or edge, an edge's curve or
halves' faces) that answers its miss as absent — `None`, `continue`,
`false`, `Unread`, or a kind (`Nurbs`, not a plane, null scaffolding).

- `boolean/boxes.rs` `face_window_steps` (`?` on the face's loops, the
  walk, its half-edges and edges; `census.rs` passes its `None` on),
  `face_box` and `edge_box` (`get_curve_geom(..).and_then(certified)`:
  a torn curve reads as uncertified);
- `boolean/carrier_cross.rs` `boundary_crossing` (a half's vertex point
  skipped; a half's edge curve answers `BoundaryCrossing::Unread`);
- `boolean/finish.rs` `pinch_site`, `discarded`'s `on_face` and
  `edges_of` (walk members' starts skipped);
- `boolean/mod.rs` `build` (a declared face's surface leaves the side
  `Unread`), `tangent_struts` (face of a half, a face's surface, an
  edge's curve, two surfaces: each `continue`), `border_held`,
  `locus_through_plane_face`;
- `boolean/ops.rs` `describe_edges` (an edge's curve reads as none; a
  half's face as `false`), `sphere_extent_scan` (a face's surface reads
  as not NURBS / not a sphere);
- `boolean/recl.rs` `carrier_of`, `require_same` (`map_or(Nurbs, ..)`),
  `resolve_edge_edge` (a torn surface reads non-planar);
- `boolean/reduce.rs` `face_plane` (a torn surface reads as "not a
  plane"), `gate_maximal_faces`, `face_edges`, `edge_face_read`,
  `edge_covers`, `on_declared_shared_carrier` (`face_of_half_edge` +
  `flatten`), `parents_distinct_from` (`all(is_some_and)`),
  `boundary_meets_circle_only_at` (a torn curve answers `Ok(false)`);
- `boolean/rim_wedge.rs` `face_boundary_arcs` (loops, walk, halves,
  edges: `continue`; a torn curve reads as null);
- `boolean/solid_contain.rs` `wall_outline` (a torn curve refuses as a
  capability), `torus_chart_windows`, `sphere_chart_trim` (`continue`),
  `point_in_face` (a torn outer loop answers `Ok(Some(false))`);
- `boolean/surface_group.rs` `unmated_boundary` (a torn outer loop or
  curve answers `Ok(None)`);
- `boolean/vtxfac.rs` `classify_vertex_on_face` (a torn surface gives no
  curvature lever), the sector/contact `surface` closure (tangent read
  `false`), `pierced_kind` and the error-path `map_or(Nurbs, ..)`s;
- `boolean/zip.rs` `split_across` (`chart_of`, `outer` over faces read
  from loop records).

Unclear, to decide at the site: `finish.rs` `weld_pinches`' skip of a
welded vertex that no longer resolves; `ops.rs`
`declared_surface_pairs`' graft-map and result-surface reads;
`combine.rs` `graft_solids_impl`'s `let Some(Certified(c)) = src.curves.get(k) else continue`.

## Direction

As the closing PR did for `rest.rs`: a hop past a record resolved in
the same call goes through `live::linked` / `proven` /
`Body::face_surface_linked` / `edge_curve_linked` / `face_of_linked`,
and a site on a mid-operation body (the reduction's operands, the
graft) states the premise it panics on — a link every Euler operator
leaves resolving, never a fact read before a write (`insert.rs`
`orbit_step_at`'s P0). A key carried across the operation's own kills
stays typed. `reduce.rs` and `ops.rs` were under another lane's edit
when this was filed; take them after it lands.


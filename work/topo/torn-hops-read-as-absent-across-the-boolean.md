---
id: torn-hops-read-as-absent-across-the-boolean
kind: issue
title: Record hops past a resolved face, half-edge or edge read a torn link as absent across boolean/
status: open
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
  from loop records); since retired (PR 4139), its successor
  `split_cones` reads a loop's face through `linked`.

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


## 2026-10-05 — converted; the declared-pair and coincidence sites wait for D10

Every listed site that is not declared-pair, contact or coincidence
machinery now reads its hops through `live::linked` / `proven` /
`face_surface_linked` / `edge_curve_linked` / `face_of_linked` /
`faces_of_vertex_linked` / `vertex_orbit_linked` / `proven_mate`, and a
key the caller carries keeps its typed or `None` answer: `boxes.rs`
`face_window_steps` / `face_box` / `edge_box`, `carrier_cross.rs`
`boundary_crossing`, `finish.rs` `pinch_site` and `discarded`'s two
walks (`section_boundary`), `ops.rs` `describe_edges` and
`sphere_extent_scan`, `reduce.rs` `face_plane` / `gate_maximal_faces` /
`boundary_meets_circle_only_at`, `solid_contain.rs` `wall_outline` /
`torus_chart_windows` / `sphere_chart_trim` / `point_in_face`,
`surface_group.rs` `unmated_boundary`, `vtxfac.rs`
`classify_vertex_on_face`'s pierced surface and kind, `zip.rs`
`split_across` (since retired, PR 4139), `combine.rs` `graft_solids_impl`'s curve read, and the
sweep hit `sphere_region.rs` `sphere_face_region`. `weld_pinches`' skip
stays: it asks whether a pierce copy survived the carve's kills.

**Left for after D10** (the HOLD of PR 3990), because each exists to
read a declaration or a coincidence: `mod.rs` `build`'s declared-face
side, `tangent_struts` (it feeds `DeclaredPairs::build`),
`border_held` (held edges of covered pairs) and
`locus_through_plane_face` (`verify_tangent_declaration`); `ops.rs`
`declared_surface_pairs`; `recl.rs` `carrier_of` / `require_same` (the
carrier-identity ladder) and `resolve_edge_edge`'s flank-sense
coincidence arm; `reduce.rs` `face_edges` (the undeclared-coincidence
scan), `edge_face_read`, `edge_covers`, `on_declared_shared_carrier`
and `parents_distinct_from` (the carrier-distinctness ladder);
`rim_wedge.rs` `face_boundary_arcs`, whose every caller is the
declared seam or cover machinery; and `vtxfac.rs`' sector/contact
`surface` closure and the two conformal-lump `map_or(Nurbs, ..)`s.
This row stays open for them.

## 2026-10-05 — the HOLD criterion, stated

What stays for D10 is a site whose **own logic reads a declaration or a
coincidence**. A generic geometric helper whose only callers are
declared or coincidence arms has no such read of its own, so it is
converted. Two converted sites are helpers of that kind:
`carrier_cross.rs` `boundary_crossing` (its one caller, `reduce.rs`
`interior`, is reached only in `edge_covers`' covered arms past
`on_declared_shared_carrier`) and `reduce.rs`
`boundary_meets_circle_only_at` (its one caller, `lying_on`, is reached
only past `parents_distinct_from` on the `LiesOn` arm). Measured against
the same criterion, `rim_wedge.rs` `face_boundary_arcs` is a helper as
well, so it is converted now and comes off the held list.
`mod.rs` `locus_through_plane_face` stays held, because it is part of
`verify_tangent_declaration` and reads the declared pair's plane face.

A face's outer-then-rings loop links now have one home,
`Body::face_loops_linked`, whose field names are `outer` and `rings`.
Two sites still spell the iteration by hand, because they sit in files
under another open PR: `boolean/rest.rs` `face_witnesses`
(PR 4067) and `attach.rs` `check_moved_boundary` (PR 4060). Each moves to the helper
once its PR lands.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: every held site reads a declaration or coincidence (DeclaredPairs, carrier identity/distinctness ladders, undeclared scan, verify_tangent_declaration); the stage-4 door rewrites them. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E deletes one held site, `reduce.rs` `face_edges` (the undeclared-coincidence scan). Every other held site is still there:

- `mod.rs` `tangent_struts`, `border_held` and `locus_through_plane_face` (`crates/topo/src/boolean/mod.rs:1546`, `:5054`, `:5875`);
- `ops.rs` `declared_surface_pairs` (`crates/topo/src/boolean/ops.rs:3592`);
- `recl.rs` `carrier_of` and `require_same` (`crates/topo/src/boolean/recl.rs:53`, `:81`);
- `reduce.rs` `edge_face_read`, `edge_covers`, `on_declared_shared_carrier` and `parents_distinct_from` (`crates/topo/src/boolean/reduce.rs:723`, `:1569`, `:2435`, `:2870`).

The hold no longer applies to the coincidence sites. Under E they read the glue door's pairs, declared or not, and D10 keeps that machinery, so `recl`'s ladder, `edge_covers`, `parents_distinct_from` and `edge_face_read` can be converted now. The declared-pair-only sites (`build`'s declared-face side, `tangent_struts`, `declared_surface_pairs`) may instead leave with `declared-pairs-retire` (F), and need no conversion if they do.

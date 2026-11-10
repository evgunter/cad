---
id: lane-prefixed-k-names-outside-their-lane
kind: issue
title: "[topo.homes] K names that carry a caller's lane prefix are decided outside that lane: split_ring_*, bool_pierce_normal_*, bool_ring_run_winding, bool_chord_side, bool_curved_contain_carrier"
status: open
opened: 2026-11-03
---


Found in PR 6488's review (R2, style item 7). `[topo.homes]` (`crates/topo/README.md`) says a K name's prefix is the module that decides the margin, never a caller's lane. These names carry an operation's prefix (`split_`, `bool_`) but are decided in a crate-root module that is not that operation's:

- `crates/topo/src/ring_path.rs`, 27 names under `split_ring_path_*` and `split_ring_torus_*`, among them `split_ring_path_off_apex` (~:245), `split_ring_path_span` (~:299), `split_ring_path_torus_loop` (~:998), `split_ring_torus_meridian` (~:1108), `split_ring_torus_parallel` (~:1124), `split_ring_torus_crest` (~:1143), `split_ring_path_crest_graze` (~:1305), `split_ring_path_in_span` (~:1656), `split_ring_path_meets_plane` (~:1841). Full list: `grep -noE '"split_ring_(path|torus)_[a-z0-9_]+"' crates/topo/src/ring_path.rs`.
- `crates/topo/src/face_normal.rs`, `bool_pierce_normal_on_chart` (~:374), `bool_pierce_normal_on_cone` (~:391), `bool_pierce_normal_on_spline` (~:556).
- `crates/topo/src/loop_winding.rs`, `bool_ring_run_winding` (~:105).
- `crates/topo/src/bound_side.rs`, `bool_chord_side` (~:618, ~:1178).
- `bool_curved_contain_carrier`, decided in `crates/topo/src/contain.rs` (since PR 6775 moved it to the crate root) and asserted on the validator's path (`crates/topo/src/validate.rs` ~:12634): a `bool_` name the validator also reaches.

**The class:** a K prefix used as a module category (a lane's name) goes stale on every move. Each time a reading leaves an operation's directory for a crate-root module or for `geom-brep`, its names keep the old lane's prefix unless someone renames them, and every reader keyed on prefixes (`crates/sweep/tests/common/crossing.rs`, the K report's grouping) misfiles them. PR 6488 renamed 33 such names for the doors it moved and left these.

**Should K names carry a lane prefix at all?** The finding here is that they should not: a name's prefix should be the deciding module's, as `[topo.homes]` already says, and a reader that wants "the Boolean's pipeline" should read a roster held against the code (as `geom_brep::curve_face::DOOR_K_PREFIXES` is), not a lane prefix. Renaming each family is a census note per `docs/K-REPORT.md`'s runbook (one site, one population, no pooling), with `crates/topo/src/boolean/mod.rs`'s subject map and any pin updated in the same change.

## More evidence: the containment component at the root (PR 6775, 2026-11-10)

PR 6775 moved boolean's containment component to the crate root as a pure
move, so its K names now sit in root modules, each still under the
Boolean's `bool_` prefix. The renames are out of a move's scope and are
this row's. By module, from `grep -oE '"bool_[a-z0-9_]+"'` over each
file's production code:

- `contain.rs`: `bool_contact_arc_end_vertex`, `bool_contact_vertex`,
  `bool_curved_contain_carrier`, `bool_face_disc_carrier`.
- `sphere_region.rs`: `bool_sphere_region_{arm, at, component, corner, cross, order, parallel}`.
- `torus_region.rs`: `bool_torus_frame_radius`, `bool_torus_region_{at, component, corner, cross, order}`.
- `ruled_region.rs`: `bool_ruled_region_{at, component, corner, cross, end, frame, nappe, order}`.
- `spline_region.rs`: `bool_spline_region_{arm, carrier, trim}`.
- `solid_contain.rs`: `bool_cone_partial_reach`, `bool_point_axis`,
  `bool_point_in_solid_{denom, infinity, infinity_enclosure, plane, sphere}`,
  `bool_point_past_every_reach`, `bool_ray_cone_{disc, incidence, lead, nappe}`,
  `bool_ray_cylinder_disc`, `bool_ray_sphere_disc`, `bool_ray_torus_incidence`,
  `bool_sphere_iso_{meridian, rim}`,
  `bool_sphere_trim_{antipode, latitude, period, pole_end, pole_interior}`,
  `bool_wall_outline_reach`, `bool_wall_turn_true`.
- `surface_group.rs`: `bool_wrap_rim`.

That is 53 names in seven root modules. The `bool_ray_torus_*` ladder rows
named through `RAY_TORUS_ROWS`, and the `RayRows` schedules, are read
through consts, not literals, so the grep does not list them.

---
id: track-caller-fns-panic-inside-closures-in-topo-and-sweep
kind: issue
title: #[track_caller] fns in topo and sweep panic, or call a #[track_caller] helper, from inside a closure, so the panic reports the fn's own line
status: open
opened: 2026-10-25
---


`wire-emithelpers` found these while widening the closure sweep for PR 5530 (2026-10-25). A panic inside a closure, including a call of a `#[track_caller]` helper or an `.expect` from inside one, reports the closure's line, which sits inside the fn. So the fn's `#[track_caller]` buys nothing. PR 5503's review named the shape. The repair is a `let … else` or a `match` outside the closure.

**Ownership.** The hits span several programs (`work.py territory`):
- carve and carvetail, for `sweep/src/revolve/*`;
- carve, carverest and strut, for `sweep/src/swept.rs`;
- topotail, for `topo/src/body.rs`;
- cleave and hone, for `topo/src/boolean/combine.rs`;
- topo, for `euler`, `merge_faces`, `movefac` and `revert`;
- origin, for `live.rs`;
- charttail, for `pcurves.rs`;
- shelf and shell, for `shell.rs`.

So this row sits in `work/issues/`. Split it per program when one takes it.

**The sweep.** It is a heuristic script over each `#[track_caller]` fn body. It matches a closure head (`|…|` or `||`), then, inside that closure's extent, any of:
- `unreachable!`, `panic!` or `assert*!`;
- `.expect(` or `.unwrap(`;
- a call of any fn that is `#[track_caller]` anywhere in `crates/`, matched by name.

**Its false positives and blind spots:**
- a method call that shares a name with some `#[track_caller]` fn is a false positive;
- a closure head whose lookbehind the script does not anticipate is a blind spot;
- a panic two calls deep, in a fn that is not `#[track_caller]`, is a blind spot.

So check each hit by eye before you convert it.

The hits, as `file line fn match`:
```
crates/sweep/src/revolve/full.rs 888 twin_wall unreachable!
crates/sweep/src/revolve/partial.rs 362 he_start unreachable!
crates/sweep/src/revolve/partial.rs 379 he_edge unreachable!
crates/sweep/src/swept.rs 1363 face_surface_key unreachable!
crates/sweep/tests/common/stations.rs 143 construction_state panic!
crates/topo/src/body.rs 1568 point_of dangling_link(
crates/topo/src/boolean/combine.rs 151 image dangling_link(
crates/topo/src/boolean/combine.rs 159 fresh unreachable!
crates/topo/src/euler.rs 4063 rebased_endpoints linked(
crates/topo/src/euler.rs 4071 rebased_endpoints linked_vertex_point(
crates/topo/src/euler.rs 4589 site_cycle_from unreachable!
crates/topo/src/euler.rs 4846 proven_mate mate_of(
crates/topo/src/live.rs 185 answer dangling_link(
crates/topo/src/live.rs 190 answer_geometry dangling_link(
crates/topo/src/live.rs 209 answer unproven(
crates/topo/src/live.rs 214 answer_geometry unproven(
crates/topo/src/live.rs 286 proven unproven(
crates/topo/src/live.rs 332 resolve_vertex_point answer_geometry(
crates/topo/src/live.rs 341 face_surface_linked dangling_link(
crates/topo/src/live.rs 353 edge_curve_linked dangling_link(
crates/topo/src/live.rs 533 loop_cycle_live of(
crates/topo/src/live.rs 534 loop_cycle_live unreachable!
crates/topo/src/merge_faces.rs 2767 boundary_points linked_vertex_point(
crates/topo/src/movefac.rs 864 assert_panics_torn .unwrap(
crates/topo/src/offset_nappe.rs 237 corner_stations linked_vertex_point(
crates/topo/src/pcurves.rs 853 edge_vertex_points linked(
crates/topo/src/pcurves.rs 861 edge_vertex_points linked_vertex_point(
crates/topo/src/pcurves.rs 1998 open_loops held_open(
crates/topo/src/pcurves.rs 2217 face_loop_walks loop_rows(
crates/topo/src/pcurves.rs 2439 split_cache unreachable!
crates/topo/src/pcurves.rs 2509 split_cache is_plus(
crates/topo/src/pcurves.rs 2894 derive_face unreachable!
crates/topo/src/pcurves.rs 3323 releases_a_gap held_open(
crates/topo/src/pcurves.rs 3457 site_rows half_edge_carrier(
crates/topo/src/pcurves.rs 3458 site_rows unreachable!
crates/topo/src/pcurves.rs 3462 site_rows entry_vertex(
crates/topo/src/pcurves.rs 3462 site_rows is_plus(
crates/topo/src/pcurves.rs 3466 site_rows unreachable!
crates/topo/src/pcurves.rs 3477 site_rows half_edge_record(
crates/topo/src/pcurves.rs 3479 site_rows unreachable!
crates/topo/src/pcurves.rs 3487 site_rows unreachable!
crates/topo/src/pcurves.rs 3495 site_rows is_plus(
crates/topo/src/pcurves.rs 3496 site_rows entry_vertex(
crates/topo/src/pcurves.rs 3525 site_rows half_edge_record(
crates/topo/src/pcurves.rs 3581 site_rows unreachable!
crates/topo/src/pcurves.rs 3590 site_rows unreachable!
crates/topo/src/pcurves.rs 5054 validate_pcurves is_plus(
crates/topo/src/pcurves.rs 5061 validate_pcurves derive_image(
crates/topo/src/revert.rs 197 reverted of(
crates/topo/src/revert.rs 203 reverted face_surface_linked(
crates/topo/src/revert.rs 215 reverted dangling_link(
crates/topo/src/revert.rs 229 reverted face_of_linked(
crates/topo/src/sector_face.rs 167 resolve point_of(
crates/topo/src/sector_face.rs 169 resolve proven(
crates/topo/src/shell.rs 2249 ring_rows unreachable!
crates/topo/src/shell.rs 3491 split_cycle unreachable!
crates/topo/src/shell.rs 3758 offending_face face_of_linked(
crates/topo/src/shell.rs 4091 planar_faces unreachable!
crates/topo/src/shell.rs 4777 moved_walls proven(
crates/topo/src/shell.rs 4778 moved_walls linked_vertex_point(
crates/topo/src/shell.rs 4887 moved_walls unreachable!
```

---
id: mfkrh-leaves-an-orphan-surface-under-the-per-op-scalpel
kind: issue
title: topo: mfkrh leaves an orphan surface, so ten sweep rows panic under topo/per-op-postcondition
status: open
opened: 2026-10-09
priority: P2
cost: M
---



(Filed by the ENCL orchestrator from the full review of PR 4433. It is pre-existing: it reproduces on `origin/main` `9409cb5e43`, and PR 4433 does not cause it.)

## What

With `topo`'s `per-op-postcondition` feature on (the scalpel), ten sweep rows panic in about 0.2 s:

```
crates/topo/src/surgery.rs:357:9: mfkrh postcondition: result is not tier-1 valid (kernel bug)
  left: Err([OrphanGeometry { geometry: Surface(SurfaceKey(1v3)) }])
```

The rows:
- `a_plane_across_a_one_face_wall::*` (7);
- `germ_coplanar_conic::every_op_refuses_or_answers_its_closed_form`;
- `one_segment_loop::a_boolean_on_an_extruded_seam_wall_builds_along_and_across_it`;
- `run_walls_built::fully_revolved_arc_runs_build_one_wall_each`.

Reproduce: `cargo nextest run -p topo -p sweep -p editor-core --all-features -E 'test(a_plane_across_a_one_face_wall) | test(germ_coplanar_conic) | test(one_segment_loop) | test(run_walls_built)'`. With `-p sweep --all-features` alone the rows pass, because feature unification does not turn on topo's scalpel. It is deterministic, not load.

So `mfkrh` leaves a surface no face references, inside a surgery scope, on the paths these sweep rows drive. It is a tier-1 invariant break that the per-op scalpel catches and the end-of-op check does not.

## More rows (2026-10-11)

A local `cargo nextest run --profile ci --all-features` over geom-core, geom-brep, topo, sweep, step-import and test-utils, on PR 4675's branch at a head before `7dc0fdcfe8`, panicked at the same `surgery.rs` `mfkrh` postcondition (`OrphanGeometry { geometry: Surface(..) }`) in fourteen sweep rows, which all pass without `--all-features`. None of them calls the analytic rung-3 certificate that PR changes. Names read off that run's log, not re-run:
- `a_plane_across_a_one_face_wall::` `a_blind_pocket_through_a_plates_top_face`, `a_box_across_a_full_revolved_tube`, `a_circle_edge_in_the_partners_face_refuses`, `a_one_site_loop_beside_matched_ones`, `a_plane_close_by_the_wrap_edges_end`, `a_slab_across_a_one_segment_cylinder` (6);
- `an_annular_tube_through_a_plate::` `a_plug_flush_in_a_bore_is_sided_by_its_caps`, `an_annulus_through_the_plate_builds`, `an_off_centre_annulus_builds`, `the_annulus_builds_at_every_witness_azimuth_pair`, `two_nested_annuli_build` (5, not in the list above);
- `germ_coplanar_conic::every_op_refuses_or_answers_its_closed_form`;
- `one_segment_loop::a_boolean_on_an_extruded_seam_wall_builds_along_and_across_it`;
- `run_walls_built::fully_revolved_arc_runs_build_one_wall_each`.

`topo::all offset_together::scope_walks::the_door_panics_on_an_out_of_scope_malformed_solid` hit the same panic and passed, being a `should_panic` row. The same run also failed one predicate-count row, `topo::all rim_dim_review_probes::silent_fixed_predicates_scale_linearly` (`rim_dim_review_probes.rs:168`, "undeclared flush must refuse"), which may be a different cause; it was not investigated.

## Still on main (2026-10-12)

The same fourteen sweep rows, and no others, fail under `--all-features` on main `a1a1bcec43` (`cargo nextest run --profile ci -p geom-brep -p topo -p sweep -p step-import --all-features`; PCERT, PR 4846).

## Related

- `work/ciw/no-ci-row-runs-topo-under-the-per-op-postcondition-scalpel.md`: no CI row runs the scalpel, which is why this is invisible to CI.
- The closed `tests-fail-under-the-per-op-postcondition-feature` covered different rows.

## Repair shape

Find the `mfkrh` call on these paths that ends with the face's surface unreferenced. Either the surface should be removed in the same op, or the op's precondition should forbid the state. Then the rows pass under the scalpel; the ciw row would keep them there.

**Wider than ten rows** (seen 2026-10-18 at `683f44c3c0`'s parent, `cargo nextest run --workspace --all-features` locally, while landing PR #5267, which touches no crate below editor-core): 34 sweep rows panic at `crates/topo/src/surgery.rs:357` with this item's message, `mfkrh postcondition: result is not tier-1 valid (kernel bug)`, `OrphanGeometry { Surface }`. Beside the ten listed: `a_closed_conic_in_the_partners_face::*` (5), `an_annular_tube_through_a_plate::*` (8), `germ_plane_torus_chord::*` (6), `germ_plane_torus_frame::*` (2), `germ_plane_torus_spiric::*` (1), `pi_seam_and_kiss_through_the_boolean::*` (1) and `section_end_poses::a_torus_section_read_square_refuses_as_the_inputs_and_its_lever_builds` (1). CI, which does not build the workspace with `--all-features`, passes them all.

## Still on main (2026-10-28, GREEN)

A hosted run of `cargo nextest run --profile ci --workspace --exclude
viewer --all-features` on `origin/main` `4beeeb05` (run `40720990736`, job
`122931500237`) ran 14 009 tests in 116 s. 24 failed. Twenty-one of those are
this item: `surgery.rs:385`, `mfkrh postcondition`, `OrphanGeometry {
Surface(1v3) }`. By file:

- `a_closed_conic_in_the_partners_face::*` (6)
- `a_plane_across_a_one_face_wall::*` (6)
- `an_annular_tube_through_a_plate::*` (4)
- `germ_coplanar_conic`, `germ_plane_torus_chord`, `one_segment_loop`,
  `pi_seam_and_kiss_through_the_boolean` and `section_end_poses` (1 each)

The nightly `--all-features` row that GREEN adds holds
`topo/per-op-postcondition` out until this item closes. The workflow
comment there names this file, so whoever closes it should drop the
feature from that row's `HELD_OUT_FEATURES` in `nightly.yml`.

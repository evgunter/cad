---
id: the-ci-slow-set-names-tests-that-no-longer-exist
kind: issue
title: The ci profile's slow-set filter names editor-core tests that no longer exist, so their successors run in the per-PR gate
status: closed
opened: 2026-10-13
closed: 2026-10-15
priority: P3
cost: M
branch: ciw/slow-set-names
---


## What

Found on branch `emit/glued-rim-build` while adding a slow test.
`.config/nextest.toml`'s `ci` profile excludes the slow set by exact
name (`test(=suite::name)`). Nothing checks that a name still matches a
test, so a rename leaves a dead entry, and the renamed test runs in the
per-PR gate.

Measured against `cargo nextest list -p editor-core --test all` (default
features), over the `binary_id(editor-core::all)` clause only. Four
entries matched nothing:
- `emit_union_member_order::no_name_rebinds_across_the_member_orders_of_a_cut_seam_union`.
  The test is now
  `every_member_order_of_a_cut_seam_union_publishes_one_table` (about
  15 s). That branch corrected this entry.
- `emit_nested_union_flat::a_nested_unions_merged_face_divided_in_the_fold_is_never_read_as_consumed`
- `coincidence_door::a_declared_rest_is_one_unproven_row_named_by_its_operands`
- `glue_on_zero_rows::an_undeclared_flush_rest_glues_and_records_one_unproven_row`

`grep` finds no `fn` of these last three names under
`crates/editor-core`.

## Not checked

- The other binaries' clauses (`editor-core` lib, `geom`, `geom-core`,
  `geom-brep`, …).
- Names gated behind a feature the list was not built with.
- Whether each dead entry's successor still takes 1 s or more.

## Owed

- A gate (`scripts/gates/`) that fails when a slow-set entry matches no
  listed test.
- Re-homing or dropping the three entries above.

## Closed

Audited every exact-name entry of the `ci` filter (555, over 15 binary
ids) against `cargo nextest list --lib --test all` of the nine packages
the filter names, default features; every live entry resolved there, so
no entry depends on a feature. Seven distinct names were dead, two of
them listed twice:

- `editor-core::all` `emit_nested_union_flat::a_nested_unions_merged_face_divided_in_the_fold_is_never_read_as_consumed`:
  deleted outright (e305ed3c80), no successor. Dropped.
- `editor-core::all` `coincidence_door::a_declared_rest_is_one_unproven_row_named_by_its_operands`:
  renamed `a_glued_rest_is_one_unproven_row_named_by_its_operands`
  (60b58da54b). Re-pointed.
- `editor-core::all` `glue_on_zero_rows::an_undeclared_flush_rest_glues_and_records_one_unproven_row`:
  renamed `a_flush_rest_glues_and_records_one_unproven_row` (60b58da54b).
  Re-pointed.
- `topo::all` `three_solids_on_one_line::pierces_a_weld_leaves_apart_build_one_body_in_every_member_order`:
  renamed `pierces_of_one_point_build_one_body_in_every_member_order`
  (e564d25b23). Re-pointed.
- `topo::all` `sphere_face_flips_are_refused::every_sphere_face_flip_is_refused_and_named`:
  the test lives in `crates/sweep/tests`, so the entry named the wrong
  binary. Moved under `sweep::all`.
- `sweep::all` `germ_plane_torus_frame::a_spiric_pose_keeps_the_chords_typed_refusal`
  (twice): split into `germ_plane_torus_spiric`'s rows (dade619dcd),
  which are in the slow set already. Dropped.
- `sweep::all` `germ_plane_torus_frame::two_one_site_loops_on_one_face_pair_reach_the_dispatch`
  (twice): renamed `..._pass_the_dispatch` (3d74703a34), which is in the
  slow set already. Dropped.

The check is `scripts/check-slow-set-names.py`, run as a ci.yml `test`
step over a `cargo nextest list` of the packages that job already built
(so a rename fails the PR that does it), and over the whole workspace in
the nightly's `full suite` job; its selftest runs in `lint`.


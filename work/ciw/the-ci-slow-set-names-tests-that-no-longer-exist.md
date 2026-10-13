---
id: the-ci-slow-set-names-tests-that-no-longer-exist
kind: issue
title: The ci profile's slow-set filter names editor-core tests that no longer exist, so their successors run in the per-PR gate
status: open
opened: 2026-10-13
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

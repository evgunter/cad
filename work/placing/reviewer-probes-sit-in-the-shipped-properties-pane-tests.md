---
id: reviewer-probes-sit-in-the-shipped-properties-pane-tests
kind: issue
title: Reviewer probes from old PRs sit in the shipped properties_pane_tests, some ignored and asserting nothing
status: closed
pr: 6938
branch: placing/p3p4-cleanup
opened: 2026-11-14
priority: P4
cost: E
closed: 2026-11-14
---


Found by the review of PR 6916 (its S10), filed by `placing/editor-preview`.

`crates/viewer/src/app.rs` `properties_pane_tests` carries reviewer
probes of earlier PRs merged into the shipped test module:

- `review_6295_e2e_remove_a_filleted_hole`;
- `mod review_6144` (its rows, e.g. `a_driven_step_the_edit_deletes`,
  `driven_step1_remove_step3`, `control_remove_a_step_no_driver`,
  `e2e_land_then_drag_then_release_then_undo`);
- `mod review_6299`: `r1_lone_profile_at_zooms`,
  `r2_tie_on_curved_and_straight_rims` and
  `r6_tools_get_no_profile_unless_any` are `#[ignore = "reviewer probe
  (PR 6299): prints what it saw and asserts nothing"]`;
  `r3_profile_behind_the_body_is_not_picked`, `r4_overlapping_profiles`,
  `r5_currency_undo_edit_replace`, `r8_mate_kinds_and_a_profile_on_a_face`,
  `r9_startup_rim_from_above`, `r10_far_end_and_silhouette_offsets`,
  `r7_e2e_second_profile_on_the_top_face` run.

`docs/prompts/implementer-discipline.md` §8: a test that asserts nothing is
never a gate; drop it, or `#[ignore]` it with its run command.

**Owed.** Each probe either becomes an ordinary named row whose assertion
a bug could break (renamed for what it guards, out of the `review_*`
modules), or is deleted; the asserting-nothing ones are deleted. **Done
when** no `review_*` module or `#[ignore]`d reviewer probe remains in
`properties_pane_tests`.

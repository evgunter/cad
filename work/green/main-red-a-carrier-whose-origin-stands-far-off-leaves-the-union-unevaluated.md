---
id: main-red-a-carrier-whose-origin-stands-far-off-leaves-the-union-unevaluated
kind: issue
title: main red (eps 1e-12): union_fold_face_face_touch's far-off-origin row refuses VolumeUncomputable at back 900; the Unevaluated read at the default eps is gone
status: open
opened: 2026-11-09
priority: P1
cost: M
parent: nightly-reds-the-1e-12-rows
---

Nightly run 42567046151 (sha `836426e07e`), both slow-set rows: `crates/editor-core/tests/union_fold_face_face_touch.rs` `a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line` panics in `test_support.rs` with `the body has a value: Unevaluated { node: … ordinal: 47 … }`.

It was green in the 11-07 nightly (run 42342466395) and is already red at `7cbccda83d`, the first parent of BOXES' PR 6579. So it arrived in the window after the 11-07 nightly (`45cae9d7a4`) and before PR 6579 (`3b7794bb5d^1` = `7cbccda83d`), whose code merges are PRs 6371 (`onehome/finished-body`), 6578 (`placing/place-live`), 6380 (`onehome/run-owns-rounding`) and 6586 (`onehome/fix-moved-row-cites`). Not BOXES'. That window also brought the session box's `OutsideSessionBox` gate (PR 6380, `work/onehome/main-red-four-sweep-slow-rows-refuse-outside-session-box.md`), so read the node's refusal first.

(BOXES nightly-reds triage, 2026-11-09)

## What fails now (GREEN, 2026-11-14)

Nightly 43371638371 (main `e36675c1`, 2026-11-13), from each row's junit: the test passes at the default ε and at 1e-6. It fails at 1e-12 only, with `back 900: plate, rod, block: refused: Boolean(ResultInvalid { NotFinished { VolumeUncomputable … Escalated`. So the `Unevaluated` read above is fixed or moved, and what stands is a different refusal at a different ε. Five sweep rows fail at 1e-12 in that run with the same `VolumeUncomputable … Escalated { Indeterminate` shape (`full_turn_wall::both_doors_answer_the_bead_by_its_height_window`, both `near_circular_sections` rows, `parallel_cylinder_join::a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings`, `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`), so read them as one move before bisecting this one alone.

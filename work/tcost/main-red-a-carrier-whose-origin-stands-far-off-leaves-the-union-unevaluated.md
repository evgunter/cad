---
id: main-red-a-carrier-whose-origin-stands-far-off-leaves-the-union-unevaluated
kind: issue
title: main red (slow set): union_fold_face_face_touch's far-off-origin row reads an Unevaluated body, since the 11-08 onehome merges
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), both slow-set rows: `crates/editor-core/tests/union_fold_face_face_touch.rs` `a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line` panics in `test_support.rs` with `the body has a value: Unevaluated { node: … ordinal: 47 … }`.

It was green in the 11-07 nightly (run 42342466395) and is already red at `7cbccda83d`, the first parent of BOXES' PR 6579. So it arrived in the window after the 11-07 nightly (`45cae9d7a4`) and before PR 6579 (`3b7794bb5d^1` = `7cbccda83d`), whose code merges are PRs 6371 (`onehome/finished-body`), 6578 (`placing/place-live`), 6380 (`onehome/run-owns-rounding`) and 6586 (`onehome/fix-moved-row-cites`). Not BOXES'. That window also brought the session box's `OutsideSessionBox` gate (PR 6380, `work/onehome/main-red-four-sweep-slow-rows-refuse-outside-session-box.md`), so read the node's refusal first.

(BOXES nightly-reds triage, 2026-11-09)

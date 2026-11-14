---
id: main-red-two-door-at-rest-rows-on-the-2026-11-11-nightly
kind: issue
title: Main red: two viewer door_at_rest slow rows fail on the 2026-11-11 nightly (viewer app feature job)
status: open
opened: 2026-11-12
priority: P1
cost: E
---


Found by PLACING's `placing-target-row` lane (PR 6850), which saw them fail locally with the same assertions. Its diff does not touch the code under test.

- `crates/viewer/tests/door_at_rest.rs:78`, `a_landings_door_reads_pending_until_its_own_answer_settles`
- `crates/viewer/tests/door_at_rest.rs`, `the_threaded_door_seam_answers_the_newest_landing`

Both are in the slow set, and both fail on the 2026-11-11 nightly (viewer app feature job). Filed on COINCIDE because `no-row-cancels-the-door-between-replay-and-discharge` owns the threaded door seam's pins. This is not yet bisected.

**Done when** both rows are green on main's nightly, or each is re-pinned with its cause stated.

## Re-pinned, with the cause (GREEN, PR 6931, 2026-11-14)

GREEN's default-eps lane (`green/default-eps`) reproduced both on main
`b363f5fa` at the default ε: each asserts 3 `unproven-coincidence`
findings on `die_composed` and main reports 2.

- **Cause.** PR 6710 (`onecopy/edge-ends-turns`, merge `75a84e844`): both
  rows are green at its first parent and red at the merge, run on the
  unchanged test. The die's second blend joint proves since then
  (`editor-core`'s `the_door_proves_half_the_composed_dies_joints`, which
  PR 6771 re-blessed from one proved joint to two); the two findings left
  are the booleans' `Meet` rows. So it is not the threaded door seam's
  pins that moved, and
  `no-row-cancels-the-door-between-replay-and-discharge` is not involved.
- **Re-pin.** `door_at_rest.rs`'s `DIE_UNPROVEN` is 2, its doc saying
  which rows they are.
- **Not yet shown.** The lane ran both at ε default, 1e-6 and 1e-12 at
  default features. The nightly's `viewer app feature` job, where this
  row saw them, was not reproduced locally and no job on the PR runs the
  pair under `--features app`; the first nightly after the merge is what
  shows that job past them. The row is left open for its owner to close
  on that.
- The count is a hand-kept copy and will go stale again:
  `work/tint/the-viewers-door-at-rest-rows-keep-by-hand-a-count-editor-core-owns.md`.

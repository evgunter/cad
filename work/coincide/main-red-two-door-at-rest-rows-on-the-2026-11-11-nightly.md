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

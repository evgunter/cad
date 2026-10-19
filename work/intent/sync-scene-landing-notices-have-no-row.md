---
id: sync-scene-landing-notices-have-no-row
kind: issue
title: viewer: no row drives sync_scene, so a landing's withdrawal notices are unpinned
status: closed
opened: 2026-10-18
priority: P3
cost: E
closed: 2026-10-18
---

Found by the sequential review of PR 5296 (F5, by grep).

`App::sync_scene` (`crates/viewer/src/app.rs`, just after `pump()`) drains
`DocSession::take_landed_withdrawals` into `self.notices` through
`frame::withdrawal_notices`. No test drives `sync_scene`, so those lines are
unpinned: deleting them leaves every row green, and a landing's prune (a
free-move frame a world pin now pins, or a gesture whose space joined another)
would then reach no reader. `frame_policy.rs` calls `withdrawal_notices`
directly instead.

The fix is a row in app.rs's own tests (the `driven` harness), shaped like the
asynchronous app: commit a probe on a loose copy, commit a mate that pins it,
let the landing arrive through `sync_scene`, and assert that the notice reaches
`notices`.

## Built

`app::properties_pane_tests::a_landings_withdrawal_reaches_the_status_line`
drives the real app frame by frame. It authors a part file and an assembly
with one instance, takes the instance's placement off, and commits a free-move
on the loose copy. It then undoes the removal, which pins the copy again.
Through that edit the frame still holds; the landing drops it, in a later
frame, and the `InTheWorld` withdrawal is on the status line. Removing the
`sync_scene` lines that extend `notices` turns the row red.

Review tier: single STYLE review. The unit adds one test and no production
code.

The style review gave APPROVE-WITH-FIXES with no MAJOR, and all of its fixes
are taken:
- the doc says which frame shows the notice;
- the idle frame is gone;
- the row asserts that the undo's own outcome says nothing about the frame;
- the part-workspace prelude both `Driven` rows used is `part_workspace`.

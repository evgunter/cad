---
id: sync-scene-landing-notices-have-no-row
kind: issue
title: viewer: no row drives sync_scene, so a landing's withdrawal notices are unpinned
status: open
opened: 2026-10-18
priority: P3
cost: E
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

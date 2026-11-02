---
id: an-edit-whose-inner-run-stopped-still-commits
kind: issue
title: An edit whose inner evaluation was canceled or ran out of budget still commits, reading the prefix as an unread face
status: open
opened: 2026-11-02
---

Found building `kernel-run-evaluations-mint-their-own-cancel-token` (PR 2 of WIRE's budget work). The edit doors now run their inner evaluations on the caller's token, but none of them refuses when that run stops short:

- `edit.rs` `Recording::nearest_to_face` (called by `sketch_on_face` with no `orient`): a run canceled or out of member budget has no value for the face, so the orientation falls back to the identity and the sketch commits with it.
- `edit.rs` `Recording::rebundle_by_geometry` (`constrain`, `remove_mate`): a stopped `mate::solve_document` reports no `NoChart` fault, so the structural bundle commits.
- `take_over.rs` `Recording::take_over`: a stopped run leaves the copy's names unchecked; the rounds end, and the takeover commits without the rebinds a full run would write.
- `mate/corner.rs` `corner_frame`: refuses through the node's standing (`NotEvaluated`), which names both stops — typed, but as a pin refusal rather than as the run stopping.

The fix is a typed refusal (an `EditError` arm carrying `crate::Stopped`) where an edit's inner run stopped, so a canceled gesture commits nothing.

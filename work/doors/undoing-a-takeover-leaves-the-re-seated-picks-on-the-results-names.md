---
id: undoing-a-takeover-leaves-the-re-seated-picks-on-the-results-names
kind: issue
title: Undoing a takeover restores the document, but the selection and the latched faces keep the result's names and read stale
status: open
opened: 2026-11-01
priority: P3
cost: E
---

Found by the takeover unit (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`). A takeover re-seats the viewer's held picks on the result's names once its run lands (`DocSession::land` completes the session's pending takeovers and re-seats the selection; `App::reseat_held` re-seats the forms' latched faces and the open tool's picks, `crates/viewer/src/app.rs`). An undo before the run lands drops the pending takeover, so the picks never move. An undo after it lands restores the document (`crates/viewer/tests/takeover_reseats.rs`, `a_mate_on_the_taken_copys_face_stays_seated`), but the held picks keep the result's names, which the restored copy no longer holds. They then read stale (`Resolution::Failed`, a typed stale reading, never a wrong face) until re-picked.

The re-seat is not in the history: a history step is the edits alone (`History::commit_group`, `crates/viewer/src/history.rs`). An undo that also restores the picks needs the inverse map kept beside the step, or the undo door to re-seat by the step's `Rebind` edits read backwards (which covers only names the document itself held).

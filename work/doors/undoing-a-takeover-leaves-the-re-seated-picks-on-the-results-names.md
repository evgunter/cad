---
id: undoing-a-takeover-leaves-the-re-seated-picks-on-the-results-names
kind: issue
title: Undoing a takeover restores the document, but the selection and the latched faces keep the result's names and read stale
status: open
opened: 2026-11-01
priority: P3
cost: E
---

Found by the takeover unit (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`). A takeover re-seats the viewer's held picks on the result's names (`Selection::reseated` in `DocSession::create`, `crates/viewer/src/session.rs`; `Drafts::reseat`, `crates/viewer/src/drafts.rs`). One undo restores the document (`crates/viewer/tests/takeover_reseats.rs`, `a_mate_on_the_taken_copys_face_stays_seated`), but the held picks keep the result's names, which the restored copy no longer holds: the selection and a latched face then read stale until re-picked.

The re-seat is not in the history: a history step is the edits alone (`History::commit_group`, `crates/viewer/src/history.rs`). An undo that also restores the picks needs the inverse map kept beside the step, or the undo door to re-seat by the step's `Rebind` edits read backwards (which covers only names the document itself held).

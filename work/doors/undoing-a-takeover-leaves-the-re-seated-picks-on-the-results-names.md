---
id: undoing-a-takeover-leaves-the-re-seated-picks-on-the-results-names
kind: issue
title: Undoing a takeover restores the document, but the selection and the latched faces keep the result's names and read stale
status: open
opened: 2026-11-01
priority: P3
cost: E
---

Found by the takeover unit (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`). A takeover moves every pick held outside the document onto the result's names at its commit: `DocSession::create` hands the door the drawn copies the session's selection and the chrome's holders (`Holds`: the forms' latched faces and the open tool's picks) hold names on, and `DocSession::follow` moves them all with the door's answer (`TakenOver::moved`, `crates/viewer/src/session.rs`). One undo restores the document (`crates/viewer/tests/takeover_reseats.rs`, `a_mate_on_the_taken_copys_face_stays_seated`), but the held picks keep the result's names, which the restored copy no longer holds. They then read stale (`Resolution::Failed`, a typed stale reading, never a wrong face) until re-picked. A redo is the mirror case: it restores the result, but a pick re-picked on the restored copy meanwhile holds the old names, and the redo moves nothing (a history step replays edits; it runs no door).

The move is not in the history: a history step is the edits alone (`History::commit_group`, `crates/viewer/src/history.rs`). An undo or redo that also moves the picks needs the step's `Moved` kept beside it (inverted for the undo), or the step door to move held picks by the step's `Rebind` edits read in either direction (which covers only names the document itself held).

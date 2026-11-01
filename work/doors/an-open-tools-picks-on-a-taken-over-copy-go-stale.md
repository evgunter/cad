---
id: an-open-tools-picks-on-a-taken-over-copy-go-stale
kind: issue
title: A takeover re-seats the selection and the forms' latched faces, but not an open tool's picks on the copy it takes over
status: open
opened: 2026-11-01
priority: P3
cost: E
---

Found by the takeover unit (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`). A takeover answers the names it moved per copy (`TakenOver`, `crates/editor-core/src/refactor/take_over.rs`), and the viewer re-seats the session's selection (`Selection::reseated`, `crates/viewer/src/session/select.rs`, called from `DocSession::create`) and the drafts' latched faces (`Drafts::reseat`, `crates/viewer/src/drafts.rs`, called from the app's outcome loop in `crates/viewer/src/app.rs`).

An open tool's picks are not re-seated: the mate tool's held faces (`MateTool`, `crates/viewer/src/matetool.rs`), the mirror tool's plane (`crates/viewer/src/combine.rs`, `plane: Option<FaceSelection>`) and the blend tool's edges (`crates/viewer/src/blend.rs`). A tool's own commit closes it, so this needs a takeover authored while another tool is open (a form's sketch on a face while the mate tool holds a face of the same copy). The pick then names the copy's old source's name and reads stale.

The fix is one `reseat` per tool beside `Tools::respeak` (`crates/viewer/src/tools.rs`), fed the outcome's `reseated`.

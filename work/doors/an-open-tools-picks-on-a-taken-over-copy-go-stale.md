---
id: an-open-tools-picks-on-a-taken-over-copy-go-stale
kind: issue
title: A takeover re-seats the selection and the forms' latched faces, but not an open tool's picks on the copy it takes over
status: closed
opened: 2026-11-01
priority: P3
cost: E
closed: 2026-11-01
---

Found by the takeover unit (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`). A takeover answers the names it moved per copy (`TakenOver`, `crates/editor-core/src/refactor/take_over.rs`), and the viewer re-seats the session's selection (`Selection::reseated`, `crates/viewer/src/session/select.rs`, called from `DocSession::create`) and the drafts' latched faces (`Drafts::reseat`, `crates/viewer/src/drafts.rs`, called from the app's outcome loop in `crates/viewer/src/app.rs`).

An open tool's picks are not re-seated: the mate tool's held faces (`MateTool`, `crates/viewer/src/matetool.rs`), the mirror tool's plane (`crates/viewer/src/combine.rs`, `plane: Option<FaceSelection>`) and the blend tool's edges (`crates/viewer/src/blend.rs`). A tool's own commit closes it, so this needs a takeover authored while another tool is open (a form's sketch on a face while the mate tool holds a face of the same copy). The pick then names the copy's old source's name and reads stale.

The fix is one `reseat` per tool beside `Tools::respeak` (`crates/viewer/src/tools.rs`), fed the outcome's `reseated`.

## Closed

Closed in the PR 6264 fix pass. A takeover's held-pick re-seat is now one app method over the forms and the open tool (`App::reseat_held`, `crates/viewer/src/app.rs`), fed at landing by `DocSession::take_landed_reseats`. `Tools::reseat` (`crates/viewer/src/tools.rs`) re-seats the mate tool's faces, the mirror tool's plane and the blend tool's edges and rims. The row is `app::properties_pane_tests::an_open_tools_pick_on_a_taken_over_copy_is_re_seated`.

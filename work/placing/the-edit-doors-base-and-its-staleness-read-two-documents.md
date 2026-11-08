---
id: the-edit-doors-base-and-its-staleness-read-two-documents
kind: issue
title: The Edit door loads its base from the shown document but reconcile judges it stale against the landed one
status: closed
opened: 2026-11-02
branch: placing/place-live
pr: 6578
closed: 2026-11-08
---


Found by PR 6300's dual review (NOTE); filed by `placing-bundle-editor` in the fix pass.

`MateTool::edit` (`crates/viewer/src/matetool.rs`) loads `PlaceDoor::Edit`'s `base` from the document it is handed. The chrome hands it the shown, committed one (`Tools::feed`, the tree's `edit placement`). `MateTool::reconcile` compares `base` with `held_bundle` of the landed document (`Tools::reconcile` passes the landed pair). While a run is held, the two documents differ, so a fresh draft reads as stale and reloads, and its picks go with it. `SessionOp::SetBundle` judges staleness against the committed document (`DocSession::bundle_fresh`).

**Owed.** One document for both questions: the committed one, which the Apply is judged against. **Done when** a row opens the Edit door while a run is held, reconciles, and keeps its draft.

## Closed (PR 6578)

`MateTool::reconcile(committed, landed)` judges the draft's bundle against the committed document (`Tools::reconcile` is handed `committed_doc()`, `ViewerApp::survive_tools`), and pick survival stays on the landed pair. Done-when row: `place_live::a_draft_opened_while_a_run_is_held_is_kept_by_reconcile`.

Residue, unsure it bites (PR 6578's review, judgement d): `Tools::feed` loads an editor's `base` from `session.doc()`, which is a gesture's scratch while one is in flight (`ViewerApp`'s frame, `tools.feed`). A slot drag on the placement's own value while a pick opens its editor would load the dragged bundle.

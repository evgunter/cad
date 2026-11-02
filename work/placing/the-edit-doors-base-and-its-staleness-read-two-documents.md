---
id: the-edit-doors-base-and-its-staleness-read-two-documents
kind: issue
title: The Edit door loads its base from the shown document but reconcile judges it stale against the landed one
status: open
opened: 2026-11-02
---


Found by PR 6300's dual review (NOTE); filed by `placing-bundle-editor` in the fix pass.

`MateTool::edit` (`crates/viewer/src/matetool.rs`) loads `PlaceDoor::Edit`'s `base` from the document it is handed. The chrome hands it the shown, committed one (`Tools::feed`, the tree's `edit placement`). `MateTool::reconcile` compares `base` with `held_bundle` of the landed document (`Tools::reconcile` passes the landed pair). While a run is held, the two documents differ, so a fresh draft reads as stale and reloads, and its picks go with it. `SessionOp::SetBundle` judges staleness against the committed document (`DocSession::bundle_fresh`).

**Owed.** One document for both questions: the committed one, which the Apply is judged against. **Done when** a row opens the Edit door while a run is held, reconciles, and keeps its draft.

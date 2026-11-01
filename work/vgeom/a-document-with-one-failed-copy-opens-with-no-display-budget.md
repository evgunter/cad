---
id: a-document-with-one-failed-copy-opens-with-no-display-budget
kind: issue
title: a document whose product gather refuses gets no display budget, though its healthy copies are drawn
status: open
opened: 2026-11-01
priority: P2
cost: M
---


## Finding

Found by CHROME's partial-picture lane (`work/chrome/a-failed-feature-hides-every-later-edit.md`), checking the all-or-nothing shape one layer over from the pick index.

`crates/viewer/src/session.rs`, `DocSession::fit_request`: the display budget is asked only for a landing whose product gather succeeded; for a landing whose gather refused (`run.gather` is `Err`, which one failed or poisoned world copy is enough for) it answers `None`, and `ViewerApp::sync_scene` (`crates/viewer/src/app.rs`) spends the budget on that first landing either way (`derived.fit_delta = false`). So a document that OPENS with one failed node is never budgeted: its healthy copies are tessellated at the un-budgeted δ in force, which is the build `scene::TRIANGLE_BUDGET` exists to avoid, on exactly the dense documents it was cut for.

The doc comment's reason ("the index build below is about to refuse it with its own typed answer") stopped being true when the pick index became per-copy: it now draws every copy with a value (`PickIndex::undrawn` holds the rest), so there is a picture to price.

Not a two-line fix: the fit's subject is one gathered `Body` (`evalseam::FitSubject`), and a refused gather has none. The likely shape is to price the copies the index would draw (the drawn copies with a value) rather than the product, or to keep the budget armed until a landing has something to price — that choice is VGEOM's.

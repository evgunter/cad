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

## 2026-11-02 — the empty-product case, and the budget now stays armed

From VSEAM's `an-empty-new-document-spends-both-fits-on-a-boundless-scene`.

- **The budget is now kept armed** until a landing has something to
  price. `ViewerApp::sync_scene` spends `fit_delta` only on a landing
  whose `fit_request` is `Some`, and `ViewerApp::set_delta` disarms it,
  so a typed δ still wins. That is the second of the two shapes above.
  The first shape, pricing what the index draws, is still open, and
  it is what this row now asks for.
- **A refused gather is not the only `None`.** A document whose
  product is EMPTY gets none either (`ProductError` `EmptyProduct`,
  which `means_no_body`), while its picture draws every body. That is
  every document authored in the viewer: a body lands unplaced, and
  "Add part…" lands an instance in a space of its own, which the
  product door refuses ("its body is in Space …, not the world"). So
  such a document is never priced, however many instances it draws.
  `app::properties_pane_tests::an_empty_new_documents_first_body_is_framed_and_budgeted`
  shows it: after `NewDocument` and one `AddInstance`, the scene has
  bounds, `fit_delta` is still owed, and nothing has been priced.

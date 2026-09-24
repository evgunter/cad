---
id: a-creation-forms-held-pick-survives-a-document-swap
kind: issue
title: A creation form's held pick survives a document swap and resolves against the new document
status: open
opened: 2026-09-21
priority: P1
cost: D
---


## Finding

Found by AUTH-1's correctness reviewer (AUTHOR, PR 2955, 2026-09-21),
confidence `likely`, argued from `crates/viewer/src/app.rs` and
`DocSession::clear_for_new_document` rather than driven.

**A creation form's held pick outlives the document it was picked
in.** `Drafts` is app state built once (`app.rs`) and nothing resets
it; `clear_for_new_document` (`crates/viewer/src/session.rs`) clears
`derived`, so the SELECTION goes, but it cannot reach the drafts. A
`RecipeNodeId` is a small integer and a `StableName` carries no
document identity, so opening a second document whose node ids
coincide — two box-shaped documents is the ordinary case, not a
contrived one — leaves the stale pick RESOLVING against the new
document, the gate admitting it, and the form's commit button live
with nothing selected in the viewport.

## Two instances, one pre-existing

- `Drafts::datum_frame`, the add-datum form's frame pick, held through
  `frame_picker` (`crates/viewer/src/pane/create.rs`) — **pre-existing**:
  the picker keeps an id that need not be in the new document's list.
- `Drafts::datum_face`, AUTH-1's face latch — the same hazard, newly
  added, and the reason the class surfaced.

AUTH-1 corrects its own doc, which claimed the gate's per-frame read
of the landed evaluation made a stale pick safe (it does not — the
gate cannot tell one document's name from another's), and leaves the
BEHAVIOUR alone deliberately, because the class is wider than that
unit. That is this row.

## Shapes

1. A doc-replacing op clears the picks — which means `Drafts` learning
   about document identity, or the session clearing what it can reach.
2. A held pick carries the document it was picked in, and a pick from
   another document reads as no pick.
3. Nothing, with the hazard stated at each holder — the weakest, and
   what the tree has today by accident rather than by decision.

## Why P1

Two forms already hold a pick this way and every new form is another.
`work/README.md`'s P1 band: a special case that should be handled
uniformly, before more things are built on it.

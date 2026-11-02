---
id: display-frames-are-not-saved-with-the-document
kind: issue
title: display frames (an Add part… frame, a free-move) are not saved with the document, so a reopened file draws two inserted parts on each other
status: dispatched
opened: 2026-11-01
priority: P1
cost: M
branch: spaces/saved-frames
---


Found by the review of PR 6269 (executed): after Save then Open,
`free_move_of(second space)` is `None`, so two "Add part…" of one file
are drawn on top of each other again. `DisplayState` has no saved form.

IDENTITY DI5 allows it ("a free-move may write display metadata … viewer
data outside the canonical bytes, never hashed into a `ContentPin` and
never read by evaluation; without it, a reopened document draws its
shown spaces at a default pose", Ev on PR 5312), and Ev's answer on PR
6209 (the insert gesture *stores* the new space's frame, one way to
store where something is displayed) relies on it for the reopen case.
Nothing writes or reads it yet.

**Done when.** A document's display frames (insert frames and free-move
frames, keyed by space) are saved as viewer data outside the canonical
bytes and restored on Open; a row saves two inserted parts, reopens, and
finds them apart; a document opened without that data draws at the
identity; the content pin and evaluation never read it.

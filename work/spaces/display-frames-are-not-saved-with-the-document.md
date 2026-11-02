---
id: display-frames-are-not-saved-with-the-document
kind: issue
title: display frames (an Add part… frame, a free-move) are not saved with the document, so a reopened file draws two inserted parts on each other
status: closed
opened: 2026-11-01
priority: P1
cost: M
branch: spaces/saved-frames
pr: 6296
closed: 2026-11-02
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

## Closed

PR 6296. Display frames (insert and free-move) are saved beside the
document as `<file>.pncad.layout` (TOML, `crates/viewer/src/layout.rs`,
through the preferences' `FileStore`): viewer data outside the canonical
bytes (the `.pncad` bytes and `ContentPin` are identical with and without
it), read with the preferences' posture (an unknown key or unreadable
frame reported, the rest applied). Each frame carries a witness node; one
rule, `display::space_standing`, decides whether a frame still belongs to
its space in session and on reopen (IDENTITY DI5 as ratified). Review:
single FULL, fixed.

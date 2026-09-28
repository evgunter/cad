---
id: the-cursor-reads-the-display-view-in-hand-not-the-one-the-picture-was-drawn-under
kind: issue
title: The cursor comparison and the pick read the display view in hand, which a refused scene rebuild leaves ahead of the picture on screen
status: open
opened: 2026-09-28
priority: P3
cost: M
---


Found by `vnews/every-id-is-said`, which made the id pass's answer
read through the display view (`idpass::IdAnswer::of`).

## The finding

`crates/viewer/src/app.rs`'s frame hands the viewport
`self.session.display_view()`, the view as it stands this frame. The
picture on screen was drawn under the view `sync_scene` last built it
with, and a rebuild that REFUSES (`scene_focused` answers `Err`, held
as `scene_fault`) leaves the old picture up while the view in hand has
moved on. `ViewerApp::revision` does not bump on the refusal, so the
id log's `IdSubject` holds the query too.

In that window `crates/viewer/src/pane/viewport.rs` asks both picking
paths under the NEW view. After a hide:

- the ray path (`faces_under_cursor`, and the pick path's `op_under`)
  leaves out a part the picture still draws, so the cursor over it
  answers *nothing* or the face behind it;
- `idpass::IdAnswer::of` reads the id buffer's word for that part's
  patch as `IdAnswer::Undrawn`, so the frame says *"id N, which no
  patch of this picture draws"* about a patch that is on screen.

Before `every-id-is-said` the second half read *"id buffer <face>, ray
nothing"*. That was also false, so the change altered the words of an
existing wrong sentence and did not open a new one. The picking
defect in the first half is the larger one.

## What a fix would be

Keep the `DisplayView` a successful rebuild drew with beside
`scene_key` (the `scene_display` revision already marks which one it
was) and hand the viewport that view, as `drawn_index` hands it the
index the picture was drawn from. Which of the pane's readers want the
drawn view and which want the view in hand (the tree's hide toggles,
for instance) is the question to settle first.

## Fence

`crates/viewer/src/app.rs` (`sync_scene`, the `ViewerBehavior` field
`display`) and `crates/viewer/src/pane/viewport.rs`. VSEAM owns
`viewerapp-document-derived-state-has-no-boundary`, which is nearby.

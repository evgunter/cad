---
id: a-pick-over-a-stale-picture-answers-about-a-picture-nobody-can-see
kind: issue
title: A click over a stale picture resolves against the current index, so it selects something the screen is not showing
status: open
opened: 2026-09-15
---


## Disclosed by the co-guard unit, deliberately not decided by it

`drawn_index` (`crates/viewer/src/pane/viewport.rs`) sorts the pane's
reads of the index into *about the picture* and *about the document*,
and routes only the first through the picture's `(generation, δ)` key.
The pick path is the second: `PickIndex::op_under` takes the index and
the session's evaluation, resolves a ray against the index's own
geometry, and answers in the session's currency. It is not gated on the
picture, and the rule's own doc says so in as many words.

**So there is a window where a click answers about geometry that is not
on screen.** `ViewerApp::sync_scene` marks the scene's pair current only
on a successful rebuild, so a landed index over a refused
`scene_focused` leaves a newer index beside an older picture, held there
until the display revision or the focus set moves. `frame::scene_badge`
reads `scene_fault` and says the picture is older than the document —
but the cursor still picks, and what it picks is the document's answer
for a shape the user cannot see.

## Why it was left open rather than fixed

Both available answers are product decisions, not repairs:

- **Refuse.** A typed *the picture is older than what you are pointing
  at*, in the shape `pickcache::unindexed` already has for *no index to
  ask*. Honest, and it makes a refused tessellation stop the mouse.
- **Answer anyway**, which is what happens today, on the argument that
  the index is what the selection is expressed in and refusing a pick
  that would have worked is a worse failure than answering about a
  picture one frame stale.

Nothing in the tree picks between them, and the guard unit had no
mandate to. Whoever takes this owes the reachability question too: the
window needs a refused `scene_focused` over a landed index, which no
test in `crates/viewer/` currently arranges.

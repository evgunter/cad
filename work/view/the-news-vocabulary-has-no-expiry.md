---
id: the-news-vocabulary-has-no-expiry
kind: issue
title: Nothing owns when news stops being news, so a stale-but-true complaint sits on the line indefinitely
status: open
opened: 2026-09-04
refs: [camera-fold-clears-status-line, status-line-writers-bypass-the-ranking]
---

## The gap

`crates/viewer/src/frame.rs` states what the status line is FOR — one
frame's news, ranked by `frame_status` — and `camera-fold-clears-status-line`
landed the two rules that follow from it. **Neither says when a piece of
news stops being news.**

`frame_status` owns which of a frame's messages WINS. The only thing
that ever removes one is `batch_status`'s `StatusUpdate::Clear`, which
fires when a batch contains an op `acts` counts as an action — and
`acts` deliberately excludes `Hover`, which is the only op pure
navigation emits. So:

**Refuse a camera operation, then navigate.** `frame::fold_status`
answers `Keep` on every subsequent clean fold, and no batch acts, so
the camera refusal stays on the line for as long as the user orbits.
Before `camera-fold-clears-status-line`, the next clean fold swept it —
which made camera refusals nearly invisible, so this is plausibly the
better trade, but it is a behaviour change that the stated rule does
not cover and the item disclosed rather than resolved.

Same class, and pre-existing:

- `crates/viewer/src/pane/viewport.rs:324` — `projection: {error}`,
  re-written on every frame until the camera moves.
- `crates/viewer/src/pane/viewport.rs:363` — the two picking paths
  disagreed at a cursor that has since moved on.

None of these becomes FALSE. Each becomes stale-but-true, which is a
different failure and one the current vocabulary cannot express:
`StatusUpdate` has `Keep`, `Clear` and `Show`, and nothing that says
"this was news, and is not any more".

## The question

Does a message carry an expiry, and if so, what expires it?

Candidates, none obviously right:

1. **Nothing — accept staleness.** Argue that a true sentence on the
   line costs less than a swept one, and let `Clear` on an acting
   batch be the only sweeper. Cheapest; the status quo after
   `camera-fold-clears-status-line`.
2. **A message carries its subject**, and a later message about the
   SAME subject supersedes it — a camera verdict expires on the next
   camera event whatever it says, a projection refusal on the next
   camera move, a disagreement on the next cursor move. This is the
   typed-status shape `camera-fold-clears-status-line` explicitly did
   NOT take (its candidate 1) — it did not need it then; this is the
   case that would.
3. **A frame count or a wall clock.** Rejected on sight for a fault,
   but arguably right for a report nobody has to act on.

## What this unblocks

`work/view/opoutcome-superseded-has-no-production-reader.md` asks
whether a discarded free-move probe deserves the line, a badge or
nothing. Answering it needs exactly this vocabulary: a superseded probe
is news if the discard is the outcome of an act, a standing fact if
"the probe you are reading is stale" persists, and neither answer is
expressible until "stops being news" has an owner.

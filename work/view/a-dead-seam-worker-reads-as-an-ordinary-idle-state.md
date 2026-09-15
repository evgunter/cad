---
id: a-dead-seam-worker-reads-as-an-ordinary-idle-state
kind: issue
title: A seam whose worker has died is indistinguishable, in the chrome, from one with nothing to do
status: open
opened: 2026-09-15
refs: [2637]
---



## What

`indexing-seam-outlives-a-worker-panic`'s residue, and the fork it
ruled out rather than a defect it left. That item stopped the chrome
promising an answer a dead worker will never send. What it did not
decide is what the chrome should say **instead of** the promise, and
today the answer on all three seams is *nothing*:

- **index** (`crates/viewer/src/pickcache.rs`, `PickCache::indexing`) —
  the indicator goes dark and a click is refused `NotIndexed::Absent`,
  whose sentence now lists a dead seam among three causes. The picture
  on screen is the last one that built and no pick will ever be
  answered again, for the life of the window.
- **evaluation** (`crates/viewer/src/session.rs`,
  `DocSession::outstanding`) — `Outstanding::Canceled`, drawn as
  *canceled — showing an older result* with a Re-evaluate button. The
  state is honest about the seam being idle and **wrong about the
  cause**, and the recourse it offers does nothing: `Reevaluate`
  submits into a `Sender` whose receiver died with the worker, so
  `Coalescing::dispatch`'s failed-send arm fires and the button
  answers by changing nothing.
- **fit** (`crates/viewer/src/app.rs`, the two `FitService::busy`
  reads) — `settled` becomes `Some(self.delta)` and the index builds at
  the δ in force. That is the un-budgeted build the display budget
  exists to avoid, taken silently, on exactly the documents the budget
  was cut for.

None of the three worker handles respawns, so every one of these is
permanent.

## Why it was not taken with the fix

The fix shape the parent item asked for is *a consumer whose "work is
outstanding" answer consults the seam it asked* — a read, in the
consumer, of a value the seam already publishes. This is a different
thing: a new typed fact (**the worker is gone**, as against **no work
is outstanding**), a decision about which of `frame`'s channels carries
it — a badge, by the provenance rule, since it is a whole-run
environmental fact that outlives the frame it started on — and its
tone and affordance. `ThreadIndexer::spawn`'s docs already argue the
matching case for a worker that never STARTED: loud rather than
degraded, because *"a seam whose worker never started accepts every
submit and answers none"*. A worker that started and died leaves
exactly that state, and takes the quiet path.

That is a design decision on `crates/viewer/GUI-DESIGN.md`'s side of
the record/design line, and it wants Ev.

## What it would cost to measure

Nothing is known about how often a seam worker dies — the parent item
says the same, and it is still true: this is filed because the state is
silent and permanent, not because it is frequent.

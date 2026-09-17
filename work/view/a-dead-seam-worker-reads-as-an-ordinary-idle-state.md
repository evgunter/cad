---
id: a-dead-seam-worker-reads-as-an-ordinary-idle-state
kind: issue
title: A seam whose worker has died is indistinguishable, in the chrome, from one with nothing to do
status: closed
opened: 2026-09-15
refs: [2637]
closed: 2026-09-17
branch: view/dead-seam-badge
pr: 2762
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

## RULED (Ev, in-chat, 2026-09-16): loud, Actionable, and the fit seam refuses

**It is loud, not silent.** A badge, by the provenance rule the item
already cites — a whole-run environmental fact outliving the frame it
started on.

**`Tone::Actionable`.** The orchestrator raised `Advisory` on the
ground that nothing in the app can revive a worker; Ev's answer is that
it could hardly be anything else. The reasoning holds on the
vocabulary's own words: `Advisory` is *"there is nothing to do about
it"*, and restarting the viewer IS something to do — a dead worker
means every later answer on that seam is missing for the life of the
window, which is exactly a verdict a reader needs to act on. The
restart belongs in the badge's own text, so the tone and the recourse
agree.

**The fit seam REFUSES the index build** rather than taking the
un-budgeted one. Today a dead fitter makes `settled` `Some(self.delta)`
and the index builds at the δ in force — the un-budgeted build the
display budget exists to prevent, taken silently, on exactly the
documents the budget was cut for. Ev: *"presumably it should refuse."*

**The consequence a taker should state rather than discover**: on a
document large enough to need the budget, refusing means no index, so
no picking, for the life of the window. That is the right trade — a
frozen window is worse than a dead one, and the badge now says why —
but it is a second silent-capability loss stacked on the first, and the
badge text should account for both rather than naming only the fitter.

## SUPERSEDED (Ev, in-chat, 2026-09-17): a crashed worker panics

The ruling above was made on a description of the cost that was wrong,
and Ev replaced it rather than amending it.

**What moved.** The review of #2762 found that the fit refusal does not
cost picking, it costs the picture: `settled_delta` answers `None`
forever once the fitter is gone, `PickCache::sync` then forgets and
returns `CacheStep::Nothing`, and `ViewerApp::sync_scene` returns on
that step BEFORE the scene rebuild — and `self.scene` has one writer.
On a fresh open, where the first landing is what fires the fit, the
document never draws at all. So the justification the refusal rested on
— *a frozen window is worse than a dead one* — pointed the other way
once the true cost was on the table.

**The ruling.** *"isn't a worker dying an infra thing that should show
up as a panic?"*, then *"panic on crash is good."* A crashed worker is
not a state the application may be in, so it is not described, badged
or worked around: the process ends at the point of detection.

## Closed

Closed by the panic, not by the badge.

`Coalescing` now tells its two endings apart — `close` takes the
request channel and is called from `Drop` alone, so a detection that
still holds the channel is a crash and nothing else. Shutdown forgets
quietly (`forget_worker`); a crash panics on the UI thread naming the
seam (`crashed`), and the message says it is a bug and offers no
recourse, because the process is already going down.

The whole vocabulary the first ruling asked for is deleted — the badge
family member, the typed `WorkerGone`, `worker_gone` on the three seam
traits, `settled_delta` and the fit refusal, the disabled Re-evaluate
control. None of it described a state anything can now be in.

Residue, filed rather than left in this prose:

- `the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer` —
  `IndexService::busy`'s half of `PickCache::indexing`, and
  `Coalescing::forget_worker`, lost their producers to the ruling.
- `the-dying-seam-fakes-mirror-a-machine-they-do-not-share` — the two
  `Dying*` fakes are hand-written mirrors of `Coalescing`'s bookkeeping
  and no longer agree with it.

The earlier residue row, `the-canceled-label-names-a-cause-a-dead-worker-did-not-have`,
was **deleted rather than carried**: its premise was a dead evaluator
reaching `Progress::Canceled`, which the ruling makes unreachable.

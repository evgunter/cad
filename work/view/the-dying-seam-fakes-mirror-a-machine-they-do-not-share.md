---
id: the-dying-seam-fakes-mirror-a-machine-they-do-not-share
kind: issue
title: The Dying* seam fakes are a hand-written mirror of Coalescing's bookkeeping and no longer agree with it
status: closed
opened: 2026-09-17
priority: P3
cost: E
closed: 2026-09-21
branch: view/seam-residue
---



## What

`tests/frame_policy.rs` stands up `DyingIndexer` and `DyingEvaluator`:
two structs that re-implement `Coalescing`'s bookkeeping by hand —
a `running` flag set from `send(...).is_ok()`, cleared on
`TryRecvError::Disconnected` — behind a worker thread that really
panics.

**They are a mirror, and mirrors drift.** They already did: the shipped
machine now discriminates its two endings (a request channel still in
hand at either detection arm is a crash, and it panics), and these do
not — they go quiet, which is what the shipped handles used to do.

So the consumer rows built on them certify `PickCache` and `DocSession`
against an `IndexService`/`EvalService` implementation that exists
nowhere in `src/`. That is not nothing — the traits are public and the
contract is real — but it is not what the rows' names claimed, and a
reader met them as evidence about a worker panic.

The rows were renamed at `view/dead-seam-badge` so the names say what
they drive (`a_seam_that_goes_quiet_stops_promising_an_answer`,
`a_quiet_evaluator_reaches_the_chrome_as_canceled_not_as_evaluating`).
The mirror itself is untouched.

## The repair, and the question under it

The shape that removes the class is the one the crate already uses
elsewhere: drive the CONSUMER through the shipped seam rather than
through a hand-written stand-in for it. That is not free here —
`Coalescing` is private to `evalseam::threaded` and the shipped
handles cannot be crashed from outside by construction — so the
options are to stop asserting the quiet-seam state at all (which is
`the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer`'s
question), or to give the seam module a test-only door and accept what
that costs.

Both are decisions rather than edits, and they are the same decision as
the sibling row's; whoever takes one should take both.

## What it would cost to measure

Nothing. The divergence is visible by reading the two fakes against
`Coalescing::dispatch` and `Coalescing::poll`.

## Closed: both fakes deleted, and neither was covering a live state

The repair is the first of the two options this row names — stop
asserting the quiet-seam state at all — because the sibling row
settled that there is no such state to assert about.

**`DyingIndexer` was modelling a dead state.** Its whole subject was
`PickCache::indexing`'s seam conjunct, which is deleted; nothing else
in the row was unique. The retry-policy tail
(`CacheStep::Indexing` on a re-sync) is held by
`a_refused_index_is_attempted_once_per_generation_and_not_once_per_frame`,
and `unindexed`'s `Absent`/`Building` discrimination by
`a_click_with_no_index_refuses_typed_and_a_hover_stays_quiet`.

**`DyingEvaluator` was modelling a LIVE state by a dead route, and the
live route is already covered.** `Outstanding::Canceled` is reachable
through the shipped seam — a real `SessionOp::CancelEvaluation` leaves
`busy() && !running()` — and
`tests/eval_seam.rs`'s
`a_cancel_keeps_the_last_good_picture_and_reevaluate_recovers_it`
drives exactly the fold this row asserted, through `InlineEvaluator`,
plus the `Reevaluate` recovery the fake could not exercise at all.
`frame::progress(Outstanding::Canceled, false)` is pinned directly by
`the_chrome_has_one_progress_state_and_evaluation_outranks_indexing`.
So the fake's only contribution was reaching a covered state by a
route no implementation may take.

`dying_worker` went with them, which removes the hand-written mirror
class from this file entirely. What remains driving `Coalescing`'s two
endings is `evalseam`'s own `mod tests`, which drives the shipped type
rather than a copy of it — the shape this row asked for.

Receipt: 714 rows green after the deletion, and every assertion the two
rows made is named above with the row that still makes it.

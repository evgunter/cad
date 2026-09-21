---
id: the-dying-seam-fakes-mirror-a-machine-they-do-not-share
kind: issue
title: The Dying* seam fakes are a hand-written mirror of Coalescing's bookkeeping and no longer agree with it
status: open
opened: 2026-09-17
priority: P3
cost: E
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

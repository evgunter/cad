---
id: display-clear-drops-free-move-placements-silently-while-prune-reports-them
kind: issue
title: DisplayState::clear swallows the free-move placements prune announces
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: open
opened: 2026-09-05
---


Found by the #1885 style review (S5). Pre-existing. Line numbers are
as of `6acf001b`, before #1886 lands.

## The asymmetry

`DisplayState` has two ways to lose a free-move placement, and they
report differently:

- `prune` (`crates/viewer/src/display.rs:651-679`) collects every
  placement it drops into `discarded` and RETURNS it. That vector is
  `OpOutcome::superseded`, which `app.rs:785` turns into
  `frame::supersession_notice` — the user is told.
- `clear` (`crates/viewer/src/display.rs:683-690`) does
  `self.moves.clear()` and returns `()`. The same placements, dropped
  by `Open` or `NewDocument` through
  `DocSession::clear_for_new_document`, are swallowed.

So whether a user hears that their placements are gone depends on
which op removed them, and the case where MORE is removed is the quiet
one.

## Why it matters more now

A sibling lane (#1886) has just made `prune` report more of what it
discards. Every widening of the reporting side widens this gap rather
than narrowing it: the two paths through this module diverge further
each time the announced one improves.

## What resolving it looks like

`clear` returns what it dropped, on the same channel `prune` uses, and
`clear_for_new_document`'s callers put it on the outcome. The
counter-argument to weigh in the fix: a document REPLACEMENT arguably
does not owe a per-placement notice the way an edit does, since the
whole session's subject changed — if that is the answer, it should be
written at `clear`, because today the silence reads as an oversight
and not as a decision.

Related: `prune-drops-a-hidden-instance-silently` (the hidden set, on
the `prune` side), `free-move-drag-dissolved-by-open` (the in-flight
gesture, dropped by the same `clear` call).

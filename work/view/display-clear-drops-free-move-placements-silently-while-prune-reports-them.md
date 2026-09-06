---
id: display-clear-drops-free-move-placements-silently-while-prune-reports-them
kind: issue
title: DisplayState::clear swallows the free-move placements prune announces
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: open
opened: 2026-09-05
---


Found by the #1885 style review (S5). Pre-existing. Written against
the tree with #1886 merged, which widened the reporting side.

## The asymmetry

`DisplayState` has two ways to lose display state, and they report
differently:

- `prune` (`crates/viewer/src/display.rs:799-836`) returns a
  `PruneReport` (`:540-556`) carrying `superseded` — every committed
  free-move placement it discarded — AND, since #1886,
  `dropped_hides`, every hide the document stopped admitting, each
  with the `DisplayFault` that explains it. `DocSession` returns that
  report from `step` and `commit_action` (`session.rs:1213,1590`) and
  the chrome renders it.
- `clear` (`crates/viewer/src/display.rs:839-846`) does
  `self.hidden.clear()`, `self.moves.clear()`, `self.free_move = None`
  and returns `()`. The same placements and the same hides, dropped by
  `Open` or `NewDocument` through
  `DocSession::clear_for_new_document`, are swallowed.

So whether the user hears that their placements and hides are gone
depends on which op removed them, and the case that removes MORE — all
of them, unconditionally — is the quiet one.

## Why it is wider than it was

Before #1886 the gap was one kind of news (`Vec<RecipeNodeId>` of
superseded placements) announced on one path and swallowed on the
other. #1886 made `prune` report a second kind, with a typed cause for
each. Every widening of the reporting side widens this gap rather than
narrowing it: `clear` now swallows two kinds of withdrawal that its
sibling explains individually, and `PruneReport`'s own doc — *"the
report a caller turns into what the user reads"* — is true of exactly
one of the two ways this module withdraws state.

## What resolving it looks like

`clear` returns a `PruneReport` on the same channel `prune` uses, and
`clear_for_new_document`'s callers put it on the outcome. The
counter-argument to weigh in the fix: a document REPLACEMENT arguably
owes no per-instance notice, since the whole session's subject
changed, and `PruneReport`'s wording ("superseded", "the document no
longer admits it") is about a document that MOVED, not one that was
swapped. If that is the answer, it belongs written at `clear` — today
the silence reads as an oversight rather than as a decision, which is
why this is filed rather than left.

Related: `prune-drops-a-hidden-instance-silently` (closed by #1886 on
the `prune` side; this is the same news on the other path),
`free-move-drag-dissolved-by-open` (the IN-FLIGHT gesture, dropped by
this same `clear` call and named by neither report).

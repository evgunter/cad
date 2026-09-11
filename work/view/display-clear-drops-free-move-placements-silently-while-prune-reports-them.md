---
id: display-clear-drops-free-move-placements-silently-while-prune-reports-them
kind: issue
title: DisplayState::clear swallows the free-move placements prune announces
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: closed
opened: 2026-09-05
closed: 2026-09-11
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
  report from `step` and `commit_action` (`session.rs:1510,1882`) and
  the chrome renders it.
- `clear` (`crates/viewer/src/display.rs:849-862`) destructures and
  does `hidden.clear()`, `moves.clear()`, `*free_move = None`
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

## Closed

Closed by `view/silent-withdrawals`, and closed by ANSWERING rather
than by symmetry: the item's own counter-argument is the right one,
and it is now written at `clear`
(`crates/viewer/src/display.rs:886-924`).

A prune's withdrawals are a SIDE EFFECT of an act about something
else, which is what makes them news. `Open` and `NewDocument` are the
act itself — display state is state of a session over ONE document
(G3), so a user who replaces the document asked for exactly this, and
a per-instance notice would report the act back to the person who
performed it.

The tree makes it more than a taste in wording. A `Withdrawn` carries
a fault ABOUT a document and the only document left to ask is the
replacement, where these ids mean other nodes. Implementing the
item's proposal (`clear` reporting through `prune`'s channel) and
measuring it:

- reopening the same file, the report is EMPTY — `superseded=[]`,
  `dropped_hides=[]`, `killed_gesture=None` — while all three kinds
  of display state are in fact taken;
- `NewDocument` reports `NoSuchNode { node: 0 }`, a true sentence
  about a document the user has never held state on, offered as the
  explanation of where their hide went.

So the silence is kept and stated. What a user sees is unchanged at
this door and now decided rather than absent, and
`a_document_replacement_takes_all_display_state_and_reports_none_of_it`
(`crates/viewer/tests/assembly_display.rs:782-870`) is what goes red
if it is ever widened back into an oversight — it asserts both that
everything went and that nothing was said.

The IN-FLIGHT drag this same `clear` drops is a different question
and stays open on `free-move-drag-dissolved-by-open`: a drag
dissolved under the pointer is answered with a REFUSAL at the door,
not a report after the fact, and that fork is not this row's to
settle.

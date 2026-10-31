---
id: a-labelled-creation-composes-its-label-edit-after-the-action
kind: issue
title: A labelled creation appends its SetLabel to the recorded action by hand instead of recording it in the action's run
status: closed
opened: 2026-10-03
closed: 2026-10-31
priority: P1
cost: M
pr: 6128
branch: vseam/a-labelled-creation-is-one-recording
---

Found by RECIPE's sweep for
`three-loops-apply-several-edits-and-net-their-maintenance` (its second
pass, shaped at compositions done at the history level rather than over
`Applied`).

## What

`DocSession::create_labelled` (`crates/viewer/src/session.rs`) runs a
creation through `perform`, which records it as one action, then applies
`DocEdit::SetLabel` on the id it minted by hand: `apply` against the
history's document, `History::extend_current` to graft the edit and its
document onto the state the creation recorded, and
`outcome.committed.push(edit)` to splice it into the outcome. The
action's edits, document and outcome are composed a second time, outside
`editor_core::Recording` — the one recorder the viewer's own `stage_run`,
refactor and `regauge_then_mate` share, and the one pncad-py's labelled
`Doc.insert` records its insert and label through.

It is not wrong today: a `SetLabel` performs no maintenance
(`Applied::maintenance`'s contract), so the outcome's netted rows stay
the action's, and the graft keeps one undo. It is the shape that
drifts: a second edit of a kind that does report would be composed
outside the net.

## Shape of a fix

Record the label inside the creation's own run, so the action is one
`Recording` end to end: e.g. a `Creation` that carries its label to the
door that stages its run, with the `SetLabel` applied after the run's
last insert. `History::extend_current` then loses its one caller.


## Closed 2026-10-31

Landed by PR 6128. Each creating door now reads the session, writes
nothing, and answers a `Creating` run. `DocSession::perform_op`'s one
exhaustive dispatch hands that run to `DocSession::create`, which stages
it, ends it with the `SetLabel` on `crate::world::made`'s node where a
label is given, and records it. A labelled creation is therefore one
`Recording`: its edits, minted ids and netted maintenance come from one
`Recorded`, and it makes one evaluation request instead of the two the
graft cost. The label is carried by the dispatch alone and reaches no
door. `History::extend_current` is deleted. The sweep for other
history-level compositions in `crates/viewer/src` found none (the hit
list is in the PR body). Filed from the review:
`a-pattern-of-zero-is-refused-by-the-form-and-accepted-by-the-door`.

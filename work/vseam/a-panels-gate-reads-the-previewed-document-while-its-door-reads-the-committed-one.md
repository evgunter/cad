---
id: a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one
kind: issue
title: chrome gates read session.doc() while the doors they stand in for read history.doc()
status: open
opened: 2026-09-20
refs: [a-disabled-control-says-why-in-four-shapes]
priority: P2
cost: E
---

Found by the sweep of `vnews/properties-controls-read-their-refusals`
(PR #2961), after its style review caught the same defect inside that
lane's own diff. The three members below are the residue the lane could
not fix: they are in `crates/viewer/src/app.rs`, which is VSEAM's.

## The class

`DocSession::doc()` answers `derived.scratch` while a gesture is
previewing and `history.doc()` otherwise. `DocSession::committed_doc()`
answers `history.doc()` always. **Every door reached through
`SessionOp` is handed the committed document** — `set_hidden`,
`begin_free_move` and `create_param` name `self.history.doc()` /
`self.committed_doc()` directly, and `DocSession::start` passes
`history.doc()` into every gesture target's check.

So a panel that computes a gate, a notice or a disabled reason from
`session.doc()` is answering a question about a document its own click
will not be judged against. The two agree whenever no preview is in
flight, which is most of the time — and `SessionOp::permitted_during_
value_gesture` is `true` for several ops, so "most" is not "always".

**This is a known shape in this crate, twice over.** `pane/profile.rs`
and `pane/properties.rs`'s `feature_rows_ui` already take
`committed_doc()`, the latter argued as *"so the two never disagree at
a live button"*; and `session/probe.rs`'s `probe_scale` records the
same bug being repaired once already — *"the two arms below once read
two different documents … which agreed only because a probe refuses
while a gesture is in flight … makes that agreement structural instead
of circumstantial."*

## The members left, all in `app.rs`

Each needs its door named before it is called a defect — that is the
work here, and the sweep did not do it:

- **`app.rs`, `DisplayState::reconcile(self.session.doc(), …)`** — the
  sharpest of the three, because reconciliation DISCARDS display state
  whose subject the document no longer admits. A prune decided against
  a previewed document is a discard decided by a gesture that has not
  committed and may be cancelled. Whether a preview can change what
  `prune` admits is the question to answer first.
- **`app.rs`, `marks::focus(index, self.session.doc(), …)`** — likely
  correct as it stands: a highlight is about the PICTURE, and the
  picture is drawn from the previewed document. Wants the reason
  written down rather than a change.
- **`app.rs`, `self.tools.feed(self.session.doc(), &ops)`** — the tool
  state machines read the document to decide what a pick means; which
  document that should be is a decision nothing states.

## What was fixed in fence, for the pattern

In `pane/properties.rs` (VNEWS's), on the PR above: the per-instance
section's `instance_check`, the hide toggle's `display_check` and the
free-move probe's `free_move_check` now read `committed_doc()`, and the
add-parameter form's already-declared notice reads it too, because
`create_param` does. The parameter row's field keeps `doc()` on
purpose — that field IS the drag and must show the previewed value —
with the reason stated at the site.

## Why this is not just tidiness

It is the concrete member of a blind spot the disabled-control census
could not see and PR #2961's sweep named without an instance:
**a gate that agrees with its door by coincidence rather than by
reading the same value.** A reviewer could not construct a preview that
flips the hide toggle's answer today. That is a fact about the current
edit vocabulary, not a property of the code, and it is exactly the kind
of fact that stops being true without anyone editing the panel.

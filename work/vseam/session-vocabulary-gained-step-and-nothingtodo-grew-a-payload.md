---
id: session-vocabulary-gained-step-and-nothingtodo-grew-a-payload
kind: issue
title: session::Step is a new public name in session.rs and Refusal::NothingToDo now carries a direction
status: open
opened: 2026-09-20
priority: P2
cost: E
---

**Notice, not a defect.** Filed so VSEAM meets this in a row rather
than in a merge: VSEAM has rows in flight on
`crates/viewer/src/app.rs` and `crates/viewer/src/session.rs`, and
VNEWS's `vnews/app-controls-read-their-refusals` changed both, plus the
shape of a `Refusal` arm.

## What moved

**1. `Refusal::NothingToDo` carries its direction.**
`crates/viewer/src/session/refuse.rs` (VNEWS's ground, shared with
VSEAM) now spells it `NothingToDo { direction: Step }`, and its
`Display` renders *"nothing to undo"* or *"nothing to redo"* where it
used to render the joint *"nothing to undo or redo"*. The joint
sentence was reachably false: undo at the root refuses while the redo
the cursor just left is live, which is the state
`tests/undo_tree.rs`'s `undo_at_the_root_and_redo_at_a_leaf_refuse_
rather_than_wrap` constructs. Any VSEAM row matching on that arm needs
`NothingToDo { .. }` or a named direction.

**2. `Step` is a new public name re-exported from `session.rs`.**
`pub use refuse::{NodeKindWanted, Refusal, Step, admits};`. It is NOT
in `crates/viewer/src/lib.rs`'s root re-export list, deliberately and
by the precedent `admits` sets there — and
`work/vdoc/every-crate-root-reexport-is-a-second-path-not-the-only-one`
is the argument that a root re-export is a second path rather than the
path. If VSEAM wants the root spelling it is a decision about
`lib.rs`, which is VDOC's.

**3. `DocSession::step` takes the direction instead of a bool.**
`fn step(&mut self, direction: Step)`, private, two call sites in
`perform` (`SessionOp::Undo`/`Redo`). It asks
`Refusal::nothing_to_step` BEFORE the move rather than reading the
move's `None`, because the toolbar's two buttons ask the same question
of the same history to decide whether to draw themselves live — one
predicate, so a button cannot offer a step the door refuses. A
`debug_assert!` at the move states that coupling.

**4. `app.rs`'s toolbar history controls are one loop, not two
blocks.** `toolbar_ui` draws Undo and Redo from
`[(label, direction, op); 2]`, each gated on
`Refusal::nothing_to_step(self.session.history(), direction).is_none()`
and showing that refusal's `to_string()` when it is `Some` — the same
shape the cancel doors twenty lines below already use. The
New-document Create button is the same shape over
`Refusal::empty_name`. **No new function was added to `app.rs`**: this
crossing changed control code and a comment, and minted no door of the
kind `app-rs-gained-a-toned-door-from-a-vnews-unit` records.

## Why it was done from VNEWS

`work/vnews/a-disabled-control-says-why-in-four-shapes` (closed) named
the three controls as genuine hits of the rule *a control a reader
cannot use owes the sentence a click would have been answered with*,
and the two rows it handed down —
`undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`
and `the-new-document-button-states-its-refusal-twice` — are VNEWS's.
The crossing was announced in the PR body per this program's `keep_out`.

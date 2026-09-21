---
id: undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words
kind: issue
title: Undo and Redo are disabled on exactly the condition Refusal::NothingToDo refuses, and say nothing
status: review
opened: 2026-09-19
refs: [a-disabled-control-says-why-in-four-shapes]
priority: P1
cost: E
---

Found by the census in `a-disabled-control-says-why-in-four-shapes`, at
merge base `2654cc111417da806d9786c40136106469096fec`. A genuine hit of
the rule that census lands on: *a control a reader cannot use owes the
sentence a click would have been answered with — when there is such a
sentence.* The qualifier travels with the rule; it is what decides most
of the population, and dropping it here was how this row first read.

## The two halves, and that they are the same condition

`crates/viewer/src/app.rs:1496` and `:1502` draw the history controls:

```
.add_enabled(self.session.history().can_undo(), egui::Button::new("Undo"))
.add_enabled(self.session.history().can_redo(), egui::Button::new("Redo"))
```

Neither has an `on_disabled_hover_text`, and neither draws a sentence
beside it. `crates/viewer/src/session.rs`'s `Session::step` — the
handler both ops reach (`SessionOp::Undo => self.step(true)`,
`SessionOp::Redo => self.step(false)`) — refuses on the *identical*
condition: it calls `history.undo()` / `history.redo()` and, when the
result is `None`, returns `OpOutcome::refused(Refusal::NothingToDo)`.

`Refusal::NothingToDo` renders (`crates/viewer/src/session/refuse.rs`,
`Display`) as:

> nothing to undo or redo

So the sentence exists, has one home, is the sentence the operation
would answer a click with — and the reader is shown a grey button and
nothing else. This is the same defect as
`the-new-document-button-states-its-refusal-twice` with the second copy
empty rather than divergent.

## What a fix has to decide, and it is not obvious

The refusal's wording is **shared between the two buttons** — *"nothing
to undo or redo"* names both directions because a status line reporting
a refusal does not know which was attempted from the sentence alone.
A tooltip does: it hangs off one button. So either

- both buttons show `Refusal::NothingToDo.to_string()` and the Undo
  button says "nothing to undo or redo", which is half about a control
  the reader is not hovering; or
- `NothingToDo` grows a direction (or a wording helper takes one, the
  `Refusal::exists_wording` shape), and the status line keeps the joint
  sentence while each button gets its own half.

The second is the one the census's rule points at — one composition,
parameterised — but it changes a `Refusal` arm's payload, so it is a
decision rather than a two-line edit.

## Whether the refusal is reachable, which does not change the verdict

The buttons are the only producers of `SessionOp::Undo` / `Redo`
(`app.rs:1499`, `:1505`), and a disabled egui widget reports no click
from a pointer, a keyboard or an AccessKit action alike — so
`Refusal::NothingToDo` may be unreachable through the chrome today, and
`frame.rs`'s tests construct `SessionOp::Undo` directly. **That makes
the case for showing the words stronger, not weaker:** if the only
reader who will ever meet this condition is the one looking at the grey
button, the button is where the sentence has to be.

## Home

`crates/viewer/src/app.rs` — owned by chrome, view and vseam; not
vnews's ground. Filed here because the subject is news vocabulary and
because `the-new-document-button-states-its-refusal-twice`, the row
eighteen lines up the same file, is already on this slate. A lane that
takes it announces the crossing.

## Done (2026-09-20) — answer 2, the direction is the payload

Taken with `the-new-document-button-states-its-refusal-twice` as one
unit; both are `crates/viewer/src/app.rs`, one rule and one shape of
fix. Branch `vnews/app-controls-read-their-refusals`.

**The row's two answers, and why the second won.** This file called
answer 2 *"a decision rather than a two-line edit"* and left it there.
It is the right answer and the argument is stronger than the row's own
tooltip-ergonomics one: **the joint sentence is reachably FALSE, and
not only at a button.** Undo at the root refuses while the redo the
cursor just left is live — `tests/undo_tree.rs`'s
`undo_at_the_root_and_redo_at_a_leaf_refuse_rather_than_wrap`
constructs exactly that state — and there the status line's *"nothing
to undo or redo"* denies a control the reader can see and click. So
answer 1 would have moved a false sentence onto a button rather than
composing a true one once, and the census's third obligation (*that
the sentence be true*) is not satisfied by either surface today.

That makes it this program's own charter case and not a tooltip
preference: the session works out WHICH direction had nothing and the
type throws the fact away.

**What landed.** `Refusal::NothingToDo { direction: Step }`, rendering
*"nothing to undo"* / *"nothing to redo"*; `Refusal::nothing_to_step`
as the one predicate `DocSession::step` refuses on and the two buttons
gate on, in the `Refusal::self_instance` shape the module already
carries; `DocSession::step` taking the direction instead of a bool.
`app.rs` draws the pair the way the cancel doors below them are drawn.

**The row's reachability premise was verified and holds.**
`grep -rn 'SessionOp::Undo\|SessionOp::Redo'` over `crates/`, `demos/`
and `tools/`: the only production producers are the two buttons; every
other hit is a test or an example. No keyboard route exists —
`consume_key` appears nowhere in the crate. So the refusal's sentence
was unreachable from the chrome entirely, which is the row's argument
for putting it on the button.

Asserted by `crates/viewer/tests/chrome_labels.rs`'s
`a_disabled_toolbar_control_says_what_its_own_operation_refuses`,
mutation-proven red three ways on a committed tree.

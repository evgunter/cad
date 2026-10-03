---
id: session-vocabulary-gained-step-and-nothingtodo-grew-a-payload
kind: issue
title: session::Step, a NothingToDo payload, and a new pub(crate) door in app.rs, from a VNEWS unit
status: open
opened: 2026-09-20
priority: P2
cost: E
---

**Notice, not a defect.** Filed so VSEAM meets this in a row rather
than in a merge: VSEAM has rows in flight on
`crates/viewer/src/app.rs` and `crates/viewer/src/session.rs`, and
VNEWS's `vnews/app-controls-read-their-refusals` (#2960) changed both,
plus the shape of a `Refusal` arm.

## 1. `app.rs` gained a `pub(crate)` door — `refusable_button`

```rust
pub(crate) fn refusable_button(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    blocked: Option<&Refusal>,
) -> bool
```

Live exactly while `blocked` is `None`, and out of it carrying that
refusal's own words as `on_disabled_hover_text`. It answers whether it
was clicked and keeps no opinion about what a click means.

**Four callers, in two programs' territory:**

- `app.rs`'s two cancel doors, its Undo and Redo, and its
  New-document Create — **VSEAM's ground**.
- `pane/create.rs`'s parts-catalogue entries — **VNEWS's ground**.

Each of those spelled the same three lines (`add_enabled`, then a
`match` on the refusal to attach the disabled text) before this. The
crossing was made deliberately and with the VNEWS orchestrator's
explicit preference for one function over a fourth copy of the idiom —
the opposite of the call recorded in
`app-rs-gained-a-toned-door-from-a-vnews-unit`, and recorded here for
the same reason that one is.

**What its visibility forecloses** is the same thing `toned`'s does:
`pub(crate)` puts it out of reach of `crates/viewer/tests/*`. That
costs nothing here, because the door's whole output is a painted
widget and the row that holds it
(`app.rs`'s `a_disabled_toolbar_control_says_what_its_own_operation_refuses`)
reads the painted frame from inside the crate. Widening it is a
decision about this file and therefore VSEAM's.

## 2. `Refusal::NothingToDo` carries its direction

`crates/viewer/src/session/refuse.rs` (VNEWS's ground, shared with
VSEAM) now spells it `NothingToDo { direction: Step }`, and its
`Display` renders *"nothing to undo"* or *"nothing to redo"* where it
used to render the joint *"nothing to undo or redo"*. Any VSEAM row
matching on that arm needs `NothingToDo { .. }` or a named direction.

The reason is a tooltip and not a status line: the two buttons are the
only producers of `SessionOp::Undo`/`Redo` and are disabled exactly
while the refusal is `Some`, so the sentence's only reader is one
button's tooltip — and a sentence naming both directions is false of
the button it is not about whenever the other direction is live.

## 3. `Step` is a new public name re-exported from `session.rs`

`pub use refuse::{NodeKindWanted, Refusal, Step, admits};`. It is NOT
in `crates/viewer/src/lib.rs`'s root re-export list, deliberately and
by the precedent `admits` sets there
(`work/vdoc/every-crate-root-reexport-is-a-second-path-not-the-only-one`
is the argument). If VSEAM wants the root spelling it is a decision
about `lib.rs`, which is VDOC's.

**Where `Step` lives was challenged and decided, so it is not
re-litigated silently.** The alternative on the table was
`history.rs`, folding `History::can_undo`/`can_redo` into one
`can_step(Step)` — the tell being that those two have exactly one
production caller each and both are inside `Refusal::nothing_to_step`.
It was declined on two grounds, and the second is the one that
carries it:

- `Step` is the `Refusal::NothingToDo` payload and `nothing_to_step` is
  its predicate, which is the clause `refuse.rs` already states for
  `NodeKindWanted`/`admits`. (Structural, and by itself weak: that type
  exists ONLY as a payload, and `Step` re-names a distinction two other
  vocabularies carry.)
- Collapsing the pair would replace two predicates that say which way
  they go, at **eleven** reading sites that know their direction
  statically, with one that makes every reader carry an argument — and
  those eleven sites are in `crates/viewer/tests/*`, which is VDOC's
  fence. The gain would be one production call site.

What `Step` earns its place with is that it is the direction as a
**value**: `SessionOp::Undo|Redo` and `History::undo|redo` are two
operations and two moves, and neither can be passed to anything. That
is what lets the direction-to-sentence map and the toolbar's two-row
table each be written once instead of twice.

## 4. `DocSession::step` takes the direction, and still fails loud

`fn step(&mut self, direction: Step)`, private, two call sites in
`perform`. **It reads the move's own `None`** and refuses on it, as it
did at the merge base; an earlier revision of #2960 replaced that with
a pre-check plus a `debug_assert!`, which would have turned a
release-mode disagreement between `can_step` and the move into a clean
`OpOutcome` for a step that never happened. The chrome's
`Refusal::nothing_to_step` composes the same refusal VALUE ahead of the
click, because a button has to decide whether to offer the move: one
composition of the words, two readings of the history, and the door's
is the one that acts.

## 5. `app.rs`'s toolbar history controls are one loop

`toolbar_ui` draws Undo and Redo from `[(label, direction, op); 2]`.
That table is the map this change introduces and it is now observed:
`a_disabled_toolbar_control_says_what_its_own_operation_refuses` lays
the real toolbar out headlessly, hovers each control, and reads the
words back off the painted frame, so a permutation of the direction
column reds.

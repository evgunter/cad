---
id: the-range-button-re-mints-the-ratified-affordance
kind: issue
title: The slot range button mints a third sentence for the condition Refusal::affordance is the one home of
status: open
opened: 2026-09-19
refs: [a-disabled-control-says-why-in-four-shapes]
---

Found by the census in `a-disabled-control-says-why-in-four-shapes`, at
merge base `2654cc111417da806d9786c40136106469096fec`. The sharpest
genuine hit of that census's rule, because the sentence it re-mints is
one whose single home is *documented as ratified*.

## What happens

`crates/viewer/src/pane/properties.rs`'s `Panel::range_button`
(`:759-773`) gates the probe control:

```
let offered = !row.driver.is_driven() && row.value.is_ok();
let button = ui.add_enabled(offered, egui::Button::new(label).small());
… else { button.on_disabled_hover_text("a computed slot has no range of its own to probe") }
```

`crates/viewer/src/session.rs`'s `Session::probe_bounds` (`:1487`)
refuses the same condition, and says in its own comment that it is the
same condition:

> A driven slot is not a field the user can put a number into, so a
> range of numbers for it is not an answer to any question they can act
> on: **the probe refuses it with the same affordance the write and the
> drag do**, which names the parameters to probe instead.

It does that through `guard_driven` (`session.rs:240`), which returns
`Refusal::DrivenByExpression`, whose `Display` is composed by
`Refusal::affordance` (`crates/viewer/src/session/refuse.rs:418`). That
helper's doc comment is explicit about being the only composition:

> "Dragging an expression-driven dimension → refuse, with an
> affordance" is a ratified micro-decision whose WORDING is part of the
> decision, so it is composed once and every surface that shows it —
> the status line, the inline note under the slot row — calls this.
> **Two independently-built copies is how the wording drifts from the
> decision.**

The range button is the third surface, and it does not call it. Worse,
the *same panel* already does: `Panel::slot_notes_ui`
(`properties.rs:735-743`, the `Refusal::affordance` call at `:741`) draws `Refusal::affordance(params, …)` for a
driven slot. So a reader looking at a driven slot sees the ratified
affordance on the row and a different sentence on the button beside it.

## The second arm, and what is and is not proved about it

`offered` has two conjuncts. The hover text speaks to one.

**What is proved, from the predicate alone.** `offered` is
`!row.driver.is_driven() && row.value.is_ok()` and the `else` branch is
taken whenever it is false. So *if* a row is reached with a literal
driver and an `Err` value, the reader is told *"a computed slot has no
range of its own to probe"* about a slot that is not computed. Nothing
refuses that condition either: `probe_bounds` guards only the driven
case, through `guard_driven`. The sentence would be false there, and
the arm would not be the census's class at all.

**What is NOT proved, and is stated here as unproven rather than
asserted.** That such a row is reachable. `SlotRow::value` is an
`Err` when the slot did not evaluate, and whether the panel ever draws
a `range?` button for a literal slot in that state depends on what
`DocSession::slot_rows` produces and on what the panel does above the
button — neither of which this census walked. The claim is therefore:
**the predicate admits the state; no witness is exhibited.** A lane
taking this row owes the witness first, because the repair differs —
a reachable state needs its own true sentence, an unreachable one needs
the two conjuncts told apart so the code cannot start lying later, and
the second is worth doing either way but is a smaller change.

**Do not reuse `properties.rs:353`'s words for it.** An earlier draft
of this row offered *"no evaluation yet to resolve this against"* as
the neighbour that *"already says [it] for the same state"*. It does
not: that sentence is in `entity_standing_ui`'s `resolution` match and
is about a standing FACE or EDGE with no evaluation to resolve its
`StableName` against — a different subject from a slot whose
expression failed to evaluate. The words for this arm, if it is
reachable, are unwritten.

## What a fix does

- Driven arm: call `Refusal::affordance(params, row.value.as_ref().ok().copied())`,
  the same call `slot_notes_ui` makes fifty lines up, so the button and
  the row and the status line are one composition.
- Errored-value arm: first a witness that it is reachable; then a
  distinct, true sentence, newly written.
- The two arms have to be told apart to do either, which is the whole
  change: `offered` collapses them today.

## Home

VNEWS's: `crates/viewer/src/pane/properties.rs` (double-claimed with
chrome, vgeom and view). `crates/viewer/src/session/refuse.rs` is read,
not edited — `Refusal::affordance` already has the shape this needs.

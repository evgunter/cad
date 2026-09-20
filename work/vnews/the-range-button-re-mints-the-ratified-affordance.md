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
(`properties.rs:718-726`, the `Refusal::affordance` call at `:724`) draws `Refusal::affordance(params, …)` for a
driven slot. So a reader looking at a driven slot sees the ratified
affordance on the row and a different sentence on the button beside it.

## The second arm, which is a plain falsehood

`offered` has two conjuncts. The hover text speaks to one.

When `row.driver` is a **literal** and `row.value` is `Err` — an
evaluation that failed — the button is disabled and tells the reader
*"a computed slot has no range of its own to probe"*, about a slot that
is not computed. Nothing refuses that condition either: `probe_bounds`
guards only the driven case. So this arm is not the census's class at
all; it is a sentence that is simply wrong, and it needs its own words
("there is no evaluation to probe against", or whatever the panel's
neighbour at `properties.rs:353` already says for the same state).

## What a fix does

- Driven arm: call `Refusal::affordance(params, row.value.as_ref().ok().copied())`,
  the same call `slot_notes_ui` makes fifty lines up, so the button and
  the row and the status line are one composition.
- Errored-value arm: a distinct, true sentence.
- The two arms have to be told apart to do either, which is the whole
  change: `offered` collapses them today.

## Home

VNEWS's: `crates/viewer/src/pane/properties.rs` (double-claimed with
chrome, vgeom and view). `crates/viewer/src/session/refuse.rs` is read,
not edited — `Refusal::affordance` already has the shape this needs.

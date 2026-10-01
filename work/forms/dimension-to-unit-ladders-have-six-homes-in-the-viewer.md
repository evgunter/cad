---
id: dimension-to-unit-ladders-have-six-homes-in-the-viewer
kind: issue
title: Six Dimension-to-unit ladders in the viewer, each answering which dimensions have a notation
status: open
opened: 2026-09-21
priority: P1
cost: D
---


## Finding

Found by AUTH-2's style reviewer (AUTHOR, PR 2957, 2026-09-21),
confidence `sure`, and **it is where that unit's own sweep said it
could not look**. The unit swept unit-symbol literals, `factor()` and
the `written_*` doors and found the crate clean; it then named as a
blind spot *"a unit vocabulary reached through a `match` on
`Dimension` with no symbol text in it"* — and that is exactly the
shape the duplication has.

Six sites in `crates/viewer/src` each answer *which dimensions have a
notation, and what it is*, with their own `Length / Angle / Scalar |
Count` ladder:

- `props::unit_options`
- `props::rendering_unit`
- `props::doc_param`
- `properties::new_param_unit`
- the unit picker's `match` in `pane/properties.rs`
- `forms::drag_tick`

plus a seventh shape in `sketch.rs`. **Three of them were added by
AUTH-2's own diff**, which is why this is filed now rather than
whenever someone noticed.

## The sentence that shows the rule has no home

`param_unit_ui`'s doc (added by that unit) says `props::unit_options`
*"is the one place that answers which those are"*. It is not, and the
same diff added three of the places it is not. A comment asserting a
single home, written beside code that contradicts it, is
`docs/prompts/reviewer-style-lane.md` Q2's shape and is usually the
only evidence the rule wants one.

`properties::new_param_unit`'s *"One answer read by three places"* is
the same error one level down: it is two places and a parallel ladder,
because the picker re-derives the dimension mapping instead of calling
it. They can disagree, and that sentence is the reason nobody will
check.

## Why P1 and why CHROME

`work/README.md`'s P1 band — two implementations of one underlying
logic, entrenching as things are built on it. Every new form adds a
ladder; three arrived in one unit. The files are CHROME's ground
(`pane/properties.rs`, `props.rs`, `forms.rs`) and the class is the
entrenching-architecture half CHROME kept at the 2026-09-20 cut.

AUTHOR's AUTH-2 fix pass is told not to add a seventh and to say which
of its three are instances of this row; it is not told to close the
class, which is wider than that unit.

## Where else to look

The reviewer's sweep suggestion: `Dimension::Scalar | Dimension::Count`
and `Dimension::Length =>` across `crates/viewer/src`. Two rows already
on this slate are the same SHAPE with different subjects —
`a-fifth-spelling-of-this-seat-is-empty` and
`four-spellings-of-one-finiteness-predicate-in-datums-rs` — and whether
any of them want one unit is CHROME's call.

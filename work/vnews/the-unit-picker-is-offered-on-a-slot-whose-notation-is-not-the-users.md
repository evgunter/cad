---
id: the-unit-picker-is-offered-on-a-slot-whose-notation-is-not-the-users
kind: issue
title: the slot unit picker is enabled on a driven slot and SetSlotUnit refuses the pick
status: open
opened: 2026-09-20
refs: [a-disabled-control-says-why-in-four-shapes, the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give]
priority: P3
cost: E
---

Found by the sweep of `vnews/properties-controls-read-their-refusals`
(the hide-toggle and range-button unit), which swept
`crates/viewer/src/pane/properties.rs` for **a control whose enabling
condition is not the condition the op it pushes refuses on**. It is the
mirror class's second member in that file, after the hide toggle.

## What happens

`ViewerBehavior::slot_unit_ui` draws a `ComboBox` for every slot row
whose dimension has units, with no test on the row's driver, and a pick
pushes `SessionOp::SetSlotUnit`.

`DocSession::set_slot_unit` runs `props::slot_unit_edit`, which refuses
`SlotUnitFault::NotALiteral` for a slot whose expression is not a bare
literal — i.e. for exactly the rows `SlotDriver::is_driven` is true of.
`set_slot_unit`'s own doc states the refusal and why it is the narrow
one: *"the driven refusal protects a computed slot from being
overwritten with a NUMBER, and this op writes no number. What a driven
slot refuses is the narrower `SlotUnitFault::NotALiteral` the panel
model raises — an expression has no authored notation to change."*

So the picker on a driven slot is enabled, opens, and answers the pick
with a refusal.

## Stated so it is not overstated

There IS a sentence on screen for a driven row: `slot_notes_ui` draws
`Refusal::affordance` under it. But that sentence is about the VALUE —
*"driven by an expression over t (currently 5) — edit the expression?"*
— and this control does not write a value. A reader is told the number
is not theirs to move and left to infer that the notation is not theirs
to choose either, which is a different fact with its own refusal and
its own words.

## The shapes

- **Gate the picker on the driver and carry `SlotUnitFault::NotALiteral`'s
  sentence**, the shape the hide toggle just took. Straightforward for
  `SlotGroup::Scalar`.
- **The vector case is what makes this more than an E.** One picker
  serves three component rows and writes the unit to all three —
  `slot_unit_ui`'s ONE-picker rule: *"three components of a point are
  written in one unit or the user is being told something they did not
  mean to say"*. **That sentence is a code comment and nothing more**:
  it occurs once in the tree, in `slot_unit_ui`, and zero times in
  `crates/viewer/GUI-DESIGN.md`, `docs/DESIGN.md` and
  `crates/viewer/README.md`. An earlier draft of this row called it
  *"itself ratified reasoning"*, which was the same unchecked claim
  `ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
  is the census of; it is good reasoning and it is not a ratification,
  so a row that departs from it does not thereby need Ev. A family with one
  driven component and two literals has no single answer to "is this
  control usable" — so the row has to decide whether the picker is
  refused for the family, or applies to the components it can and says
  what it skipped. That decision is the work here, not the gate.

## Home

VNEWS's: `crates/viewer/src/pane/properties.rs`, double-claimed with
VGEOM and written on both sides, so a change there announces. The
refusal's words already have one home (`SlotUnitFault`'s `Display`);
nothing in `props.rs` needs to move.

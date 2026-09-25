---
id: probe-scale-refuses-a-no-readable-value-case-its-only-caller-pre-empts
kind: issue
title: probe_scale's NoSuchSlot covers a value-less slot that guard_driven and the dimension doors both pre-empt
status: open
opened: 2026-09-20
refs: [the-range-button-re-mints-the-ratified-affordance]
priority: P3
cost: E
---

Found by `vnews/properties-controls-read-their-refusals` (PR #2961),
whose unreachability argument for the range button's second conjunct
implies something about this arm. `crates/viewer/src/session/probe.rs`
is VSEAM's, so the finding is filed rather than acted on.

## Two sites, opposite beliefs about one condition

`probe::probe_scale`'s slot arm collapses two misses into one refusal
and says the collapse is sound because both are real:

> One refusal for both misses — the node does not carry the slot, and
> the slot carries no readable value — because a probe needs a place to
> search FROM and neither case gives it one.

PR #2961 argues, and its tests hold, that **a slot the document holds a
bare literal for always evaluates**: `SlotDriver::of` answers `Literal`
only for a leaf with no parameter reference, the only `EvalError` such
a leaf can raise is the Count/continuous divide, and
`Node::slot_dimension_fault` — asked by the edit doors and by the load
walk alike — refuses an expression whose dimension disagrees with its
slot's.

Put beside `probe_bounds`'s own guard, the second miss has no way in
through this door's only caller:

- a **literal** row cannot have an `Err` value, by the above;
- a **driven** row with an `Err` value never reaches `probe_scale`,
  because `DocSession::probe_bounds` runs `guard_driven` first and
  refuses it with the affordance.

So the *"carries no readable value"* half is dead for
`SessionOp::ProbeBounds`. The *"node does not carry the slot"* half is
a different matter and is defensible: the door takes a `BoundsTarget`
from the operation vocabulary and nothing in its type says the slot is
one the node has.

## Why this is worth a row rather than a deletion

The two sites now hold opposite beliefs about one condition, and
whichever is wrong, that is the defect. Three outcomes are open and the
row exists to pick one:

1. The argument is right and the arm is genuinely unreachable — then
   the comment should say which caller pre-empts it, not that the case
   is real, and the collapse should be justified by the reachable half
   alone.
2. There is a caller or a route PR #2961 did not consider — then the
   panel's dropped conjunct needs re-examining too, and this row is the
   thing that catches it.
3. `probe_scale` wants to keep the arm as a defensive refusal for a
   `pub(super)` door that could gain a second caller — a fine answer,
   and one that should be written as that rather than as a claim about
   documents that can exist.

**Do not silently delete the arm.** A refusal removed on an argument
made in another file is exactly the shape that comes back as a panic.

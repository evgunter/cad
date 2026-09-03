---
id: split-side-and-pattern-instance-as-operand
kind: issue
title: A split's side and a pattern's instance cannot be an operand - the recipe has no way to name one
status: open
opened: 2026-08-31
github: 1394
---

## From GitHub issue 1394

Opened 2026-08-31; 0 comments.

## What

The Phase-B combining doors (`SessionOp::AddBoolean` / `AddSplit` / `AddTransform` / `AddPattern`) gate every body seat on "this node's value is ONE body" — `viewer::combine::denotes_body`, which **tracks** the evaluator's single-body operand door (`editor_core::eval::wire::body_operand`) rather than restating a set. `Node::Split` (two role-tagged sides) and `Node::Pattern` (N unfused instances) are therefore refused at the seat, typed:

```
node 7 is not a body in this document
```

That is the honest answer today, and the refusal is better than the alternative (the same mismatch arriving as a failed node after the edit lands). But it names a real modelling gap rather than a mistake: **"union the upper half of that split into this block" and "subtract instance 3 of that pattern" are ordinary CAD sentences and there is no way to spell either.**

## Tracks, not equals — the two known divergences

Stated exactly, because an earlier version of this issue said "exactly the evaluator's set" and that was wrong in a live case:

- **`Sweep` is admitted by the seat and evaluates to nothing at all.** It is the curved-solid frontier (`wire_sweep` refuses every recipe-expressible sweep today), so the door never gets asked. A seat that refused it would answer "that is not a body" to a node that is one in every sense but the one the kernel has reached; its own frontier refusal is the honest diagnosis and arrives by poison propagation.
- **An admitted kind can still refuse downstream.** An empty boolean result is a typed success that is not a body (`EmptyOperand`), and a lofted NURBS body meets the rigid transform's own placement frontier.

Both directions are now asserted rather than promised: `crates/viewer/tests/combine_ops.rs::the_body_seat_tracks_the_evaluators_operand_door` feeds a minimal instance of fourteen of the eighteen node kinds into a real `Node::Transform` — whose operand IS `body_operand` — and asserts admitted ⇒ not refused as an operand, refused ⇒ refused as an operand, with the sweep exception asserted by name. (`Mate`, `Measure`, `Assertion` and `InstantiatePart` are the four it does not build; the row says why.)

## Why it is not a doors bug

The recipe vocabulary has no operand that selects *part* of a multi-body value. `body_operand` says so in its own words:

> A single-body operand: a Body value, or a boolean's non-empty result. Splits and patterns need PR 3's naming layer to select a part — typed refusal, not a guess.

So the fix is upstream of layer 3: either an operand that carries a part selector (a `SplitSide` role, a `RoleSeg::Instance` index — both already exist in the naming vocabulary), or a node that projects one part out of a multi-body value. Once one of those exists, the doors' seat gate widens at one site (`denotes_body`) and the tools need no change: they hold node picks and judge no kinds.

## Where it is pinned today

`crates/viewer/tests/combine_ops.rs::several_bodies_are_not_one_body_at_a_seat` asserts the current refusal for a split node and a pattern node at all three body seats. That row is the thing to rewrite when this is answered — it is a receipt for today's behaviour, not an argument for keeping it.

## Not in scope of the unit that found it

GAUTH-4 authors the four combining doors as the plan specifies them ("two sequential body picks"). Widening what a body pick may be is a vocabulary decision, not a chrome one.

## Home

GAUTH's closing entry names this issue as its residue; the fix is a recipe-vocabulary decision above `editor_core::eval::wire::body_operand`, which sits in no open program's territory (M10 fences editor-core eval outside its analysis lane), so it lands in `work/issues/`.

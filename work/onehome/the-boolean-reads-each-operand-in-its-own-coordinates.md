---
id: the-boolean-reads-each-operand-in-its-own-coordinates
kind: issue
title: The boolean reads each operand in its own coordinates: a fold no longer rewrites carriers, and only cross-operand readings go through the relative motion
status: open
opened: 2026-11-16
priority: P1
cost: H
refs: [7006, a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs]
---


Decision 2 of fork 274 (PR 7006, merged), built against `docs/DESIGN.md`
[placement.frame] as now worded: "An operand in another frame is read
into it through their relative placement, which moves where the operand
stands and none of its data: a reading within one operand is made in its
own coordinates, and only a reading between two goes through their
relative motion."

## What changes

- **The fold stops rewriting carriers.** Today a fold reads every other
  operand into one member's frame by moving its body:
  - `Framed::read_in` (`crates/editor-core/src/eval/framed.rs:394`)
    takes the relative map and moves the operand's body through `Held::moved`;
  - that calls `topo`'s `FinishedBody::moved_by` / `LeafBody::moved_by`
    (`crates/topo/src/finish.rs:144`, `:233`), which rewrite every carrier.
  
  The boolean reads each operand in its own coordinates instead. Only
  the readings between two operands go through their relative motion:
  sections, classification across operands, the `⊔` census.
- **Readings within one operand stay in its own coordinates.** A
  reading of a single operand (its own facts, tiers, measures) moves
  none of its data. A finished input's facts therefore stay bit-identical
  through the operation.

## Closes with it

`a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs`.
An operand that is no longer rewritten keeps the data its facts were
derived on, so the later finish can carry them.

## Done when

No fold moves a body through `moved_by` to read it, cross-operand
readings take the relative motion at the reading, and a Boolean's
untouched operand keeps its facts.

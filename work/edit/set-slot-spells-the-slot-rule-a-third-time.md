---
id: set-slot-spells-the-slot-rule-a-third-time
kind: issue
title: set_slot restates both arms of the slot rule a third time, and the refusal SENTENCE lives at three sites
status: closed
opened: 2026-09-16
branch: edit/one-predicate-round-three
pr: 2780
closed: 2026-09-16
---



Found by the style review of `edit/one-predicate-round-three` (PR
#2780), which gave the slot rule one home
(`Node::slot_dimension_fault`, `crates/editor-core/src/node.rs:3166`)
and taught the load door to ask it.

**The third spelling of the PREDICATE.** `crates/editor-core/src/edit.rs:2699`
`set_slot` restates BOTH arms of `SlotDimensionFault`, not one
comparison:

- `crates/editor-core/src/edit.rs:2707` — `node.expr(slot).is_none()`
  → `EditError::UnknownSlot`, which is the `MissingExpression` arm;
- `crates/editor-core/src/edit.rs:2711` — `expr.dim() != slot.dimension()`
  → `EditError::SlotDimensionMismatch`, which is the `Mismatch` arm,
  including the `expected: slot.dimension()` payload.

The PR body discloses this ("`edit.rs`'s `set_slot` compares
`expr.dim() != slot.dimension()` a third time — **not this unit.** It
asks the rule of an expression the node does not hold yet … what is
left after the difference is one comparison"). Its premise is that the
difference is the SUBJECT — a candidate expression rather than a node
— which is true, and its conclusion that only one comparison is left
is not: the vocabulary bug arm is restated too, and it is the arm the
unit argued hardest about. A predicate over `(SlotId, &Expr)` that
`Node::slot_dimension_fault` itself calls per slot would hold both.

**The refusal SENTENCE, at three sites.** `"slot {} needs {} {expected}
expression, got {} {found}"` is written verbatim at
`crates/editor-core/src/edit.rs:1238`
(`EditError::SlotDimensionMismatch`) and twice more at
`crates/editor-core/src/persist/check.rs:785` and `:799`
(`SnapshotError::SlotDimension`'s profile-address arm and its general
arm). The round-two fix pass already solved exactly this shape one
rule over — `Frame::admission_fault`'s `Display` is "one predicate
CLAUSE, so every door that refuses a frame forwards the same sentence
into its own subject" — and the pattern was not applied here.

The two doors also spell a PROFILE slot address differently for the
same fact: the load door renders `loop 0 step 2's centre-x argument`
(`check.rs:785`) and the edit door renders `SlotId::label`'s
`loop 0 step 2 · centre x` (`node.rs:539`). One fact, two sentences.

**Not a correctness defect**: both doors refuse the same documents,
which the round-three rows pin. This is the Q1 residue the unit's own
move leaves behind.

## Closed (2026-09-16, `edit/one-predicate-round-three`'s fix pass)

**The predicate.** `SlotId::dimension_fault(self, &Expr)` is D6's
comparison over one address and one candidate expression, and it is
the only place the comparison is written: `Node::slot_dimension_fault`
asks it per slot of a node the document holds, and `set_slot` asks it
of an expression the node does not hold yet. The vocabulary-bug arm is
not restated because it no longer exists — `slots()` is `expr()`'s
domain, asserted at the site and pinned for every node kind
(`switch_slots::every_node_kinds_slots_are_all_readable`), so what
`set_slot` still decides for itself is whether the node HAS the slot a
caller named, which is its own question and a reachable mistake.

**The sentence.** `SlotDimensionFault`'s `Display` is the one clause,
forwarded into each door's subject — `EditError::SlotDimensionMismatch`
renders it bare, `SnapshotError::SlotDimension` under "node 7: " —
the `Frame::admission_fault` shape round two built. The load door's
separate spelling of a PROFILE slot address is gone with it: both
doors render `SlotId::label`, so one fact reads as one sentence.
`display_contract::a_slot_refusal_addresses_its_slot_in_the_slot_vocabulary`
asserts the two renderings against each other for three slot shapes.

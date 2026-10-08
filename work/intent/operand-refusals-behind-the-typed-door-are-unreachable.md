---
id: operand-refusals-behind-the-typed-door-are-unreachable
kind: issue
title: Typed operand slots refuse a wrong-kind read at the door, so eval's WrongOperand kind arms and AssertionTarget are defences no document reaches
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [operands-are-reads, measure-is-an-operation]
---

Found by INTENT stage 2 unit B (`operands-are-reads`). Every operand
slot admits one `SlotKind` (`crates/editor-core/src/operand.rs`,
`OperandSlot::kind`), and both the edit door (`lower_operand`,
`crates/editor-core/src/edit.rs`) and the load walk
(`first_operand_read_fault`, `crates/editor-core/src/persist/check.rs`)
refuse a read of another kind. So the evaluator's kind refusals behind
them are reached by no document:

- `wire_operand_door.rs` now asserts 15 of its 18 miswirings as the
  edit door's `SlotVarKind`; the `WrongOperand` arms that built
  their `expected`/`found` pairs (`eval/wire.rs`'s operand door) are
  defences. Three remain reachable, where the kind is right and the
  value is not (a 3-D axis at a revolve, a half or an instance of a
  plain body).
- `EditError::AssertionTarget` and `SnapshotError::AssertionTarget`:
  the `Measure` slot admits `SlotKind::Measured`, and only a
  `Measure` defines a measured value, so a non-measure target refuses
  `SlotVarKind` first at both doors. Unit D restates the assertion
  over a scalar variable and is where the arm retires or changes
  meaning.
- `mate/member.rs`'s recipe-side family word for a circular rule's
  axis (the four `msolve3_placer_refused` rows that compared it with
  the twin's now assert the door refusal instead).

Whether each becomes an `unreachable!` with its reason, retires, or
stays as a defence is a sweep with a hit list; the census in
`wire_operand_door.rs` counts which rows still reach evaluation.

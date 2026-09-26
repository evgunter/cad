---
id: node-slot-tables-are-spelled-three-times-in-node-rs
kind: issue
title: Node, TubeWindow and the placement rule each spell their slot roles three times in node.rs - slots(), expr() and a hand-copied expr_mut() - where program.rs now reads one role table
status: open
opened: 2026-09-25
priority: P1
cost: D
refs: [step-arg-roles-are-spelled-in-three-homes]
---


(GATHER lane, `gather/step-arg-roles-one-home`.) Found by the sweep for
the class "one role vocabulary spelled per consumer", which that PR
reduced to one role table (`loop_roles`) for `StepArg` in
`crates/editor-core/src/program.rs`.

Each of the three `SlotId` tables in `crates/editor-core/src/node.rs`
decides which slot each field of a shape carries, and each of them
writes that decision three times:

- **`Node`**: `Node::slots` (the enumeration, about `:2994`),
  `Node::expr` (`:3096`) and `Node::expr_mut` (`:3195`). The last two
  are two independent matches of about 100 lines each that differ only
  in `comp` against `comp_mut`.
- **`TubeWindow`**: `TubeWindow::slots`, `TubeWindow::expr` and
  `TubeWindow::expr_mut` (about `:975-998`).
- **The placement rule**: `rule_slots`, `rule_expr` and `rule_expr_mut`
  (about `:2564-2605`). `rule_expr_mut`'s doc says "same mapping", which
  is a claim nothing checks.

Resolution does not add a fourth home here. `eval::slots::eval_slots`
walks `Node::slots` and reads each slot through `Node::expr`, so it
consumes the addressing table and does not restate it.

What guards the three now:
`switch_slots::every_node_kinds_slots_are_all_readable` checks that
`slots()` is inside `expr()`'s domain for one node of every shape.
No census walks `expr_mut` over every shape. It is reached only through
`DocEdit::SetParam` tests, and only for the slots those tests happen to
edit, so nothing checks that `expr_mut` addresses the field `expr` does
for each slot. A transposed arm in
`Node::expr_mut` (for example `U` writing to `v`) would pass every
reader and misdirect every edit.

The pattern `program.rs` now uses carries over. One borrow-generic
macro per shape lists `(SlotId, field)` rows, and match ergonomics bind
`&` or `&mut` from the same text. Then `slots`, `expr` and `expr_mut`
all read those rows. `comp` and `comp_mut` would become an `[x, y, z]`
array pattern. `work/wire/two-verb-seats-do-not-compose.md` names the
wider family of node traversal doors (item (a)). This row is only the
slot-table part of it, which can be fixed without the verb-seat design.

---
id: node-slot-tables-are-spelled-three-times-in-node-rs
kind: issue
title: Node, TubeWindow and the placement rule each spell their slot roles three times in node.rs - slots(), expr() and a hand-copied expr_mut() - where program.rs now reads one role table
status: review
branch: edit/slot-tables-one-home
opened: 2026-09-25
priority: P1
cost: D
refs: [3264]
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

## Ruled and spec'd (2026-09-29, EDIT orchestrator) — middle tier, one opus style review, branch `edit/slot-tables-one-home`

The row's own remedy is the ruling. For each of the three shapes
(`Node`, `TubeWindow`, the placement rule), one borrow-generic list of
`(SlotId, field)` rows is the only place the mapping is written, and
`slots`, `expr` and `expr_mut` all read it. Match ergonomics bind `&`
or `&mut` from the same text, as `program.rs`'s `loop_roles` does for
`StepArg`. `comp`/`comp_mut` become one array pattern. The "same
mapping" doc on `rule_expr_mut` goes, because nothing is left for it to
claim.

1. **The guard the row says is missing.** A census walks every shape
   and every slot and asserts that `expr_mut` reaches the same field
   `expr` reads, for example by writing a sentinel through `expr_mut`
   and reading it back through `expr`. A transposed arm (`U` writing
   `v`) must red it: plant that mutant once, record which row reds,
   revert.
2. **No behaviour moves.** No stored bit or golden should move. If one
   does, root-cause it before re-baselining.
3. **Not in scope:** the wider verb-seat family
   (`work/wire/two-verb-seats-do-not-compose.md` item (a)).

## Built (2026-09-29)

Each of the three shapes has one borrow-generic row list in
`crates/editor-core/src/node.rs`, and `slots`, `expr` and `expr_mut`
all read it: `window_rows!` (read by `TubeWindow`'s three doors and by
both tube kinds), `rule_rows!` (`Node::Pattern` and
`Node::PlacedUnion`) and `node_rows!` (every other node kind, with
`tube_rows!` holding the head both tube kinds share). `comp`,
`comp_mut`, `comp2` and `comp2_mut` became one `axis_rows!`, which zips
a 3- or 2-vector with `Axis3::ALL`. The profile arm still delegates to
its payload, whose table is `program.rs`'s `loop_roles`. `rule_expr_mut`'s
"same mapping" doc went with the function.

The guard is
`switch_slots::every_node_kinds_expr_mut_writes_the_field_expr_reads`.
It writes a sentinel through `expr_mut` at every slot of every node
shape, reads it back through `expr`, and checks that no other slot
moved. Across the union of every shape's slots, it checks that
`expr_mut` answers exactly where `expr` does. A transposed `U` arm reds
it, both in the old `Node::expr_mut` and as a remap in the new one.

No slot order and no stored bit moved: every shape's `(slot, expr)`
list was dumped before and after the change, and the two dumps are
identical.

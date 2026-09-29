---
id: node-slot-tables-are-spelled-three-times-in-node-rs
kind: issue
title: Node, TubeWindow and the placement rule each spell their slot roles three times in node.rs - slots(), expr() and a hand-copied expr_mut() - where program.rs now reads one role table
status: closed
closed: 2026-09-29
branch: edit/slot-tables-one-home
pr: 3438
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

Each shape has one borrow-generic row list, and every door reads it.
In `crates/editor-core/src/node.rs`: `node_rows!` (every node kind),
with `rule_rows!` (`Node::Pattern` and `Node::PlacedUnion`),
`tube_rows!` and `window_rows!` (both tube kinds) and `axis_rows!`
(every 3- or 2-vector, held to that arity at compile time) under it.
The profile arm reads `ProfilePayload::rows`, which is keyed by
`(loop, step, arg)`, so a profile node answers only `SlotId::Profile`
addresses. In `crates/editor-core/src/program.rs`: `loop_roles!` (a
whole loop, keyed `(step, role)`) and `program_rows!` (a program, over
its loops). `row_readers!` writes each `rows`/`rows_mut` pair
(`Node`, `LoopProgram`, `ProfileProgram`), and `find_row` is the one
lookup in both files. `Node::slots`, `expr` and `expr_mut` are each
one line over `Node::rows`, and `slot_dimension_fault` reads the rows
directly, so it has no unreachable arm. `TubeWindow`'s uncalled doors
went.

The guards, all in `switch_slots`:
- `every_node_kinds_expr_mut_writes_the_field_expr_reads`: a sentinel
  written through `expr_mut` at every slot reads back through `expr`,
  and no other slot moves. A transposed `U` arm reds it.
- `every_node_kinds_slots_are_all_readable`: also checks that `slots()`
  lists no slot twice.
- `every_node_shapes_slot_table_is_pinned`: the golden
  `crates/editor-core/tests/golden/slot_tables.txt` holds every shape's
  slots in order and the field each one addresses (a distinct tag per
  slot, written through `expr_mut`). It reds on a same-dimension field
  swap (Sweep `Stations`/`v_degree`) and on an order-only swap
  (Transform translation/rotation axis), which every other row passes.
  It is green on the pre-change sources, so no slot order or mapping
  moved.

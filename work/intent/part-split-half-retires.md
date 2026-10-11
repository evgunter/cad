---
id: part-split-half-retires
kind: issue
title: A split's port is read as its half, so DM3's Part { SplitHalf } projection retires; retiring it moves roots, so a later unit than B does it
status: closed
opened: 2026-10-08
priority: P2
cost: M
branch: intent/part-split-half-retires-2
pr: 4618
closed: 2026-10-11
refs: [operands-are-reads, operations-state-their-outputs]
---

FORK-1 (Ev, #4222): "a split defines two bodies; DM3's split-half
projection retires". Unit B (`operands-are-reads`) made a read of a
split's port that half: evaluation projects it exactly as
`Part { SplitHalf }` does (`eval/wire.rs`, `split_ports_projected`,
sharing `split_side` with `wire_part`), and
`intent_s2_b_reads::a_split_port_read_is_its_half` pins the two
spellings equal. `Part { SplitHalf }` stays in B, because retiring it
moves roots: a `Part` is a node, and a root list that holds one would
hold the split's port instead (orchestrator's ruling on Q2,
2026-10-08).

The work: retire `PartSelect::SplitHalf` (`crates/editor-core/src/node.rs`)
and its arm of `wire_part`, the load walk's `PartHalfPort` and the edit
door's twin, Python's `PartSelect.split_half` and the viewer's part
tool's split seat; rewrite REFERENCES DM3's text to its final form; and
regenerate the corpus documents that author a half as a `Part`
(`corpus/part_select.rs`, the die's) as port reads, stating what moves.

## Closed

PR 4618. `PartSelect::SplitHalf` retires with its doors, tags, Python
spelling and the viewer's split seat; a body seat holds a read, so a
half is read by its port everywhere. A measure reading both halves of
one split projects each reference on its own (`run_op`'s
`per_operand`), and a node reading two bodies of one operation into one
map is a kernel bug that fails loud.

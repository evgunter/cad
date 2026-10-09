---
id: a-name-is-scoped-by-the-variable-holding-its-body
kind: unit
title: A name is scoped by the variable that holds its body: name tables and the evaluator's results per output variable, and a split's roles drop the half
status: parked
opened: 2026-10-09
priority: P0
cost: H
needs_ev: true
blocked_on: [a-union-member-is-keyed-by-its-read]
refs: [part-split-half-retires, operands-are-reads]
---


The second unit of FORK-DM4 (`a-union-member-is-keyed-by-its-read`),
after the read key lands. NAMES N1 ("the scope"): a name is scoped by the
variable that holds its body, one name table per output variable. Built
evidence: the split's half segments and the pattern's `Instance { i }`
segment exist only to keep one node's table distinct by restating the
output. D10 text not yet built: `X'`, a copy of `X` placed against `X`,
takes `X`'s root, and `Place [X, X']` defines two copies with identical
tables (a copy keeps its rows, N1), which a per-node `NameTable` refuses
as `DuplicateName`; so this lands before that placement is built.

The work:

- `NodeValue.name_table` becomes one table per output variable, and the
  evaluator's `Results` is keyed by the read (`VarId`), not the node.
  The per-operand overlays that project a split's ports
  (`split_ports_projected`, the overlays in `run_op`) retire with it; this
  is the target `part-split-half-retires` already names, so the two are
  built together or this one after it.
- A split's roles drop the half: `SplitBody(half)` becomes a plain body
  role, `SectionFace { side, section }` → `SectionFace { section }`, and
  likewise `SectionEdge`, `SplitFragment` and `CrossingVertex`. The
  intra-table qualifiers (`Keeps`, `Ends`) stay. The half is said once, by
  the variable a selection states or the read a `From` carries.
- N5 still diagnoses `PredicateFlip` from the split's recorded
  classification: a piece whose side flips when the plane moves leaves
  its variable's table rather than renaming, so the diagnosis cannot be
  read off the name (NAMES N2, the Split op's pieces). Pin it with a
  test that moves the plane across a piece.
- A part instance's per-output qualifier ("qualified by the copy") is the
  same rule; read `eval/parts.rs` before building and drop it if it only
  restates the output.
- What moves: every name with a split in its ancestry (the half dropped;
  `name_words_corpus`'s "the body above" changes its words), re-baselined
  and migrated at load by the same total map the first unit adds.

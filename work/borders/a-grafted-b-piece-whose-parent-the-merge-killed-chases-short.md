---
id: a-grafted-b-piece-whose-parent-the-merge-killed-chases-short
kind: issue
title: A grafted B piece whose parent the boolean's merge killed: chase_b reads no output-stage hops, so its lineage stops at the dead parent
status: open
opened: 2026-10-14
priority: P3
cost: E
---



## What

Found in review of the fix for
`a-booleans-merge-stage-kills-split-records-no-row-copies.md` (closed).
That fix made `emit_topo::KilledSplits` read
`BooleanNaming::killed_lineage` into its `arena` hops. These are hops in
result keys, which `chase_edge_to_table` reads for an operand whose
keys are the result arena's (`KeySpace::Arena`).

A grafted B side goes through `chase_b` in `emit_topo::name_edges`
(~`emit_topo.rs:1654`). It walks the result body with
`Body::split_root`, stopping at the first key with no graft row, and
then reads `killed.graft_b`, which holds hops in B's clone keys. Neither
step reads the output stage's result-key hops. So a grafted B piece
whose parent the merge killed this step still stops at the dead parent.

Not measured: the full-suite probe behind the closed row found no chase
ending at a merge-killed parent once the fix was in. It did not
separate the graft lane, and no fixture is known to reach the case.
What follows the short stop is not known either: the
`a grafted edge's split lineage left the graft rows` refusal, or a
wrong root.

## Next

- Build a witness: a subtract or intersect whose B is grafted and whose
  merge kills a B edge piece that has a live child.
- Then have `chase_b`'s result-body walk step through
  `killed_lineage` before it reads the graft rows, as
  `chase_edge_to_table` steps through `killed.arena`.

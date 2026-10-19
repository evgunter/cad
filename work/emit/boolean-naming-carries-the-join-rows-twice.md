---
id: boolean-naming-carries-the-join-rows-twice
kind: issue
title: BooleanNaming carries the output join rows twice, as edge_joins and folded into substitutions, and a hand-built naming can make them disagree
status: open
opened: 2026-10-12
priority: P3
cost: E
design: true
refs: [4868]
---


Found by the orchestrator's delta check of PR 4868
(`fuse/face-lineage-at-append`).

PR 4868 made the settler (`BooleanNaming::settler`,
`crates/topo/src/boolean/ops.rs`) chase one checked list,
`BooleanNaming::substitutions`: the merge's lineage, then each output
join's joined-away vertex and edge onto its kept edge, built by
`output_substitutions` from the same joins the naming also carries
whole as `BooleanNaming::edge_joins`. Both fields are `pub` and the
struct is `Default`, so a naming built by hand (a test, a caller
assembling one) can hold joins in `edge_joins` that `substitutions`
does not, or the reverse. The settler then reads the joins only through
`substitutions`, and a vertex a join in `edge_joins` alone took
settles as `SettleError::Dead`, a corrupt record, while the readers of
`edge_joins` (`joined_edge` / `joined_cell`, editor-core's emitters)
see the join. The kernel's own constructions build both from one
`Vec<EdgeJoin>`, so they agree there.

What is owed: one source. Either derive the join rows of
`substitutions` from `edge_joins` where they are read, keeping only the
merge's rows stored; or make the two fields private behind a checked
constructor (`BooleanNaming` is matched and built field by field across
topo and editor-core, so that is a wider move). `design: true` because
which of the two is the answer is open: the first keeps the struct's
shape, the second closes the hand-built route for every field at once.

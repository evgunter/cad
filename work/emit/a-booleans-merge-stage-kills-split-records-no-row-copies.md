---
id: a-booleans-merge-stage-kills-split-records-no-row-copies
kind: issue
title: The boolean's merge stage kills edges with their SplitEdge records and no naming row copies them, as the join stage's now does
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## What

Found by the sweep behind
`a-booleans-same-step-split-and-join-loses-the-childs-lineage.md`
(closed). The pattern was an output-stage kill that drops a
`Provenance::SplitEdge` record a naming reader walks.

`topo::boolean::ops::finish_output` kills edges in two stages, and only
one keeps the records:
- **the join** (`edge_join::join_stage`) copies each killed edge's
  record into `BooleanNaming::joined_lineage`;
- **the merge** (`merge_coplanar_faces_unjoined`) kills the edges
  between the faces it merges, and no row copies their records.

**Why it might matter.** Suppose an edge this step split is killed by
the merge, and its child survives. The child's own record still names
the dead parent. But a walk that has to climb past that parent
(`borders::Obstacles::up_lineage`, falling back to `split_from`) stops
there, because the parent was minted this step and so was never live
at the end of an earlier step.

Not measured: no fixture is known to reach it.

**Other joins outside the boolean**, each with its own naming:
`Body::join_edges_within`'s callers in `shell.rs` and `replace_face.rs`.
Their records are not copied either. The Split copies its own
(`SplitNaming::joined_lineage`).

## Next

- Probe whether any piece edge's lineage passes through an edge the
  merge killed in the same step: count the hops `up_lineage` ends on
  for want of a record, across the editor-core suite.
- If any do, copy the record as the join stage now does.

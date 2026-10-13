---
id: null-edge-copy-rows-are-checked-for-a-cycle-by-their-reader
kind: issue
title: The boolean's null-edge copy rows are checked for a cycle by their reader (uncopied), not where the reduction records them
status: closed
opened: 2026-10-12
priority: P3
cost: E
refs: [face-lineage-rows-are-checked-by-their-readers-not-where-they-are-added]
closed: 2026-10-13
pr: 4981
---


Found by the sweep of `fuse/face-lineage-at-append`, which moved the
boolean's face lineage rows (`Descendants`' substitutions, the merge's
rows, the join's face divisions) onto lists that refuse a row that
could cycle where it is added (`crate::lineage`).

`uncopied` (`crates/topo/src/boolean/ops.rs`) chases a vertex back
through the reduction's null-edge records (`BoolNullEdgeRecord`: each
null edge minted a copy of `at_vertex` at `below_end`/`above_end`) to
the vertex it is a copy of. It bounds the walk by the record count and
refuses `BooleanError::JoinDesync` ("a vertex's null-edge copies close
a cycle with no original") when the budget runs out. The records are
written where the reduction mints each null edge, a copy of a vertex
that already exists, so the check the face rows got can hold there:
a copy is a freshly minted vertex no earlier record names.

What is owed: record the copies in a list that refuses a non-fresh copy
at append (the `FaceDivisions` shape, over vertices), so `uncopied` is
total and its `JoinDesync` arm goes. Measure first that every writer
mints the copy it records.

## Closed

Measured: every writer (`insert.rs`'s `mint_plans` run, `vtxfac.rs`'s
ring struts) records the vertex `mev_null_run` just minted; a probe
over the ci suites found no record whose copy an earlier record names.
The reduction now collects its null edges through `NullEdges::extend`,
which pushes each copy onto its operand's `crate::lineage::VertexCopies`
(the `FaceDivisions` check, over vertices, through one `Minted` core)
and refuses a non-fresh copy as `BooleanError::LineageRow`.
`uncopied` is the list's `root`: total, no `JoinDesync`. The edge- and
cone-split rows have the same shape with unbounded readers:
`split-rows-are-chased-with-no-cycle-check`.

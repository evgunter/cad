---
id: null-edge-copy-rows-are-checked-for-a-cycle-by-their-reader
kind: issue
title: The boolean's null-edge copy rows are checked for a cycle by their reader (uncopied), not where the reduction records them
status: open
opened: 2026-10-12
priority: P3
cost: E
refs: [face-lineage-rows-are-checked-by-their-readers-not-where-they-are-added]
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

---
id: an-uncut-split-at-a-shared-non-manifold-vertex-is-unbuilt
kind: issue
title: No fixture reaches an uncut split that carries a reduce row: an operand with a shared non-manifold vertex on the plane is unbuilt
status: open
opened: 2026-11-09
priority: P4
cost: M
---

Filed by the review of PR 6644 (F3), which closed `an-uncut-split-drops-its-pinch-coincidences`.

`splitting::finish::split_finish`'s uncut branch records the reduce's rows (`RawSplit::from_reduction`). No test reaches that branch with a row. A local probe found none across topo's `ci` suite (3737 tests) or the split, pinch, keel and sliver binaries with their slow set (237 tests).

Why it is hard to reach: the reduce pushes a `SplitOn` row only at an ON vertex whose neighbourhood leaves two or more runs of one side (`splitting::reduce`, `star::runs`). Faces that alternate sides about a vertex cross the plane there, which mints a section and takes the cut path. A union pinch keeps one vertex per cone, so each copy sees one run.

What would reach it: an operand with one vertex shared by two lumps on one side of the plane, with the vertex on the plane, and no section. That is a non-manifold vertex that the doors do not build today. It would be built raw, as `a_figure_eight_hole_cut_through_its_pinch` builds its plate.

Owed: build it, split it uncut, and pin that the result carries the reduce's row, naming the vertex. A revert of the carry is no longer a one-line change (`ReducedRows` is the constructor's only source of rows), but nothing pins the row's presence.

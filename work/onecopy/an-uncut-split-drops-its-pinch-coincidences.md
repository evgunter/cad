---
id: an-uncut-split-drops-its-pinch-coincidences
kind: issue
title: an uncut split (no section) returns no coincidences, where the cut path carries the reduce's pinch coincidences
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6644
---


## Finding

Filed by `onecopy/side-at-infinity` (PR 6595's review).
`splitting::finish::whole_body_side`, the split with no section, returns
`coincidences: Vec::new()`. The cut path returns `red.coincidences`: the
reduce's ON vertices whose neighbourhood leaves two or more runs on a side
(`splitting::reduce`, `Relation::OnCarrier`, `DecisionSite::SplitOn`).
Such a vertex can occur where the plane touches the body without cutting
it (two lumps on one side meeting at a vertex on the plane), so an uncut
split may drop a pinch it decided. Whether it should carry them turns on
what the naming and records consumers of `SplitResult::coincidences`
expect from an uncut split. Carrying it is small (pass `red.coincidences`
through), but it changes the published output of every uncut split that
touches at a pinch, so it is not done blind.

## Closed (`onecopy/foot-and-pinch`)

D10's rule decides it: a row is recorded where a ladder decided it from
a margin and the result holds it (`topo::coincidence`). The reduce
decides a `SplitOn` row the same way on either path, and an uncut
split's sides keep the vertex on the plane. `whole_body_side` now
returns its two sides only. `split_finish`'s uncut branch builds the
`RawSplit` from `red.coincidences`, as the cut branch does. The one
consumer (`editor-core`'s `wire_split` recording) names the row in the
target's table and the tool, and does not distinguish the two paths.

No test pins the carry, because no fixture reaches it. A local probe on
the uncut branch found no reduce row there across topo's `ci` suite
(3737 tests) or the split, pinch, keel and sliver binaries with their
slow set (237 tests). A `SplitOn` row needs ≥ 2 runs of one side about
an ON vertex, which crosses the plane there and mints a section. A union
pinch keeps one vertex per cone. An operand with a shared non-manifold
vertex might reach it; none is built.

---
id: one-region-reads-a-cycle-crossing-at-a-shared-vertex-by-its-first-vertex
kind: issue
title: one_region::read_cycle reads a curved face's cycle that crosses another at a vertex they share by its first placed vertex (Answering::Any), so a CycleOffFaceSide can pass unreported
status: open
opened: 2026-11-09
priority: P3
cost: M
---


## Finding

Filed by `onecopy/pinch-premise` (PR 6605's review). `one_region::read_cycle`
reads a curved face's cycle against the face's other cycles at each of
its vertices, through an `Answering::Any` ladder, and the first vertex
placed answers. A vertex on a cycle of the cut is set aside.

The premise that every placed vertex reads one side holds where the cycle
does not cross the cut, and a touch at a shared vertex switches no side
(`chord_join::paths_rung`'s premise). It is not checked here, and this is
a validator: nothing upstream joins the face's own cycles.

- A crossing inside two edges is the census's `EdgeEdgeCross`.
- A crossing at a vertex the two cycles share is not shown to be any
  check's. Check 9's `PinchCornerCrossed` reads one loop passing a point
  twice, not two loops at one vertex, and `surface_cycles` walks each
  loop on its own (it merges no cycles that share a vertex).

Such a cycle has placed vertices on both sides. Which one answers depends
on walk order, so `CycleOffFaceSide` can be missed.

**Probe:** a curved face, for instance a cylinder wall sheet, with two
cycles that share a vertex and cross there, read by `region_errors` in
every walk order. If another check refuses the body, record which check
and close this row. If not, make the site read every vertex (the agreeing
check, `a-ladder-that-checks-its-answers-agree-has-no-home`, after
`[ev]` PR 6592) or refuse a crossing at the shared vertex.

---
id: containment-reads-a-flip-crossing-that-names-nothing-as-proof
kind: issue
title: The drive's containment reads boundary leaves refused FlipCrossing as proven off the witness's branch, including ones whose evidence names nothing
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by all three designers weighing
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`
(2026-11-24), by reading; not measured.

`editor_core::drive`'s containment (`MeasureAccounting::containment`,
`drive::contained`: E2's chamber containment) reads "every boundary
leaf is `FlipCrossing`" as "all mass outside the box is provably off
the witness's branch, so the unresolved budget is exact". A
`FlipCrossing` whose evidence names nothing is no such proof: it is a
leaf whose record differs from the witness's with no decision shown
turned. While PR 7344's defect stood, every curved depth-0 drive
reported containment on that. The range search already declines to
read an empty flip as proof (`Unproven::FlipUnfounded`); containment
and the viewer's "boundary at the current value" do not.

It falls with the parted-path weighing's final state (a `FlipCrossing`
names what differs, by type). Until that lands, the first step is a
row pinning containment against a boundary leaf whose flip names
nothing. P1: a containment claimed on nothing is a wrong answer a
consumer reads. (BOXFINISH orchestrator)

---
id: a-trace-fault-preempts-the-charts-backwards-near-the-bands-edge
kind: issue
title: Near a face chart's pole band, a trace step's own fault can preempt the Backwards refusal the profile's carriage would give
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Found while building Ev's ruling on [ev] #7353, in PR 7151.

## What

A face chart's turn is now selected before the trace, its pole verdict alone over the session box (`mate::value::FaceChart::select`, `eval::wire::trace::drawn`). The profile's band is decided afterwards, over its loops, in `wire_profile` (`MintedChart::carries`).

So near the band's edge, a step that traces reads its faces through a turn that the profile's carriage will refuse. If the traced read itself refuses first, the profile reports that trace fault, not the `FaceChartBackwards` naming the orientation to write. Examples: a crossing the band cannot order, or a piece that no longer reaches the trace.

The refusal is still a refusal, so nothing builds wrong. But its kind, and so its recourse, depend on which read happens to fail first.

## Owed

Where the profile's carriage would refuse backwards, the profile reports that refusal, whatever its trace steps say. For example, decide the loops' carriage before reporting a trace step's fault, or report both. Add a row with a near-pole traced profile whose trace step faults.


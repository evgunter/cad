---
id: round-holes-get-no-chart-bound-benefit
kind: issue
title: A circular hole's two half-arc envelope boxes cover the whole disc - no cell inside a round hole is ever certified outside the face
status: open
opened: 2026-09-06
refs: [clearance-window-tightening-needs-chart-boundary, 1911]
---


## What

Found by TRIM-3 PR-1's dual (R2, NOTE-6). `ChartBound`'s `Envelope`
edge is the whole-span interval image of the pcurve — for a hole
bounded by two half-arcs, the two boxes together cover the entire
disc, so `certifies_outside` never certifies a cell inside a round
hole (measured: square hole 2956/2956 outside points certified; round
hole 2760/2931 — every miss inside the hole). Sound (a kept cell is
never wrong), and the tightening PR-2 buys on plates with round holes
is exactly zero inside the holes.

## Fix shape

An `Envelope` that also carries the arc's INNER side — the lune
between chord and arc lies on one side of the chord, so the region
excluded by a ring arc is bounded by the chord's polygon MINUS the
lune's box on the arc side, not the whole box. That is a second box
per envelope (the chord side) and a sign; the parity walk over the
chord polygon already exists. One unit, NUMERIC, after PR-2 measures
whether any consumer needs it.

## Home

TRIM — `chart_bound.rs` is this program's.

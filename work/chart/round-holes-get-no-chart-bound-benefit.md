---
id: round-holes-get-no-chart-bound-benefit
kind: issue
title: A circular hole's two half-arc envelope boxes cover the whole disc - no cell inside a round hole is ever certified outside the face
status: closed
opened: 2026-09-06
refs: [1911]
priority: P1
cost: M
closed: 2026-10-24
parent: clearance-cells-are-decided-in-model-space
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

## Closed

Closed 2026-10-24 into `clearance-cells-are-decided-in-model-space`.
CHART's two region designers converged there: a face's region is read off
its edges on their carriers, in model space, so no chart polygon is built
or extended. That row states what this one's case becomes under that design.
The P0 band on the pole row was wrong, and both designers said so. Today's
state is sound (the refusal stands), and clearance never asks on those
kinds. Its live boolean cost, if it has one, is the probe on
`w2-reads-a-one-sheet-predicate-off-the-walk`. (CHART orchestrator)

---
id: a-landings-edge-parameter-is-spelled-three-times-in-the-region-doors
kind: issue
title: A landing's edge parameter is spelled three times in the region doors
status: open
opened: 2026-10-24
priority: P2
cost: M
---

## What

Where a leg lands on a conic edge, each region door reads the edge's
own parameter there by hand, off the carrier's frame:

- `torus_region.rs`, `Legs::landings`, twice (the parallel-leg arm and
  the transversal-parallel arm, about `:608` and `:681`):
  `near_span((w·(axis × u_ref)).atan2(w·u_ref), span)`;
- `ruled_region.rs`, `Conic::angle` (about `:311`): the same for a
  circle or an ellipse, `atan2((w·v)/b, (w·u)/a)`.

The two spellings differ in their bits on a circle (`a = b` still
divides). Found by the sweep of
`the-sphere-ray-and-the-circle-leg-stand-outside-the-region-skeleton`,
whose items (a)–(e) did not cover it.

## Owed

One reading of a point's parameter on a conic edge, which every region
door calls (and the sphere door's landings, which take the root's own
parameter, need not). If its bits move a margin, re-baseline it and say
what moved.

## Moved (2026-10-26)

CHART folded into CHARTTAIL (Ev, in chat, 2026-10-26): CHART had closed
every P0 and P1 row, and its last rows are the chart boundary's tail.
Id and body unchanged.

## Also (2026-10-30, RESTCURVE)

A fourth spelling of the circle's own parameter at a point:
`geom_brep::chart_singularity::foot` (a line's projection, a circle's
`atan2` about its centre brought to the turn at or past a start), which
the chart-singularity reading and step-import's meridian landing
(`normalize/singularity.rs`, `param_at`) both cite since PR 6054. The one
reading this row owes can be that function, or replace it. Filed here,
not widened there (PR 6054's adjudication, S-b).

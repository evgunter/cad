---
id: the-plane-split-reads-no-crossing-of-a-spline-edge
kind: issue
title: The plane split refuses a spline edge its box cannot clear (CurvedEdgeUnsupported) though the ring-composite arm finds its crossings
status: open
opened: 2026-10-26
priority: P2
cost: M
---



Filed by SPLICE's `nurbs-edge-crossing-rung-is-the-ring-composite`
lane, from its sweep of `PlaneCrossingLane::Unlaned`'s readers.

## What

The plane split (`splitting/classify.rs`, `insert_crossings`) answers a
spline edge with `PlaneCrossingLane::Unlaned`. It passes the edge only
behind a box the plane cannot meet (`edge_clears`), and otherwise
refuses `SplitReduceError::CurvedEdgeUnsupported`. The boolean's planar
lane now reads such an edge through the curve × face table's
ring-composite arm (`geom_brep::NurbsLane::surface_crossings`,
`reduce::spline_plane_crossings`), which returns the crossings strictly
inside the span, ascending. The split could insert those roots as it
inserts a conic's.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). PARTING holds the parting's reading of edges lying in faces and the cuts on spline faces; id and body unchanged. (CLEAVE orchestrator)

---
id: a-quadric-or-wall-ring-closing-refuses-a-spline-run-edge-it-may-read
kind: issue
title: A ring closed by a conic chord or a wall's chart refuses a spline run edge that the path lane may read
status: open
opened: 2026-11-13
priority: P3
cost: M
---


Filed by the orchestrator from two designer reports on the planar
ring-winding fork (fork-log row 264, `[ev]` PR 6896). Neither
designer traced whether the case is reachable.

## What

`boolean::join::run_spline_edge` (`crates/topo/src/boolean/join.rs:4985`)
finds the first spline (NURBS) edge on a ring run. `ring_run_ccw`
refuses on it for every closing except a carried chord's. A curved
face (sphere, cone or torus) can already hold a carried chord's spline
edge and later be ringed by a conic chord, which is the `Quadric`
closing. A spline wall's chart closing is the `Wall` closing. In both
cases the run is refused before any winding is read. The carried path
already reads such an edge through `SplineArc::of_edge` evidence, so
the `Quadric` path may be able to read it too.

## Done when

Either a row reaches the case and the closing reads the spline run
edge, or the case is shown unreachable and the refusal's doc says why.

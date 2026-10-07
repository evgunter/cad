---
id: sweeps-build-one-rim-edge-per-segment-not-per-run
kind: issue
title: Sweeps build one rim edge per profile segment, not per collinear or cocircular run: extrude mints a station vertex on both cap rims, a partial revolve one wall per arc
status: review
branch: fuse/sweep-runs
pr: 4200
opened: 2026-10-06
priority: P1
cost: M
refs: [a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made, 4140]
---


## The finding

Step 2 of PR 3881's ruling (the unit
`a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made`)
asks for the join at every output stage, "boolean and sweep, with
sweeps building one rim edge per run". PR 4140 built the boolean half
only. The sweeps still build rims per profile segment:

- **extrude:** a wall over a run of collinear profile segments mints a
  station vertex on both cap rims, one rim edge per segment
  (`SideWall::bottom_rims` / `top_rims`, `crates/sweep/src/extrude.rs:231`).
  Each station is a joinable vertex of the sweep's own output.
- **partial revolve** (`crates/sweep/src/revolve/partial.rs`): a
  cocircular run is built one wall per arc.
- **full revolve** already collapses each run
  (`Collapsed`, `crates/sweep/src/revolve/full.rs:828`), so it is the
  pattern to follow.

A boolean now joins an extrude operand's planar station vertices
wherever they survive into its output, so the boolean result is
maximal. The sweep's own result, used as-is, is not.

## What the fix is

Build one rim edge per run in extrude and partial revolve, as full
revolve does; then assert `topo::joinable_vertices` is empty on each
sweep's output. The partial revolve's cocircular run is a curved
join, so its rim also needs what
`curved-joinable-vertices-are-left-unjoined` decides.

Consumers of `bottom_rims` / `top_rims` and of per-segment wall
naming move with it.

## Order

This precedes step 3, the tier-2 no-joinable-vertex check: a sweep
output holding station vertices would fail it.

## Built (FUSE, `fuse/sweep-runs`, 2026-10-06)

- **extrude** and **partial revolve** collapse each loop's runs before
  they build, as the full revolve did (`swept::collapse_runs`,
  `revolve::runs::Collapsed`): one strut, one wall and one rim edge
  per cap per run. `SideWall` carries `bottom_rim` / `top_rim`.
- **The partial revolve's arc runs are direct construction**, no curved
  join: each wedge cap carries a cocircular run as one meridian edge,
  so the props blocker that kept one wall per arc
  (`partial-revolve-arc-runs-wait-on-the-meridian-fold`) does not
  arise; `CurvedRuns` is gone.
- **A run of on-axis segments** in a partial revolve is one axis edge:
  its interior pole was a valence-2 vertex between the two wedge caps.
- **Naming:** `RimEdge(end, PieceRun)` and `AxisEdge(PieceRun)` hold
  the run (one-piece runs keep their wire spelling); a partial
  revolve's meridians are per run; no cap vertex or meridian vertex at
  a station.
- **Asserted:** `run_walls_built::holds` checks every swept body for
  `topo::joinable_vertices` and for curved stations
  (`common::stations::station_vertices`).

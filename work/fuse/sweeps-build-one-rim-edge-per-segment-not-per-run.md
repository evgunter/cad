---
id: sweeps-build-one-rim-edge-per-segment-not-per-run
kind: issue
title: Sweeps build one rim edge per profile segment, not per collinear or cocircular run: extrude mints a station vertex on both cap rims, a partial revolve one wall per arc
status: open
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

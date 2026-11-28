---
id: region-ray-exhausted-folds-an-arcs-unsettled-reading
kind: issue
title: RegionRefusal::RayExhausted folds an arc's unsettled reading (a per-edge limit) into the walk's exhaustion, so the solid door routes it as GRAZED with the move recourse
status: open
opened: 2026-11-25
priority: P3
cost: M
---


Found in the delta review of PR 7748 (lane `inside-contain-small`),
which made a curved face's region exhaustion the solid door's
`PointInSolidError::RegionRayExhausted { face }`.

## The finding

`RegionRefusal::RayExhausted` (`crates/topo/src/region.rs`) has two
sources:

- `region::walk`: no path of the walk settled
  (`ray_walk::NoRaySettled`). This is an in-band reading of the point,
  and moving the point or the geometry is the repair.
- `region::circle_edge_hit`: `ConicHit::Unsettled`, one arc's own
  reading (`ConicArc::hit`) left the point unplaced, for example an
  ellipse edge's span walk running out of budget. This is a limit of one
  edge's reading, like `Uncrossable`, which the solid door states as
  `EdgeCarrierUnsupported` with `NOT_YET_ENDING`.

Both arrive as `RegionRayExhausted`. They are routed as the walk's:
`in_band`, census `WitnessGrazed`, validate `GRAZED` with
`MOVE_RECOURSE`, and the Boolean's `NoRaySettled` text.

## The two readings

- **Leave as is:** `circle_edge_hit`'s doc says that on a circle the
  nearest foot walks nothing, so `Unsettled` "does not arise". If so,
  only the walk feeds the arm, and the fold is harmless.
- **Split:** wherever an arc can answer `Unsettled` (an ellipse, or a
  conic the hit reads by a bounded walk), its refusal is a per-edge limit
  with its edge in hand. It should be `RegionRefusal::Unreadable`, or an
  arm of its own carrying the edge, read as a limit (`NOT_YET_ENDING`),
  not as an in-band graze whose recourse is to move the geometry.

## The repair

Decide whether `ConicHit::Unsettled` is reachable from `circle_edge_hit`'s
callers (sphere, torus and ruled `on_edge`). If it is, give it a per-edge
refusal and route it as a limit.

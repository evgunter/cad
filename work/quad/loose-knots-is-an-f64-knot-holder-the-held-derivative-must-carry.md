---
id: loose-knots-is-an-f64-knot-holder-the-held-derivative-must-carry
kind: issue
title: LooseKnots holds f64 knots: carrying held knots' derivatives into the lane scalar must carry it too
status: open
opened: 2026-10-22
priority: P2
cost: E
refs: [an-enclosure-carrier-holds-point-weights-and-knots]
---


## Finding

`geom_core::spline::LooseKnots` (`crates/geom-core/src/spline/knots.rs`,
added by D30) holds its knots as `Vec<f64>`. The
`scripts/gates/knots-f64-interim.sh` holder ratchet counts it under
`holders:crates/geom-core`. The orchestrator accepted that rise
(27→28) as one holder moving from props' `Loose` into the crate that
owns knot types; geom-brep fell 15→13.

Its knots are read off `KnotVector::derived()`, which is the f64
interim. A held-knots carrier (`Knots<T>`, C6) differentiated at the
lane scalar has wide derivative knots too, and where those are no
clamped vector, `LooseKnots` is their only home. So whoever carries
held knots' derivatives into the lane scalar (C6 S5, or INLANE's
`an-enclosure-carrier-holds-point-weights-and-knots`, which asks
whether a carrier may hold non-point knots) must carry `LooseKnots`
with it, as `LooseKnots<T>` or as structure plus held values the way
`Knots<T>` is. Until then the props ladder's `Loose` directions stay
point-knot only, which today is every caller.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. `LooseKnots` still holds `knots: Vec<f64>`
(`crates/geom-core/src/spline/knots.rs:1623`). No PR in the measure door
touched it.

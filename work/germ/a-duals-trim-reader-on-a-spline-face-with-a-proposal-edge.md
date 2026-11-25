---
id: a-duals-trim-reader-on-a-spline-face-with-a-proposal-edge
kind: issue
title: A dual's trim reader on a spline face has no boundary to read for an edge whose only image is a proposal
status: open
opened: 2026-11-25
priority: P2
cost: H
---


Filed by the SCALAR orchestrator from the design fork ruled on `[ev]` PR 7719 (2026-11-25); it belongs with `a-spline-crossing-at-a-dual-is-not-built` and blocks a dual boolean on a spline wall, together with the crossing's implicit-function tangent and a dual section march.

DL5.3 now says the `Pcurve` variants that hold a proposal (a fitted image: `General`, `Fitted`) have no values at a dual. No dual build produces or reads one today: every producer (the mint's fitted lane, section edges, the offset fit, import) is closed to a dual by its bound, and the loft states determined images on its rims. So nothing is broken. When a dual boolean that cuts a spline wall is built, its section edge will have only a proposal image, and the trim reader (`crates/topo/src/spline_trim.rs`: a face's trim as a chain of `f64` Bézier pieces cut from the stored images, a point placed by ray parity; `spline_region.rs` and `pcurves_rings.rs` go through it) needs a boundary in the chart to place points against. It cannot decide from the wall and carrier alone as written.

Decided then, with a real case to test against, and deliberately not now (Ev's objection on that PR was to machinery that puts a dual's values through certifying arithmetic; a route no document can reach or test is worse): a chart-free reading from the model, or another construction. Ruled out already: the dual recomputing the fit and its check on its own values, and the base build's row handed over by a node-by-node correspondence (Ev, PR 5689).

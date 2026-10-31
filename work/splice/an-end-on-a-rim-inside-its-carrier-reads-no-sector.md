---
id: an-end-on-a-rim-inside-its-carrier-reads-no-sector
kind: issue
title: A span ending tangent on a face's rim where the carrier runs on past it (a trimmed spline face, or an analytic face) reads no sector
status: open
opened: 2026-10-31
priority: P2
cost: M
refs: [a-span-ending-on-a-faces-rim-has-no-sector-clearance]
---

Filed from `splice/rim-sector-clearance` (closing `a-span-ending-on-a-faces-rim-has-no-sector-clearance`), as the blind spot of its sector.

## What

The spline-face arm's sector at an end (`geom_brep::curve_face::spline_face`, `Pair::sector`) clears a span that ends on a face's rim, tangent there, and leaves on the far side of the rim. It reads only a rim that is an edge of the chart's **domain**, where the chart itself stops: its argument bounds every solution in a one-sided box of the state, `Δq ≥ 0` across the edge. Two rims have no such side:

- **A trimmed spline face** whose rim is a trim curve strictly inside its chart. The chart runs on past the rim, so a span leaving the rim tangent to the chart lies within the band of the chart, outside the face. The walk reads the chart, not the face, so it refuses as it did at a domain edge before the sector.
- **An analytic face's rim** (`curve_face::composite` for a spline span; the sweep's analytic root doors for an analytic one). Both read the span against the face's **carrier**, which runs on past the rim, so the same tangent departure stays in band beside the end and refuses.

The face's inward departure here is the trim's: the carrier's partials across the trim curve's pcurve, one-sided by the trim. No witness yet. Klein's rims are domain edges. The sweep `grep -rn "END_REACH\|read past\|past each end" crates/geom-brep/src crates/topo/src/boolean` turns up only the spline-face arm, the composite's end rule (`composite.rs` module docs: an end within the band is the end's; a piece in band that no cut parts refuses) and the curve-pair span rows.

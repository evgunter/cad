---
id: nurbs-curve-has-no-net-state-door
kind: issue
title: NurbsCurve has no net_state twin, so curve-side placeholder-or-described reads were never swept for a poisoned net
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [described-net-two-state-reads-hand-a-poisoned-net-the-described-arm, transform-rigid-maps-a-poisoned-net-into-the-placeholder]
---


## What

`NurbsSurface::net_state() -> NetState` (`crates/geom/src/surfaces/nurbs.rs`)
answers the three states of a net: placeholder, described, and described
but carrying poison. A consumer's match over it is exhaustive. The curve
has only `NurbsCurve::is_placeholder` (`crates/geom/src/curves/nurbs.rs`),
so every curve-side consumer asks the two-state question and hands a
poisoned net to the described arm. That is the shape the surface-side
sweep (`work/pipe/described-net-two-state-reads-…`, closed) checked at
thirteen-plus sites.

The curve side was out of that sweep's scope. One curve instance is
already filed: `map_curve`'s `Curve3::Nurbs` arm, on SHELF's
`transform-rigid-maps-a-poisoned-net-into-the-placeholder`. Others read
`is_placeholder()` on a curve, for example `geom::curves`'
`matches!(&cd, Curve3::Nurbs(n) if !n.is_placeholder())` (two sites).
They have not been traced to what a poisoned net does there.

## Fix shape

Give the curve the same three-state door, then run the surface sweep's
two passes over the curve-side reads:
- `is_placeholder` reads on a `Curve3::Nurbs` / `NurbsCurve`;
- direct reads of a curve's control points outside `geom`.

Dispose each read as fine (refuses or escalates downstream) or file it
on its owner.

Filed by the PIPE orchestrator from the routing lane's stated blind spot.

## Re-homed from FLUX to KNOT (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. KNOT collects the spline, net and fit doors in `geom-core` and `geom`: refinement inside an enclosure, the net-state reads, the NURBS derivative and projection doors, and the fit's refusals and solves. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

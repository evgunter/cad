---
id: f64-spiric-wall-ends-near-tangency-can-read-a-closed-junction-open
kind: issue
title: At the f64 lane a spiric wall's ends near tangency carry rounding past the vertex slack, so a closed junction can read open
status: open
opened: 2026-10-13
priority: P3
cost: M
---


Found in review of PR 4938 (`flux/periodic-closure-one-decide`), which
decides a spiric torus wall's junctions closed on its lifted chart ends
(`geom_brep::props::spiric::wall_windings`).

`topo::props::quad_lane::spiric_face` reads each half's ends through
`Pcurve::eval` on the lane scalar and crosses them in with
`Interval::from_certified`. A spiric wall image's azimuth is
`u₀ + σμ·atan2(f, d)` with `f = √((R + r·cos t)² − d²)`
(`geom::spiric_radial`). As the oval nears tangency (`|d| → R − r`, near
`t = π`) the radicand is a difference of near-equal squares, so at the
`f64` lane `f` carries a relative error of order `√ulp`. The crossing makes
the end a point bracket with that error unaccounted for. A junction there
that closes can then read a gap past `S + Kε` and refuse `ChartLoopOpen`.
That refusal fails closed, but it is the wrong kind. A `NaN` radicand
escalates, as it should. The interval lane encloses the cancellation and
is not affected.

Probe to write first: a near-tangent oval (`d = R − r − δ` for small
`δ`) whose junction sits at `t = π`. Run it at `f64` and at `Interval`
and compare the junction verdicts.

Fix shape: read the wall's ends at the interval scalar from the stored
data (as the quadrature does for its terms), or add the radial rounding
bound to the end's slack.

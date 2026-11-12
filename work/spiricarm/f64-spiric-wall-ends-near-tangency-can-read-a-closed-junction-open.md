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
(then `geom_brep::props::spiric::wall_windings`; since PR 6808 the
rotational lane's `loop_windings`, `crates/geom-brep/src/props/quad/rotational.rs`).

`topo::props::quad_lane`'s face builder reads each spiric half's ends
through `Pcurve::eval` on the lane scalar and crosses them in with
`Interval::from_certified` (`crates/topo/src/props/quad_lane.rs:711`,
handed to `spiric_wall_edge`); the lane's `end_slack` pays the image's
envelope only, so the defect survives PR 6808 unchanged in shape. A spiric wall image's azimuth is
`u₀ + σμ·atan2(f, d)` with `f = √((R + r·cos t)² − d²)`
(`geom::spiric_radial`). As the oval nears tangency (`|d| → R − r`, near
`t = π`) the radicand is a difference of near-equal squares, so at the
`f64` lane `f` carries a relative error of order `√ulp`. The crossing makes
the end a point bracket with that error unaccounted for. A junction there
that closes can then read a gap past `S + Kε` and refuse `ChartLoopOpen`.
That refusal fails closed, but it is the wrong kind. A `NaN` radicand
escalates, as it should. The interval lane encloses the cancellation and
is not affected.

Size, probed in review of PR 6808 at `R = 1.3`, `r = 0.45`, `t = π`
(`ρ = 0.85`): the `f64` azimuth's error in the chart metric is `2e-14`
at `δ = 1e-6`, `4e-11` at `1e-12`, `8e-10` at `1e-15`, and `5.2e-9` at
the `d` one ulp short of `ρ`. Against the junction's `3ε = 3e-9` at the
default `ε` it reads open only within a few ulps of tangency. The probe
was of the rounding (`f64` against a 60-digit decimal at the same
inputs), not a lane run.

Probe to write first: a near-tangent oval (`d = R − r − δ` for small
`δ`) whose junction sits at `t = π`. Run it at `f64` and at `Interval`
and compare the junction verdicts.

Fix shape: read the wall's ends at the interval scalar from the stored
data (as the quadrature does for its terms), or add the radial rounding
bound to the end's slack.

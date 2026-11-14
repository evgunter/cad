---
id: a-whole-turn-parallels-ends-are-read-apart-at-the-interval-scalar
kind: issue
title: A whole-turn parallel's carried miss is read between its two ends evaluated apart, so at Interval a tilted thin tube from about 800 m escalates props_chart_loop_whole at eps 1e-12
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found by BOXFINISH's edge-turns unit (branch `boxfinish/edge-turns`),
which gave the cylinder's and the torus's per-edge whole-turn misses the
cone's constructors (`EdgeTurn::exact`, `EdgeTurn::carried`,
`crates/geom-brep/src/props/closure.rs`). What it left refusing is this.

## Measured

The hollow tube `sweep::tube_along_arc_hollow::<f64>(tube_frame(origin,
(1,1,1)/√3, (1,−1,0)/√2), major, TubeWindow::Full, 2.5, 0.001)` at
ε = 1e-12 (the pinned arm of
`crates/sweep/tests/far_walls_read_their_whole_turns.rs`'s
`a_tilted_hollow_tube_measures`):

| major | at ε = 1e-12 |
|---|---|
| 300 … 750 | builds, volume within 1e-11 relative of `2π²·R·(r_o² − r_i²)` |
| 800 | `[0, 1.080e-12]` |
| 850 | `[0, 1.137e-12]` |
| 900 | `[0, 1.215e-12]` |
| 990 | `[0, 1.363e-12]` |
| 1100 | `[0, 1.882e-12]` |

Each refusal is `VolumeUncomputable { Face { FaceKey(1v5), Escalated {
Indeterminate { band (1e-12, 1e-11), predicate: "props_chart_loop_whole" },
check: Inventory } } }`, raised at `closed_loop_turns` in the `Interval`
re-derivation of the role walk (`role_walk_set` → `plus_v_verdict`). At
ε = 1e-9 and 1e-6 every major builds.

**What it is made of** (major 990, local probe at `closed_loop_turns`):
the face's loop is two whole-turn parallels (radii 992.5 and 987.5) and
two meridians. In azimuth, the meridians' short turns miss by exactly 0
and the parallels' carried misses are ±2.15e-15 rad each, at levers
992.5 and 987.5 m: Σ.hi 4.36e-12 m against the `3ε` slack, so
`[0, 1.36e-12]`. The parallel's own turn is a point (`±τ_f64`); the
width is all in its own-frame arms, `turn_arms(axis, a − centre,
b − centre)`: dot ≈ 9.85e5 m², cross ±2.1e-9 m². That is the width of
the parallel's two ends, `eval(t0)` and `eval(t0 + τ)`, each enclosed
apart at the interval scalar about 1e-12 m wide at ~990 m from the
origin: the miss reads two enclosures of one vertex as a possible
disagreement. It grows with the ends' distance from the origin.

Behind it, in the tube coordinate: the meridians' tube turns are still
read back about the core circle (`EdgeTurn::new` in `torus_chart_loop`),
±3.7e-13 to ±4.7e-13 rad at the minor radius 2.499, Σ.hi up to
2.35e-12 m on the same tube at 990: under the slack, but the next term
to pass it.

## What closes it

A rim whose stored span is exactly one period has ends that are one
carrier point a period apart: its own-frame turn agrees with them by
construction, as the cone's and cylinder's short turns do, so its miss
can be stated exact rather than read between two enclosures of one
point. Its neighbour on this slate
(`a-whole-turn-rims-headroom-is-read-through-a-one-ulp-tau`) reads the
same rim's span against the period through a one-ulp τ; one reading of
"a span that IS one period" may close both. A meridian's tube turn can
be carried as a parallel's azimuth is, read in the meridian's own plane.

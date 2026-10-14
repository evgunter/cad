---
id: closed-form-loop-whole-charges-near-axis-rounding-at-the-largest-radius
kind: issue
title: At the interval lane a torus loop with a parallel near the axis escalates props_chart_loop_whole, its near-axis rounding charged at the loop's largest radius
status: open
opened: 2026-10-13
priority: P3
cost: M
---



Found in review of PR 4938 (`flux/periodic-closure-one-decide`). It is
older than that PR's last fix pass: it has been there since the
closed forms first decided their loops whole (f623, b62). Main measures
these faces.

At `ε = 1e-12`, on the interval scalar only, spindle and horn torus
faces escalate `props_chart_loop_whole` when a parallel sits within
about `1e-8` m of the axis. The `f64` lane measures them.

Probe to write first: a torus rectangle of parallels at `v0 = 1` and
`v1`, with meridians at `u = 0` and `u = 1`. Take the spindle
`(R, r) = (4 mm, 5 mm)` and the horn `(5 mm, 5 mm)`, and put the
parallel at `v1 = acos((ρ − R)/r)` so its radius about the axis is `ρ`.
Sweep `ρ = 1e-11 … 1e-8` m and run at `f64` and at `Interval` with
`CAD_TOLERANCE_EPS=1e-12`. The interval lane escalates
`props_chart_loop_whole` across that sweep, while `f64` and main
measure. The reviewer's probe is `review_4938_v3_cusp_reach` in
`geom-brep/tests/review_4938_v3.rs` on `review/4938-verify-v1` @
bc417d37.

Likely cause: `closure::closed_form_windings`
(`crates/geom-brep/src/props/closure.rs:261`–`:303`) meters the residual
`|W − k·τ|` at the loop's **largest** junction lever
(`lever[c] = lever[c].max(j.turn[c].1)`, `:290`; the residual,
`:303`). `W`'s interval width, though, comes from azimuths read near the
axis: the near-axis parallel's `about_axis` turns
(`curved::about_axis`, `curved::tube_turn`) are read off points within
`ρ` of it, and their rounding is of order `ulp/ρ` radians. Charging
that width at the large lever (millimetres) turns sub-`ε` metres of
rounding into a residual past `3ε`.

Fix shape: meter each term's rounding at its own radius. Either carry
`Σ lever_i·width(turn_i)` beside `W` and decide the residual against
that, or bound `W`'s width term by term so a near-axis turn's width is
charged at its own lever, not the loop's largest. Re-run the probe and
the closed-form rows in `crates/geom-brep/tests/periodic_chart_closure.rs`
at all three ε after the change.

---
id: torus-chart-window-slack-reads-arc-length-where-the-pullback-needs-the-chord
kind: issue
title: The torus chart window's v slack 2·envelope/r is argued from arc length, where the pull-back between two surface points needs the chord
status: open
opened: 2026-11-25
priority: P3
cost: C
---


## Finding

Read off the code; not measured. `cover::TorusChartWindow`'s docs
argue the slack from "one radian of `v` moves a surface point by
exactly `r`", and `cover::torus_window_slack` returns `2·envelope/r`
(and `2·envelope/(R − r)` for `u`). The pull-back compares two surface
points no more than `2·envelope` apart, and what bounds their angle is
the meridian CHORD `2r·sin(δ/2)`, not the arc `r·δ`: `δ ≤ 2·asin(envelope/r)`,
which exceeds `2·envelope/r` by the factor `asin(x)/x ≈ 1 + x²/6`,
`x = envelope/r`. At the guard's edge (`envelope = r/2`) the window
is about 4.7% short of its own argument. At the envelopes stored today
(sub-nanometre against radii of order one) the shortfall is far below
an ulp, so no window is wrong in practice. The `u` channel has the same
shape, against `R − r`.

`cover::TorusChartWindow::finish_latitude` (the spindle sheet's `v`)
takes `π·envelope/r`, from the chord bound `2r·sin(δ/2) ≥ (2/π)·r·δ`
on `δ ≤ π`. The torus arm could take the same constant, or keep its own
with its guard tightened until `asin(x)/x` is covered.

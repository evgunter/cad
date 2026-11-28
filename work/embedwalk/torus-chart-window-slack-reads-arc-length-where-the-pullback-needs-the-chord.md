---
id: torus-chart-window-slack-reads-arc-length-where-the-pullback-needs-the-chord
kind: issue
title: The torus chart window's v slack 2·envelope/r is argued from arc length, where the pull-back between two surface points needs the chord
status: closed
opened: 2026-11-25
priority: P3
cost: C
branch: embedwalk/two-bands
pr: 7786
closed: 2026-11-25
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

## Closed

Closed by PR 7786 (`embedwalk/two-bands`). `cover::torus_window_slack`
now takes the chord bound, `π·envelope/(R − r)` in `u` and
`π·envelope/r` in `v` (`cover::chord_slack`, shared with the sheet's
latitude), which holds at any envelope.
`boxes::tests::a_torus_windows_slack_is_guarded_by_the_bound_it_rests_on`
plants an envelope at the guard's edge and asserts each slack holds the
chord's angle `2·asin(envelope/ρ)`; the arc-length slack fails it.

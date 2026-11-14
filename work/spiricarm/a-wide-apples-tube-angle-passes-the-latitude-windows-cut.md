---
id: a-wide-apples-tube-angle-passes-the-latitude-windows-cut
kind: issue
title: A spindle sheet's v is read in the latitude window (-π + 1, π + 1], which holds [-v*, v*] only for v* < π - 1: an apple with R > 0.54 r reads its far side a turn off
status: closed
opened: 2026-11-03
priority: P2
cost: E
branch: spiricarm/horn-cusp
closed: 2026-11-11
pr: 6806
---

(SPIRICARM, stage 1 piece 1 of `horn-and-spindle-tori-have-no-representation`.)

## What

`geom::ChartInverse::point`'s `ChartArm::Spindle` arm (`crates/geom/src/chart_point.rs`) reads the tube angle `atan2(h, ρ − R)` with `Azimuth::principal_in(Window::Latitude, band)`, the window `(1 − π, 1 + π]`. The sheet's `v` is closed on `[−v*, v*]`, `cos v* = −R/r`: every lemon (`v* < π/2`) and every apple with `v* < π − 1` (`R < r·cos 1 ≈ 0.54 r`) lies inside it, but a wider apple's points at `v < 1 − π` read `v + τ`, off the sheet's window. The angle window `(−1, τ − 1]` the torus uses is worse (it cuts at `v = −1`).

A window centred on zero with its cut at `±π`, where only a horn's cusp lies, would hold every sheet; `azimuth::Window` requires its start strictly inside `(−π, 0]`, and each window's start is a measured choice (`azimuth.rs`'s `LATITUDE_WINDOW_START` docs), so the choice is this row's.

## Done when

A sheet's points read their `v` in `[−v*, v*]` for every `R` in `(−r, r]`, pinned by an apple near the horn in `chart_point`'s foot test.


## Evidence (stage 1 piece 2)

Once the revolve mints sheets, a wide apple reaches a body. The band
`|v| ≤ 2.4` of `R = 0.8, r = 1` (`v* ≈ 2.50`, past `π − 1`) between two
cones to `y = ±1.2`, built as `crates/sweep/tests/a_conical_ended_football_revolves.rs`
builds its bands, revolves, passes tier 3 and measures its exact volume
and area at ε = 1e-9 and 1e-12: the props lane lifts its own branch. The
mesh does not: `mesh::tessellate` at δ = 1e-2 refuses
`UnsupportedCurvedDomain { off_bbox: 54, first_uv: (3.03, 2.4) }`, the
sheet's rim at `v = −2.4` read a turn off, at `+3.88`. So the window is
what keeps the wide apple out of the football suite; that row of the
suite is the pin once this is fixed.

## Evidence (stage 2)

The horn is the widest apple (`v* = π`). Stage 2 builds the needle tip
(`crates/sweep/tests/the_football_revolves.rs`
`a_horn_reached_at_its_cusp_from_one_side_is_a_needle_tip`) on the
horn's upper side, `π/2 ≤ v ≤ π`, which the latitude window holds.
`topo::Chart::poles` lists the horn's one cusp once, at `v = π`, where
that window reads it. The same needle on the lower side,
`−π ≤ v ≤ −π/2`, has its meridian's lower stretch `v < 1 − π` read a
turn off, and its cusp at `+π`; it is not in the suite. A window that
holds `[−v*, v*]` for every sheet still has to place a horn's cusp at
`+π` for an upper face and `−π` for a lower one, so the fix reads the
cusp's `v` off the face, as a seam's.

## Closed (2026-11-11)

PR 6806. A sheet's `v` has one window, `Window::Sheet` (`(−π, π]`, read in the angle window at the caller's band and renamed about zero), read by both `chart_point` and the closed-form pcurve placement (`Along::Sheet`). The wide apples mesh and measure, `SHEETS` gains (0.999, 1), and `a_wide_apples_pcurves_name_v_as_its_chart_does` is red if either reader goes back to the latitude window.

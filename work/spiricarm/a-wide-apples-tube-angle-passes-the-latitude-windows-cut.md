---
id: a-wide-apples-tube-angle-passes-the-latitude-windows-cut
kind: issue
title: A spindle sheet's v is read in the latitude window (-π + 1, π + 1], which holds [-v*, v*] only for v* < π - 1: an apple with R > 0.54 r reads its far side a turn off
status: open
opened: 2026-11-03
priority: P2
cost: E
---

(SPIRICARM, stage 1 piece 1 of `horn-and-spindle-tori-have-no-representation`.)

## What

`geom::ChartInverse::point`'s `ChartArm::Spindle` arm (`crates/geom/src/chart_point.rs`) reads the tube angle `atan2(h, ρ − R)` with `Azimuth::principal_in(Window::Latitude, band)`, the window `(1 − π, 1 + π]`. The sheet's `v` is closed on `[−v*, v*]`, `cos v* = −R/r`: every lemon (`v* < π/2`) and every apple with `v* < π − 1` (`R < r·cos 1 ≈ 0.54 r`) lies inside it, but a wider apple's points at `v < 1 − π` read `v + τ`, off the sheet's window. The angle window `(−1, τ − 1]` the torus uses is worse (it cuts at `v = −1`).

A window centred on zero with its cut at `±π`, where only a horn's cusp lies, would hold every sheet; `azimuth::Window` requires its start strictly inside `(−π, 0]`, and each window's start is a measured choice (`azimuth.rs`'s `LATITUDE_WINDOW_START` docs), so the choice is this row's.

## Done when

A sheet's points read their `v` in `[−v*, v*]` for every `R` in `(−r, r]`, pinned by an apple near the horn in `chart_point`'s foot test.


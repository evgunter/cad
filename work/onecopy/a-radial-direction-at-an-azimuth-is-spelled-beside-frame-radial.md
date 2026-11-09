---
id: a-radial-direction-at-an-azimuth-is-spelled-beside-frame-radial
kind: issue
title: The unit radial direction at an azimuth is spelled by hand beside Frame::radial at five sites
status: closed
opened: 2026-11-06
priority: P3
cost: E
closed: 2026-11-09
pr: 6624
---


Found by the review of PR 6566, which exported geom's azimuthal frame reading as `geom::Frame::radial` (`u_ref·cos u + v_ref·sin u`, `crates/geom/src/frame.rs`, through `azimuth::frame`'s `Radial`) and read it in `geom_brep::curve_pair::on_surface`, retiring the region doors' own `radial`. Sites that still spell it by hand:

- `crates/topo/src/injective.rs:2210`: `(u_ref * c + v_ref * s) * radius`, beside its derivative `v_ref * c − u_ref * s` (the pair is `azimuth::frame`'s `AzimuthFrame`).
- `crates/sweep/src/revolve/tube.rs:523`: a window's start direction, `u_ref * c + v_ref * s`.
- `crates/topo/src/boolean/rim_wedge.rs:528`: a rim's point, `center + (u_ref·cos + (axis × u_ref)·sin)·radius`.
- `crates/topo/src/boolean/section_cert/cone_search.rs:188`: a local `Frame::radial` over `e1`, `e2`, with two `sin`/`cos` calls.
- `crates/topo/src/test_support_fixtures.rs:1302`: a fixture's `radial`, with two calls (test support).

Test-only spellings (`editor-core/tests/m10_9_r2_probes_interval.rs:384`, and `geom/src/azimuth.rs:519`'s bit pin) stay: a test may spell the formula it checks.

Fix: read `Frame::radial`, or `azimuth::frame` exported, where the site holds a `Frame`. Check each for bit identity against its pins; `cone_search`'s and the fixture's two-call spelling is not bitwise `sin_cos`'s.

## Closed (2026-11-09, PR 6624)

All five read geom's one formula (`azimuth::frame`):
- `injective.rs` reads `Frame::radial` and the new `Frame::tangential` (its derivative), bitwise the formula it wrote.
- `rim_wedge.rs` reads `Frame::radial`, bitwise.
- `sweep/revolve/tube.rs` reads the new `geom::radial_of_legs(u_ref, v_ref, u)`, because its legs are the rigid motion's own, read verbatim by design; `Frame`'s `v_ref` is `axis × u_ref`, equal only to rounding. `Frame::radial` reads the same formula, so there is still one spelling.
- `section_cert/cone_search.rs` and `test_support_fixtures.rs`'s `CylFrame::radial` read `Frame::radial`. Both used std's two-call `cos`/`sin`; they now take `Real::sin_cos` (libm), the chart's own trig.

`azimuth`'s bit pin covers `Frame::radial`, `Frame::tangential` and `radial_of_legs`. The test spellings stay, as the row said.

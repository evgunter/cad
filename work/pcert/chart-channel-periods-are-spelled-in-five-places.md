---
id: chart-channel-periods-are-spelled-in-five-places
kind: issue
title: Which chart channel is an angle, and its period, is spelled in geom-brep's angle_periods and in five topo copies; topo's chart v_periodic answers no for a sphere's v where angle_periods answers τ
status: open
opened: 2026-10-19
priority: P3
cost: M
---

Filed by PCERT B2b's last fix pass (H3, from the delta-2 review of
PR 5343).

## What

B2b gives geom-brep one home for which analytic chart channel is an
angle and its `Period`: `angle_periods` (`crates/geom-brep/src/pcurve_cache.rs`),
read by the harmonic lane's channel-kind check, the placement terms and
the projected lane's `off_period`. It is private to geom-brep, and topo
answers the same question in its own copies:

| site | answers |
|---|---|
| `topo::pcurves::chart_u_period` | `u`'s period: `τ` on every curved analytic chart, a net's knot length when decided closed |
| `topo::pcurves::chart_period` | `u`'s period by kind, `None` on a plane |
| `topo::joint::u_period` | `u`'s period as a deck element applies it, band-free |
| `topo::pcurves::polar_arm` | whether `v` is an angle (sphere, torus), as the radius that levers it |
| `topo::chart::Chart::v_periodic` (`crates/topo/src/chart.rs:235`) | whether `v` wraps: the torus only |

**The disagreement.** `Chart::v_periodic` answers `false` for a sphere,
whose `v` is a latitude in `[−π/2, π/2]` that does not wrap on the
chart's own domain. `angle_periods` (and `polar_arm`) answer `τ` for
the sphere's `v`, because as a formula `S(u, v + τ) = S(u, v)`, which is
what a lifted copy and the involution twin need. The two answer
different questions, "does the chart's domain wrap in `v`" against
"is `S` periodic in `v`", under one word.

## What to decide

Export one home (geom-brep's `angle_periods`, or a `Surface` method in
`geom`) for "is `S` periodic in this channel, and with what period", and
route topo's copies through it; and name `v_periodic`'s question as the
other one (the domain wraps) so the two cannot be read for each other.
PCERT unit C (unlanded at filing) adds a net's `v` period beside
`chart_u_period`, a sixth reader of the same question for spline
charts.

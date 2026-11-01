---
id: chart-channel-periods-are-spelled-in-several-places
kind: issue
title: Which chart channel is an angle or a closed net channel, and its period, is spelled in geom-brep's angle_periods and again in topo's own readers; topo's Chart::v_periodic answers no for a sphere's v where angle_periods answers τ, and the band-free ChartPeriods::stated gives an open net periods
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
| `topo::pcurves::chart_v_period` | `v`'s period and its kind (`VPeriod`): `τ` levered by the polar radius on a sphere or torus, a net's knot length when decided closed |
| `topo::pcurves::ChartPeriods::stated` | both, band-free, for `loop_lift` (no band at hand): a net's knot lengths whether or not the net closes |
| `topo::pcurves::chart_period` | `u`'s period by kind, `None` on a plane |
| `topo::pcurves::polar_arm` | whether `v` is an angle (sphere, torus), as the radius that levers it |
| `topo::chart::Chart::v_periodic` (`crates/topo/src/chart.rs:235`) | whether `v` wraps: the torus only |

`Deck::apply` reads no copy of its own: it applies the `ChartPeriods`
its caller hands it, decided at the band by `chart_boundary` and the
walk, stated by `loop_lift`.

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


**A second disagreement, band-free against decided** (PCERT unit C).
`ChartPeriods::stated` answers a net's knot length in a channel the net
is open in, where `ChartPeriods::of` answers `None`. So an element with
periods planted by hand on an open net applies through `loop_lift` and
is a gap in `chart_boundary`'s lift; tier 3 re-decides it and reads it
loud. Pinned by `joint::tests::an_elements_v_periods_are_the_charts_own`.

`stated` is read by everything that reads a loop's lift, through
`pcurves::loop_lift` (`Body::loop_lift`) and `pcurves::lifted_images`:

| reader | band in scope |
|---|---|
| `props/quad_lane.rs:598` (`cut_face_rounds`) | yes |
| `props/quad_lane.rs:790` (`spiric_face`) | yes |
| `props/quad_lane.rs:1099` (`nurbs_face`) | yes |
| `props/quad_lane.rs:1288` (`trimmed_face`) | yes |
| `chart_region.rs:2501` (`loop_uv_polygon`, the minted read) | yes |
| `boolean/boxes.rs:1244` (`face_window_steps`) | no |
| `mesh/src/trimmed.rs:1000` (`trim_polygon`, through `Body::loop_lift`) | no |
| `mesh/src/memo.rs:758` (`gather`, through `Body::loop_lift`) | no |

(Line numbers as of PCERT unit C.)

**Next step.** Give `lifted_images` and `loop_lift` a band-taking form
that lifts in `ChartPeriods::of(surface, band)`, and move the five
banded readers above onto it. Then `stated` is left to the three
band-free readers only, and each of those either has a band threaded to
it from its own callers or is named here as reading the stated periods
on purpose. The alternative is storing the decided periods with
the face when its rows are minted, so no reader decides them again. That
also retires the question of which band they were decided at.

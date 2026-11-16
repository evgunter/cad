---
id: chart-channel-periods-are-spelled-in-several-places
kind: issue
title: The band-free ChartPeriods::stated gives an open net periods that the decided ChartPeriods::of does not
status: open
opened: 2026-10-19
priority: P3
cost: M
---

Filed by PCERT B2b's last fix pass (H3, from the delta-2 review of
PR 5343).

## Narrowed

The first half of this row is answered by `onecopy/surface-chart-period`:
- one home for which analytic chart channel is an angle, and its period: `geom::Surface::angle_periods`. geom-brep's `angle_periods` retires into it, and topo's `chart_u_period`, `chart_v_period`, `ChartPeriods::stated`, `chart_period` (retired) and `polar_arm` route through it;
- `Chart::v_periodic` renamed `Chart::v_domain_wraps`, for the other question (whether the domain wraps), so the two cannot be read for each other.

What is left is the band-free against decided disagreement below.

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

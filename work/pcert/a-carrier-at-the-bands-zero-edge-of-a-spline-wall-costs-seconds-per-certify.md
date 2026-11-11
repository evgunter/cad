---
id: a-carrier-at-the-bands-zero-edge-of-a-spline-wall-costs-seconds-per-certify
kind: issue
title: A carrier at exactly the band's zero edge from a spline wall costs seconds per edge certificate
status: open
opened: 2026-11-11
priority: P3
cost: M
---

Found by `pcert/stated-image-bounded` (PR 6815, review fix LL4), measuring
the review's fuzz carriers (`review/6815-full` at `1827b743`,
`review_6815_probes.rs` `no_false_off_at`).

## What

A spline net along the ring wall's column (`spline_door_rows::ring`),
degree 1–3 with up to 11 interior knots, moved radially off the wall by
exactly the band's zero (`d = band.zero()`), stated as an iso line or a
curve in UV, costs 3.5–5.6 s per `EdgeCurve::certify` at `f64` and
17–22 s at `Interval` (debug build), on main before PR 6815 (base
`ab4fc6ca`) and after it. The same carriers at `0`, `½·zero`,
`½ band` or `0.999·escalate` cost milliseconds.

PR 6815 makes a knot-rich net about 1.4–1.5× slower at the zero edge: its
read samples the net's own breakpoints and Greville abscissae
(`spline_incidence::own_fractions`) too, and each of those samples also
walks to the cap. The review measured 1.54× (`f64`) and 1.44×
(`Interval`) for p3 k11; one call each here gave 1.39× and 1.30×. A net
with few knots (p1 k3) is unchanged:

  | carrier (`net(p, k, d)`, ring column, curve in UV) | base f64 | head f64 | base Interval | head Interval |
  |---|---|---|---|---|
  | on the wall, p3 k11 | 0.38 | 11.1 | 1.02 | 10.3 |
  | on the wall, p1 k3 | 0.04 | 4.81 | 0.23 | 4.68 |
  | at the band's zero, p3 k11 | 3876 | 5380 (1.39×) | 17403 | 22612 (1.30×) |
  | at the band's zero, p1 k3 | 4390 | 4542 (1.03×) | 17427 | 17310 (0.99×) |
  | ½ band, p3 k11 | 3.11 | 4.01 | 5.33 | 6.69 |
  | 0.999 escalate, p3 k11 | 3.15 | 3.93 | 5.24 | 6.72 |

  (ms per `EdgeCurve::certify`, debug build, one call at the zero edge and
  five elsewhere; the review's timing probe `review_6815_timing.rs` at
  `review/6815-full` `8a152a37`, run here on base `ab4fc6ca` and head.)

Measured, not profiled. The likely cost is the spline chart's read
(`spline_incidence`): at every sample the floor and the upper bound
straddle the zero edge, so each walk runs to its depth or visit cap
(`CELL_DEPTH`, `CELL_VISITS`) before it ends unsettled.

## Fix

Profile it first. If it is the walk, stop a sample's walk once an upper
bound and a floor in band of each other decide its reading, or cap the
work per read rather than per sample. Row: the four draws above under a
time bar.

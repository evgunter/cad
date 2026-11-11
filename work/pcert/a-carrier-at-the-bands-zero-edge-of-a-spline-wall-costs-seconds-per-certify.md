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
`ab4fc6ca`) and after it alike. The same carriers at `0`, `½·zero`,
`½ band` or `0.999·escalate` cost milliseconds.

Measured, not profiled. The likely cost is the spline chart's read
(`spline_incidence`): at every sample the floor and the upper bound
straddle the zero edge, so each walk runs to its depth or visit cap
(`CELL_DEPTH`, `CELL_VISITS`) before it ends unsettled.

## Fix

Profile it first. If it is the walk, stop a sample's walk once an upper
bound and a floor in band of each other decide its reading, or cap the
work per read rather than per sample. Row: the four draws above under a
time bar.

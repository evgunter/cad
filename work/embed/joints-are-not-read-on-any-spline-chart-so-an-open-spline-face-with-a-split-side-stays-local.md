---
id: joints-are-not-read-on-any-spline-chart-so-an-open-spline-face-with-a-split-side-stays-local
kind: issue
title: The joint is read on no spline chart, which a closed net needs and an open spline face does not: its side split at a vertex stays Local at every depth
status: open
opened: 2026-11-22
priority: P3
cost: M
---


## Finding

From the `embed/collinear-cover` lane and its review (PR 7322,
2026-11-22).

`crates/topo/src/injective.rs`, `joints`, reads no joint where the
chart is a spline's (`Lines::joined`). The exclusion is needed: a
closed net carries one point at two parameters of a line, so two edges
of the line `v = 0.5` over `[0.5, 1]` and `[0, 0.3]` share a vertex by
key while the line has no edge over `(0.3, 0.5)`; joined, a cell
below the line there read `Vacuous` (the review's probe R3). It is
pinned in `a_spline_charts_edges_are_not_joined`.

It is also over-broad. On an OPEN spline face a coordinate line through
a vertex is one line as on any chart, and a side split at a vertex
(the same row's first fixture, the stored ends an ulp apart) is
`Local` at every depth on both sides of the line, as every chart was
before the joint reading.

## What it calls for

A reading of whether a spline chart is injective along the line the
joint is on (its net not closed in the moving direction, no collapsed
side on it), from the net, at the door that lifts the face; `joints`
then reads a joint there. No body is known to need it: the census
bucket the lane closed is closed-form rims. R3's fixture is the guard
the change must keep.

---
id: a-chart-foot-for-an-iso-arm-is-a-fresh-nearest-projection
kind: issue
title: edge_nurbs::chart_foot places an iso arm's end by a fresh nearest projection, not from a stored chart point: the class the wall image's start is
status: open
opened: 2026-11-25
---


Filed by the `sectarms/one-image-function` lane (PR 7537), from its
second review.

## What

`geom_brep::edge_nurbs::chart_foot` (`crates/geom-brep/src/edge_nurbs.rs`)
places one point on a spline wall by `NurbsSurface::project`: the
projection's own seed grid, then Newton, the wall's nearest point found
afresh. Its consumer is the pcurve mint's rim arms, which read where an
edge's end lands on the chart (`pcurve_cache::chart_foot_lane`).

That is the class the wall image's start is
(`crates/geom-brep/src/carrier_image.rs`, `start` and `nearest`): on a
folded wall the nearest point can be on another sheet than the edge
lies on, and at a closed wall's seam it reads one of two chart points
with nothing to say which. PR 7032's decision gives the image its start
from the start vertex's stored chart point; what a rim arm's end reads
is the same question for a single point, and waits on the same answer
(the start-point decision, open on PR 7537).

## Done when

A rim arm's end is placed from the chart point the body stores for it,
or by whatever rule the image's start is given, and a row on a folded
wall and one at a seam pin it.

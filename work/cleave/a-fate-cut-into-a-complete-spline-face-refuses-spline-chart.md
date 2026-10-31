---
id: a-fate-cut-into-a-complete-spline-face-refuses-spline-chart
kind: issue
title: A fate cut's mef or mekr into a complete spline face refuses PcurveMint SplineChart: the cut is a real edge on the chart, whose row only the fitted lane derives
status: open
opened: 2026-10-31
priority: P2
cost: M
---


Filed by SPLICE's `a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex`
lane, as a hit of its sweep. It has not been witnessed on a run.

## What

`boolean::fate::chord` (`crates/topo/src/boolean/fate.rs`) writes a
kept edge's curve into a piece of the dropped face with `Body::mef`
(the `MefSite::Lone` and `MefSite::Chords` arms) or `Body::mekr`. On a
face whose rows are complete on a spline chart, with no null edge
holding a loop open, the site mint refuses before mutating:
`pcurves::site_walks` returns `SiteRowRefusal::SplineChart`, because the
new edge's row derives only through the fitted lane, which the
`Decide`-generic operators do not carry.

Unlike the pierce ring's chord strut, which `Body::ring_vertex` now
replaces with a door that owes no row, a fate cut is a real edge that
lies on the chart and stays. So no strut-free answer applies here. Its
row has to be derived on the spline chart, or the face's rows have to
be dropped until the producer's closing mint (C4: "A topology door may
drop rows mid-surgery").

## What it needs

A witness first: a boolean whose fate pass cuts a kept edge into a
minted NURBS face of the dropped operand while no null edge holds that
face open. If the pipeline cannot reach that state, close this with the
argument.

---
id: chart-arm-joins-no-spline-or-fitted-iso-image
kind: issue
title: The curved join's Chart arm reads iso families off the analytic mint only: two pieces of one iso line on a spline chart, or of a fitted image, are left unjoined
status: open
opened: 2026-10-07
---

## The finding

The curved join's `Chart` arm (`topo::boolean::edge_join`'s `locus_of`)
reads an edge's iso family through `geom_brep::chart_iso_family`, which
answers off `chart_pcurve`'s analytic derivation: `None` for a spline
chart (its `IsoLine` / `IsoArc` rows), for a fitted image, and for an
oblique section. Two pieces of one iso line on a spline chart are so
left unjoined, and `singular_at` reads no singular set for a spline
surface either. No `ci` row reaches it: the probe over every boolean
output found no valence-2 vertex between two `Chart` edges on a spline
chart.

## What it needs

The iso family of an `IsoLine` / `IsoArc` row read off its variant
(the structure the mint chose), and a regularity reading at a spline
chart's point (its partials' cross product, or the chart's degenerate
boundaries).

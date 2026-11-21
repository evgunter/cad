---
id: arc-rim-spans-reads-the-break-count-off-the-chart-not-the-rim
kind: issue
title: geom_brep::edge_nurbs::arc_rim_spans takes an arc rim's break count from the chart's interior u knots, not the rim's own arcs
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Residue of `arc-rim-mint-frontier-on-foreign-charts` (closed 2026-11-20 by the CHARTTAIL triage), filed so that row's close does not take it along.

## What

`geom_brep::edge_nurbs::arc_rim_images` breaks each arc rim at `uniform_breaks(arc_rim_spans(chart))`. `arc_rim_spans` counts the chart's interior `u` knots (`/ 2 + 1`). That is the chart's span structure, not the rim's own arc structure. The dm1 wall that exposed it (`#382`: four `u` spans, three-arc rational rims) no longer reaches the mint, because the importer now splits a wall at its interior C⁰ knots first (`step-import` `knot_split::split_c0_knots`). So every imported wall today arrives with one span per rim arc, and the count agrees by construction of the import, not by the mint's own reading.

A foreign wall whose rim arcs do not coincide with C⁰ knots (a rim broken at C¹ knots, or a chart with knots the rim does not have) would still get a count that mis-tracks the rim, and certification would refuse it `MapResidual`. The failure is a typed refusal, never a wrong body. No witness is known, which is why the row is P3 (latent).

## Owed

Read the breaks off the rim's own knot structure, or reconcile the chart's stated spans against it, with a witness wall built to have rim arcs that differ from its chart spans.

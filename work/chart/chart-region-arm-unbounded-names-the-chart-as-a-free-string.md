---
id: chart-region-arm-unbounded-names-the-chart-as-a-free-string
kind: issue
title: ChartRegionError's chart field spells the surface kind as a free string beside geom::SurfaceKind
status: open
opened: 2026-10-02
---


Found by the kind-namer sweep of TQUERY's
`one-kind-mirror-per-geometry-enum`, which moved `SurfaceKind` down to
`geom` with one `name()` and retired `step-export`'s own namer.

`crates/topo/src/chart_region.rs` `ChartRegionError::ArmUnbounded`
carries `chart: &'static str`, filled in the chart-stretch gate
(`gate(.., "sphere")`, `"torus"`, `"cone"`, and `"NURBS"` /
`"approximating surface"` for the two spline kinds). Those are
`SurfaceKind::name` with two spellings changed (`"NURBS"` for
`"nurbs"`, `"approximating surface"` for `"approx"`); the field could
carry the `geom::SurfaceKind` and the Display read `name()`. Not changed
there: it is a public payload on another program's ground.

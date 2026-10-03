---
id: chart-region-arm-unbounded-names-its-chart-by-string
kind: issue
title: ChartRegionError::ArmUnbounded carries its chart kind as a hand-written string beside geom_brep::SurfaceKind
status: open
opened: 2026-10-01
---


Found by D36's kind-namer sweep (`pcert/d36-unsupported-carrier-split`,
which retired `geom_brep`'s `chart_name` for `SurfaceKind`).

`crates/topo/src/chart_region.rs`'s `ChartRegionError::ArmUnbounded
{ chart: &'static str }` is built from hand-written literals at each
raising site (`"sphere"`, `"cone"`, `"torus"`, `"NURBS"`, ~lines 2579,
4872–4936, 5125, 5237, and `test_support_samples.rs`'s sample). The
spelling already disagrees with `geom_brep::SurfaceKind::name`
(`"NURBS"` vs `"nurbs"`), and a consumer can only tell the charts apart
by string match. Carry `geom_brep::SurfaceKind` (`SurfaceKind::of`
at the site) and render it through `SurfaceKind::name`, as
`PcurveCertifyError`'s chart payloads now do.

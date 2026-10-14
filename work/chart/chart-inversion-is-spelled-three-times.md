---
id: chart-inversion-is-spelled-three-times
kind: issue
title: A point's chart coordinates are inverted by three copies, and stable_azimuth is spelled twice
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

A point's chart coordinates (uv inversion) are computed in three places:
- `topo::chart::Chart`;
- `ChartRead`;
- `step-import`'s `chart::uv_of` (`crates/step-import/src/chart.rs:120`).

`stable_azimuth` is spelled twice:
- `crates/geom-brep/src/pcurve_cache.rs:7052`;
- `crates/topo/src/chord_join.rs:2424`.

The NURBS foot `NurbsSurface::project` is f64-only, with three wrappers.

## Owed

One inversion per surface kind, which the three callers read. A kind with no closed inversion refuses typed.

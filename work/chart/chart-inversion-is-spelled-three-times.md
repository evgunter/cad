---
id: chart-inversion-is-spelled-three-times
kind: issue
title: A point's chart coordinates are inverted by three copies, and stable_azimuth is spelled twice
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

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

Re-aimed to D3 "One reading per question". Chart coordinates are a reading of one carrier against a point. One inversion reading with one arm per surface kind, which the three callers read. A kind with no closed inversion refuses typed in that reading.

- `step-import`'s `chart::uv_of` is not exempt: D3 says import reads the same readings, with the file's coincidence distance as the band.
- `stable_azimuth` is a part of that reading, spelled once.
- D3 places one-carrier-against-a-point readings in `geom`, the lowest crate that can state them.

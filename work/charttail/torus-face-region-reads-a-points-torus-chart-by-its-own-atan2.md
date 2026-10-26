---
id: torus-face-region-reads-a-points-torus-chart-by-its-own-atan2
kind: issue
title: torus_face_region reads a point's torus chart coordinates by its own atan2, beside geom's chart reading
status: open
opened: 2026-10-24
priority: P1
cost: E
---


Found by the chart-inversion sweep (`work/chart/chart-inversion-is-spelled-three-times.md`).

## What

D3 ("One reading per question") makes a point's chart coordinates one reading, `geom::ChartInverse::point` (`crates/geom/src/chart_point.rs`), which `topo::chart::Chart`, `shell`, ring re-homing in `chord_join` and step-import now read. `torus_face_region` (`crates/topo/src/boolean/torus_region.rs`) still spells the torus arm itself:

- the azimuth of an edge point, `w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref))` (twice, near `torus_region.rs:760` and `:829`), raw `atan2` in `(−π, π]`;
- the tube angle, `h.atan2(rho - major)` (near `:872`).

Each is then placed by `near_span` against the edge's span, so the window the raw read lands in does not reach the verdict; the read is still a second spelling of the reading's torus arm, with `atan2`'s cut at `π` rather than the angle window's (C4).

## Owed

Read the point through `geom::Surface::chart_point` at the face's band (or `geom::ChartInverse` built once for the face) and keep `near_span`'s placement. Report any moved baseline.

## Slate triage (2026-10-26, CLEAVE)

- Banded P1: a second spelling of `ChartInverse::point`'s torus arm: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Priced `E`, the effort the body describes.
- Moved to CHARTTAIL (`work/charttail/`) by `git mv`, id and body unchanged: which holds CHART's rows since CHART folded in today: chart readings and the region doors. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

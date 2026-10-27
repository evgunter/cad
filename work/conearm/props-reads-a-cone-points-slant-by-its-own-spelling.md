---
id: props-reads-a-cone-points-slant-by-its-own-spelling
kind: issue
title: props reads a cone point's chart v (its slant) by its own spelling, beside geom's chart reading
status: open
opened: 2026-10-24
priority: P3
cost: E
---


Found by the chart-inversion sweep's second pass (`work/chart/chart-inversion-is-spelled-three-times.md`, PR 5659), shaped at the cone's `h / cos α`.

## What

D3 ("One reading per question") makes a point's chart coordinates one reading, `geom::ChartInverse::point` (`crates/geom/src/chart_point.rs`), whose cone arm reads `v = h / cos α`. `crates/geom-brep/src/props/curved.rs` spells the same coordinate inline:

- `cone_loops`' `slant` closure, `(p - apex).dot(axis) / cos_a` (near `curved.rs:1982`);
- the cone's linear-boundary levels: each generator's two ends and each rim's centre, `(… - apex).dot(axis) / cos_a` (near `curved.rs:2809`, `:2810`, `:2820`).

These are the same expression as the reading's, so the values agree today. They are still a second spelling of one reading's arm, and they would drift from it if the cone's v changes: `work/pcert/the-projected-image-inverts-chart-points-by-a-second-spelling.md` asks whether a cone point's v is `h / cos α` or its ruling foot.

## Owed

Read these levels through `geom::ChartInverse` (built once per face) at the props band, or state why a props level is a record rather than a reading. Report any moved baseline.

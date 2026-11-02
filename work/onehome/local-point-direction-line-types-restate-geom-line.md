---
id: local-point-direction-line-types-restate-geom-line
kind: issue
title: Five local (point, direction) line types restate geom::Line
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the review of PR 6328, which made `geom::Line { origin, dir }` the line `Curve3::Line` holds and the home of a point's foot and parameter on a line (`Line::foot`, `Line::param`).

Five local types carry the same two fields under their own names, and their readers read them by hand (DESIGN `[one-way]`: two types carrying the same fields are a sign of a cleaner arrangement to find):

- `crates/topo/src/boolean/rim_wedge.rs` `Locus::Line { origin, dir }`: its `at` and parameter readers (`origin + dir·s`, `(p − origin)·dir`) restate `Curve3::eval` and `Line::param`.
- `crates/geom-brep/src/locus.rs` `TangentLocus::Line { origin, dir }`.
- `crates/geom-brep/src/pair_section/gap.rs` `Spine::Line { origin, axis }`.
- `crates/topo/src/census/depth_gap.rs` `Depth::Axis { origin, axis }`.
- `crates/topo/src/boolean/region.rs` `Line { start, dir }`, a leg's course: already owed by charttail's `the-region-legs-spell-a-circle-and-a-line-beside-curve3` (that row is the one to work there; this one only cross-references it).

Fix: each variant holds a `geom::Line` (`Locus::Line(geom::Line<T>)`, …) and reads its foot, parameter and point through it. Where a variant adds a fact (a certified unit `dir`, a leg's start), it holds that beside the line, not instead of it.

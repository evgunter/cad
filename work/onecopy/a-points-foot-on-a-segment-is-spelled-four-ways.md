---
id: a-points-foot-on-a-segment-is-spelled-four-ways
kind: issue
title: A point's foot on a segment (the line's parameter held to the segment) is spelled four ways in topo, with no home beside geom::Line
status: open
opened: 2026-11-03
priority: P2
cost: E
---



Found by the sweep of `onecopy/small-reads` (shape `(p − origin)·dir` and `dir·(p − origin)` over `crates/topo/src`), which moved the unclamped line parameters and feet in topo onto `geom::Line::param` and `geom::Line::foot`.

A point's foot on a segment is the line's parameter held to `[0, 1]` (or to `[0, len]` on a unit direction), and the point there. `geom::Line` has no such reading, so each site spells it by hand:

- `crates/topo/src/sector_shape.rs` `point_segment_distance`: `(w·d / |d|²)` clamped, then `|w − d·t|`. Its callers include `boolean/solid_contain.rs` `EdgeMeets::vertex`, which reads the ray's own parameter beside it inline, `(at − q)·d`.
- `crates/topo/src/link.rs` `foot` (the segment `a`–`b`'s nearest point), `((p − a)·ab / |ab|²)` clamped.
- `crates/topo/src/link.rs` `lowest_on`'s ray arm: `apex + u·max(u·(q − apex), 0)`, a ray's foot (held at one end only).
- `crates/topo/src/census.rs` `distance`, its edge arm: `(u − p0)·dir` held to `[0, len]` on a line edge's segment (`EdgeGeo`).

Test-only spellings beside them, which may stay as independent oracles: `ring_path.rs` `segment_arc_distance` (`#[cfg(test)]`).

Before you fix it, decide the home: a `Line::param` held to a parameter span (the edge's own `(t0, t1)`) fits all four, since a segment is a line edge's span; a `Segment` type would be a second holder of the line. `sector_shape`'s Body-free functions are moving to `geom-brep` under ONEDOOR's `crossing-doors-leave-topo-for-geom-brep`, so coordinate with that row.

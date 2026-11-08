---
id: a-fourth-conic-arc-spelling-beside-three
kind: issue
title: bound_side's ConicArc is a fourth spelling of a conic edge's arc, and ArcConic repeats Curve3's arms
status: open
opened: 2026-11-03
priority: P3
cost: M
---

Found by PR 6385's re-review.

`topo/src/bound_side.rs` `ConicArc` (≈338) and its `ArcConic` (≈346),
added by PR 6385 to read a conic edge's point at an arc length, are a
fourth spelling of "a conic edge's arc from a vertex", beside:
- `splitting/containment.rs` `ConicArc` (≈806);
- `ring_path.rs` `ConicArc` (≈593);
- `boolean/boxes.rs` `arc_cover` (≈2071).

`ArcConic` also repeats `geom::Curve3`'s `Circle` / `Ellipse` arms only
to rebuild the `Curve3` it evaluates. Owed: one arc type the four read,
or `bound_side` reading one of the existing ones.

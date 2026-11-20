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

Moved from TANGRIM to TANG on 2026-11-20, by `git mv`, when TANGRIM merged into TANG: TANGRIM held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one TANG carries.

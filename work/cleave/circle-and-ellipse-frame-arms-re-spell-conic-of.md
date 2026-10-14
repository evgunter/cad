---
id: circle-and-ellipse-frame-arms-re-spell-conic-of
kind: issue
title: LoopArc::of and the section outline's Circle and Ellipse arms re-spell geom_brep::Conic::of
status: open
opened: 2026-10-14
---

Found by the class sweep on `cleave/carrier-cross-one-curve`, which
built `carrier_cross.rs`'s and `carrier_speed.rs`'s conic frames through
`geom_brep::Conic::of` instead of re-spelling its Circle and Ellipse
arms.

Two more local frames re-spell those arms field by field:

- `crates/topo/src/ring_path.rs` `LoopArc::of`: a Circle arm and an
  Ellipse arm, each building `Conic { centre, axis, u_ref, a, b, .. }`.
- `crates/topo/src/splitting/section_loops.rs`, the outline walk's
  `OutlineEdge::Conic` arms: `a: u_ref * radius` / `u_ref * major`,
  `b: axis.cross(u_ref) * radius` / `* minor`.

Each can read `Conic::of(carrier)` once and keep only what differs (the
span; the outline's ellipse flag). Doing so also drops their reads of
`Curve3::Ellipse`'s field names, which PR 4861 renames.

`geom_brep::Conic::semi_axis_vectors` (added on the same branch) is the one
spelling of the frame `(semi_u·u_ref, semi_v·v_ref)`; the outline's
`a`/`b` are exactly it.

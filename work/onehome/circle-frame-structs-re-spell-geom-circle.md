---
id: circle-frame-structs-re-spell-geom-circle
kind: issue
title: Four topo and sweep structs re-spell geom::Circle's fields: rim_wedge::Rim, surgery::CircleFrame, ring_path::CircleArc, contain::LoopCircle
status: open
opened: 2026-11-02
---


Found by the conic-frame unit's shape sweep (`onehome/conic-frame`), which
grepped for types carrying `center`, `axis` and `u_ref` (DESIGN
`[one-way]`: two types carrying the same fields are a sign of a shared
part to name). The unit made `geom::Circle { frame: ConicFrame, radius }`
the one circle and deleted the conic copies its row listed; these four
carry the circle's fields and were outside that list:

- `topo::boolean::rim_wedge::Rim` (`crates/topo/src/boolean/rim_wedge.rs`):
  `{ center, axis, radius, u_ref }`, exactly `geom::Circle`'s fields, with
  its own angle reading (`locus_param`, `atan2` on the in-plane
  components, `Conic::angle_of`'s). About 87 uses.
- `sweep::blend::surgery::CircleFrame` (`crates/sweep/src/blend/surgery.rs`):
  `{ center, axis, radius, u_ref }` built by `CircleFrame::of(carrier)`
  from a `Curve3::Circle`, "destructured once": a copy of the carrier's
  own `geom::Circle`.
- `topo::ring_path::CircleArc` (`crates/topo/src/ring_path.rs`): a path
  piece `{ centre, axis, radius, u_ref, span }` with its own `v_ref`,
  point and `param` (`on_span_branch` of its `atan2`); a `geom::Circle`
  and a span.
- `topo::boolean::contain::LoopCircle` (`crates/topo/src/boolean/contain.rs`):
  `{ center, axis, radius }`, no seam; the reading of a loop's circle,
  perhaps legitimately its own (check 9 reads only the centre and the
  radius).

Fix: each holds a `geom::Circle` beside what it adds (a span, a loop
identity) and reads the `Conic` trait; `LoopCircle` decides whether it is
a circle or a narrower reading.

---
id: a-points-offset-from-a-plane-has-no-home-in-geom
kind: issue
title: A point's signed offset from a plane is homed in geom-brep's sector_shape and re-spelled inline across topo
status: dispatched
opened: 2026-11-06
branch: onecopy/feet-and-params
---


Split out of `a-points-foot-on-a-segment-is-spelled-four-ways` by `onecopy/feet-and-params`. That row asked its fix to say where `plane_offset` and `plane_offset_rounded` go. The fix homed a segment's foot on `geom::Line`, and these two do not fit there. They read a plane, not a line, and `Line::param` divides by `dir·dir`, which would break `plane_offset_rounded`'s contract that its value is bit-identical to `plane_offset`'s.

- `crates/geom-brep/src/sector_shape.rs` `plane_offset` (`(q − origin)·normal`) and `plane_offset_rounded` (the same value with its first-order rounding bound): general point readings homed in a module about a corner's sectors. Nine callers in topo (`splitting/finish.rs`, `splitting/classify.rs`, `boolean/reduce.rs`, `bound_side.rs`).
- The same expression inline beside them, a first sweep with the shape `(p − *origin).dot(*normal)` over `crates/`: `splitting/containment.rs` (`off` near `:1939`), `splitting/classify.rs` (~`:387`), `census.rs` (`distance`'s face arm, the face residuals near `:1603`, `:1652`, `:3302`, the edge-end residuals near `:3570`, and `:10432`), and `boolean/reduce.rs` (~`:1520`). `Frame::height` (`crates/geom/src/frame.rs`) is the same reading on a frame's own plane.

Before you fix it, decide the home. A plane has no value type in `geom`: `Surface::Plane` is a carrier with a frame, and the census and splitting hold their own `origin`/`normal` pairs. That choice is a design fork. Weigh it first, and say whether `plane_offset_rounded`'s bit-identity claim moves with the reading.

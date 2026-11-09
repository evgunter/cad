---
id: plane-and-line-tuples-restate-the-placement
kind: issue
title: (Point3, Vec3) tuples stand for a plane or a line at several doors
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Found by the blind-spot pass of the frame-readings unit's holder sweep (the field-shape grep cannot see a tuple). The unit retired two (step-import's `type Frame` tuple and `topo::boolean`'s `DecidedLine`); a `(Point3<T>, Vec3<T>)` still stands for a plane or a line at:

- `crates/geom-brep/src/intersect.rs` `SectionPlane::site` (a plane's point and normal);
- `crates/topo/src/census/depth_gap.rs` `Read::plane`, and `least_point`'s `(o, n)` argument;
- `crates/sweep/src/blend/reach.rs` `frame()` and the `sheet` field;
- `crates/topo/src/boolean/solid_contain.rs` (the function returning `Option<(Point3<T>, Vec3<T>)>`);
- `crates/topo/src/offset_axial.rs` the `seed`;
- `crates/topo/src/ring_path.rs` `plane_roots`' `(origin, e)` argument.

Sweep: `\(Point3<T>, (geom_core::)?Vec3<T>\)` in type position over `crates/*/src`. Blind spot: an `f64`-only tuple (`(Point3<f64>, Vec3<f64>)`) and a pair passed as two arguments rather than one tuple; the second is the loose-argument door the placement decision names, and is not swept here.

Fix: a line is a `geom::Line`. A plane with no seam has no type: `Frame` carries a seam, so whether a seamless plane gets its own (`Line` read as its normal, as `Line::square_foot` reads it, or a named plane) is a question for the kind-struct stage.

## Holders found by `onecopy/feet-and-params` (second unit)

That unit gave a point's offset from a plane one home, `geom::plane_offset{,_rounded}` beside `Frame::height` (which reads it). It ported the inline `(q − origin)·normal` copies onto it. These sites hold a seamless plane as a loose `(origin, normal)` pair, so they read the free function and are this row's to type:
- struct fields: `topo` `census.rs` `FaceGeo` (`origin`, `normal`); `splitting/mod.rs` `SplitPlane` (`origin`, `normal: UnitVec3`); `shell.rs` `PlanarFace`; `chord_join.rs` `SectionCtx` (`origin`, `normal`); `ring_path.rs`'s section (`sec.origin`, `sec.normal`).
- loose arguments or destructured fields: `topo` `bound_side.rs` (`Chord { base, far }` against `n`), `boolean/reduce.rs`, `vtxfac.rs`, `node.rs`, `contain.rs`, `sectors.rs`, `solid_contain.rs`, `trace.rs`, `ops.rs`, `factored_crossing.rs`, `attach.rs`, `splitting/containment.rs`, `census/point.rs`, `offset_derive.rs`; `geom-brep` `curve_face/plane_crossing.rs` (`plane_origin`, `plane_normal`), `offset.rs`, `implicit.rs`; `editor-core` `eval/measure.rs`; `step-import` `recognize.rs`.

The same unit also found a line family travelling as `(Point3<T>, Vec3<T>)`. `geom-brep` `intersect.rs` `cylinder_axes_parallel{,_reported}`, `parallel_swing`, `axes_feet` and `separation_range` take axis tuples, as do `pair_section/torus.rs` `axis_pose` and `torus_arms.rs` `square_pose`. `intersect.rs` now converts these at one adaptor, `axis_line`, because `Reach::foot_on` takes a `geom::Line`. Retyping the family as `geom::Line` retires the adaptor.

Not ported, and a different reading: the sites that divide by `normal.norm()` or normalize the normal first. These are a plane offset for a non-unit normal: `boolean/slit.rs`, `step-import/src/adopt.rs`, `geom-brep/src/edge_nurbs.rs`, `pcurve_cache.rs`, and `sweep/src/blend/reach.rs` (×2).

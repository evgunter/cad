---
id: plane-and-line-tuples-restate-the-placement
kind: issue
title: (Point3, Vec3) tuples stand for a plane at several doors
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Found by the blind-spot pass of the frame-readings unit's holder sweep (the field-shape grep cannot see a tuple). The unit retired two (step-import's `type Frame` tuple and `topo::boolean`'s `DecidedLine`); a `(Point3<T>, Vec3<T>)` still stands for a plane or a line at:

- `crates/geom-brep/src/intersect.rs` `SectionPlane::site` (a plane's point and normal);
- `crates/topo/src/census/depth_gap.rs` `Read::plane`, and `least_point`'s `(o, n)` argument;
- `crates/topo/src/boolean/solid_contain.rs` `FaceGeo::Plane(Point3<T>, Vec3<T>)` (~`:740`), a planar face's point and outward normal;
- `crates/topo/src/ring_path.rs` `plane_roots`' `(origin, e)` argument.

The line half is done (`onecopy/line-tuples`): every tuple that stood for a line now carries a `geom::Line`, among them `reach.rs`'s `frame()` and `sheet` field and `offset_axial.rs`'s `seed`, first listed here. The line holders that unit left are filed as `a-line-still-travels-as-loose-arguments-or-a-frame-pair`.

The same unit's sweep found these plane tuples, all a point and a normal:
- `geom-brep`: `curve_pair/on_surface.rs` (~`:80`, `:369`, `:451`); `intersect/conic_arc.rs` (~`:277`); `extent.rs` (~`:212`, `:426`); `curve_face/plane_crossing.rs` (~`:585`).
- `topo`: `face_normal.rs` `face_plane` (~`:158`); `ring_path.rs` (~`:1848`); `census/point.rs` (~`:133`); `boolean/reduce.rs` (~`:662`); `boolean/section_cert.rs` (~`:1055`).
- `sweep`: `blend/open/end_face.rs` `Triangle` (~`:338`).
- `editor-core`: `trace.rs` `plane_in_body` (~`:449`).
- Test-only: `ring_path/cone_islands.rs`, `ring_path/torus_islands.rs`, `chord_join/sphere_island_rows.rs`, `census/pierce_fuzz.rs`, `section_cert_rows.rs` `Plane`, `pair_section/spline_implicit.rs` tests.

Not planes: `ders1`, `jet_at`, `polish`'s and `join`'s germs, `offset_derive.rs` `running_as` and `cone_pair_search.rs` `on_cone`. Each is a point and its tangent or normal sample.

Sweep: `\(Point3<T>, (geom_core::)?Vec3<T>\)` in type position over `crates/*/src`. Blind spot: an `f64`-only tuple (`(Point3<f64>, Vec3<f64>)`) and a pair passed as two arguments rather than one tuple; the second is the loose-argument door the placement decision names, and is not swept here.

Fix: a plane with no seam has no type: `Frame` carries a seam, so whether a seamless plane gets its own (`Line` read as its normal, as `Line::square_foot` reads it, or a named plane) is a question for the kind-struct stage.

## Holders found by `onecopy/feet-and-params` (second unit)

That unit gave a point's offset from a plane one home, `geom::plane_offset{,_rounded}` beside `Frame::height` (which reads it). It ported the inline `(q − origin)·normal` copies onto it. These sites hold a seamless plane as a loose `(origin, normal)` pair, so they read the free function and are this row's to type:
- struct fields: `topo` `census.rs` `FaceGeo` (`origin`, `normal`); `splitting/mod.rs` `SplitPlane` (`origin`, `normal: UnitVec3`); `shell.rs` `PlanarFace`; `chord_join.rs` `SectionCtx` (`origin`, `normal`); `ring_path.rs`'s section (`sec.origin`, `sec.normal`).
- loose arguments or destructured fields: `topo` `bound_side.rs` (`Chord { base, far }` against `n`), `boolean/reduce.rs`, `vtxfac.rs`, `node.rs`, `contain.rs`, `sectors.rs`, `solid_contain.rs`, `trace.rs`, `ops.rs`, `factored_crossing.rs`, `attach.rs`, `splitting/containment.rs`, `census/point.rs`, `offset_derive.rs`; `geom-brep` `curve_face/plane_crossing.rs` (`plane_origin`, `plane_normal`), `offset.rs`, `implicit.rs`; `editor-core` `eval/measure.rs`; `step-import` `recognize.rs`.

Not ported, and a different reading: the sites that divide by `normal.norm()` or normalize the normal first. These are a plane offset for a non-unit normal: `boolean/slit.rs`, `step-import/src/adopt.rs`, `geom-brep/src/edge_nurbs.rs`, `pcurve_cache.rs`, and `sweep/src/blend/reach.rs` (×2).

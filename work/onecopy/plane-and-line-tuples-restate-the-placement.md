---
id: plane-and-line-tuples-restate-the-placement
kind: issue
title: A plane travels as a (point, normal) tuple or pair at several doors; waits on stage 3's Plane
status: parked
opened: 2026-11-03
priority: P2
cost: M
blocked_on: [analytic-surfaces-spell-their-frame-field-by-field]
---


Found by the blind-spot pass of the frame-readings unit's holder sweep (the field-shape grep cannot see a tuple). The unit retired two (step-import's `type Frame` tuple and `topo::boolean`'s `DecidedLine`); a `(Point3<T>, Vec3<T>)` still stands for a plane or a line at:

- `crates/geom-brep/src/intersect.rs` `SectionPlane::site` (a plane's point and normal);
- `crates/topo/src/census/depth_gap.rs` `Read::plane`, and `least_point`'s `(o, n)` argument;
- `crates/topo/src/solid_contain.rs` `FaceGeo::Plane(Point3<T>, Vec3<T>)` (~`:740`), a planar face's point and outward normal;
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

## Narrowed to the plane half (2026-11-09, PR 6665)

The line half is done: `onecopy/line-tuples` (PR 6601), then
`onecopy/line-placement` for the loose pairs filed as
`a-line-still-travels-as-loose-arguments-or-a-frame-pair`. What remains
is the plane holders listed above: the `(Point3<T>, Vec3<T>)` tuples, the
loose `(origin, normal)` pairs and struct fields, and the `f64`-only
tuples the first sweep could not see.

**They wait on stage 3, the kind-struct sweep.** The placement fork is
decided on `analytic-surfaces-spell-their-frame-field-by-field`
("Decided", 2026-11-03): `Surface::Plane(Plane { frame })`, a geom struct
holding one `Frame`, and no `Plane::origin()` or other re-spelling of
`frame.origin`. Minting a `Plane { origin, normal }` here would take that
name with a second meaning, and `Line` read as a plane's normal would give
`Line` two meanings. So no plane type lands before stage 3. That row
cross-references this one so stage 3 ports these holders onto the
`Plane` it mints.

**A question for the stage-3 implementer, not a choice made here.**
Nearly every holder above is a plane with no seam (a point and a normal,
no `u_ref`), while `Frame` carries one. Does a seamless plane need its
own reading (a `Plane` built from a `Frame` whose seam no reader reads,
or a seam-free placement type)? Or do these holders read a `Frame` with
a conventional seam?

## Parked on stage 3 (2026-11-09, `onecopy/placement-holders`)

Nothing here can move without the stage-3 `Plane`: every holder left is a plane, and the orchestrator ruled that no plane type lands before stage 3. Beside the listed `intersect/conic_arc.rs` tuple (`OpenConicSection::new`'s `(q, n)`), the section's own placement now holds a `geom::Frame` (`placement-holders-the-frame-sweep-found`); the tuple stays a plane's.

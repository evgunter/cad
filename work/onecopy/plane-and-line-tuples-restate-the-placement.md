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

---
id: vec3-point3-const-and-conversion-doors
kind: issue
title: Vec3::new/Point3::new are not const fn and there is no Vec→Point conversion door — the two spellings the lily rewrite could not route through a door
status: open
opened: 2026-09-05
---


(PROPS orchestrator) From the lily-vec3 lane (PR #1954), the two
residual non-door spellings after `demos/tour/src/lily.rs` was
rewritten through `geom_core`'s doors:

- `Point3::new` / `Vec3::new` are not `const fn`, so a `const
  Point3<f64>` (a scene constant) can only be a struct literal. A
  `const fn new` for the concrete field types is the obvious door; a
  generic `T: Real` constructor cannot be `const` today, so the shape is
  a decision (inherent `const fn` on `Vec3<f64>`/`Point3<f64>` only, or
  a literal is fine and the docs say so).
- No `Vec2 → Point2` / `Vec3 → Point3` conversion door; `Point2::origin()
  + v` is the spelling. `From<Vec3<T>> for Point3<T>` is one line; whether
  the kernel wants an affine/linear conversion to be that quiet is the
  question (D2-shaped: a point is not a vector).

Both are small E decisions; neither blocks anything.

- (From the lily style review, 2026-09-05.) No `Affine3::from_frame(origin, u, v)`:
  `lily.rs` (two sites) and `skinned.rs::normal_start_place` build a
  `SketchPlane<f64>` only to take `.placement`; the shared thing has no
  home. And no `SketchPlane::map`/lift, so a `from_frame` boundary lifts
  three times. Both are the same door-shaped hole as the first bullet:
  the frame-from-three-vectors constructor, and the lift of the type
  that carries it.

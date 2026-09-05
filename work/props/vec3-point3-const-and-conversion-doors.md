---
id: vec3-point3-const-and-conversion-doors
kind: unit
title: Vec3::new/Point3::new are not const fn and there is no Vec→Point conversion door — the two spellings the lily rewrite could not route through a door
status: closed
opened: 2026-09-05
closed: 2026-09-05
pr: 1977
branch: props/vec3-doors
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

## Closed

Landed as PR #1977 (an E rider, single style review, outside the A/B
experiment). `Vec2/Vec3/Point2/Point3::new` are `const fn`, generic as
they are — the bodies are struct literals and call nothing on `T`, so
the `T: Real` bound costs nothing at 1.97.0; the doctest at
`Point3::new` reads a constant of each of the four types. The frame
constructor has one home, `Affine3::from_frame(origin, u, v)`, with
`SketchPlane::from_frame` delegating to it bit-identically
(`crates/profile/tests/sketch_plane.rs`, twelve components by bits
over a corpus including signed zeros); `SketchPlane::map` lifts a
stored frame once through `Affine3::map`, its doc naming the two lift
spellings; `demos/tour/src/skinned.rs::normal_start_place` reads the
door. `lily.rs`'s two `from_frame(…).placement` sites and its three
struct-literal constants are the tour-wide layer-rule sweep's
(`work/issues/tour-scenes-lift-componentwise-not-through-map.md`), not
this unit's; `teapot.rs`'s three constants are named in PR 1977's sweep
for the orchestrator to place.

**Ruling (not to be re-asked): there is no `From<Vec3<T>> for
Point3<T>`, nor the 2-D twin.** A point is not a vector; the
affine/linear split is a kernel decision (D2-shaped), and an implicit
conversion would let a displacement be read as a position at every
`.into()`. `Point3::origin() + v` is the spelling, and the reason is
written once, at `Point3`'s type doc.

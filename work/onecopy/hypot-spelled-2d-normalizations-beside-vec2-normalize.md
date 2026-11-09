---
id: hypot-spelled-2d-normalizations-beside-vec2-normalize
kind: issue
title: Three 2-D directions are normalised by hand through f64::hypot beside Vec2::normalize and its checked form
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by `onecopy/length-rule`'s second sweep pass (a norm spelled
`hypot` on loose components rather than `Vec2::norm`, which the first
pass's `.norm()` pattern could not match).

## What

Each takes `a.hypot(b)`, refuses it unless it is finite and positive,
and divides both components by it:

- `step-import` `recognize_curve.rs`, the turn-margin check of a
  span's control polygon (`let norm = sx.hypot(sy)`, about line 645);
- `viewer` `sketch.rs`, the segment direction at a polyline index
  (`let length = dx.hypot(dy)`, about line 2479);
- `geom-brep` `pair_section/spline_implicit.rs`, the descent direction
  `(gu, gv) / g` (about line 613).

`hypot` neither over- nor underflows short of the true length, so its
refusal set is not `geom_core::LengthOutOfFormat`'s: a `(1e200, 0)`
pair passes it and fails `Vec2::checked_direction_and_length`. Routing
them to the home therefore moves which inputs are refused.

## Owed

Decide per site whether it is a door that refuses out-of-format
vectors (route to `Vec2::checked_direction_and_length`, accepting the
narrower admission) or a total reading that only needs the direction
(route to `Vec2::normalize` with a zero guard), and route it.

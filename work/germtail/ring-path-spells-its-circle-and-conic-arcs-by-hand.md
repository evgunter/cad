---
id: ring-path-spells-its-circle-and-conic-arcs-by-hand
kind: issue
title: ring_path spells its circle and conic arcs by hand beside Curve3
status: open
opened: 2026-10-24
priority: P2
cost: M
---

## What

`crates/topo/src/ring_path.rs` keeps its own copies of carriers that
`geom::Curve3` already states, each with a hand-written evaluator:

- `CircleArc` (`ring_path.rs:436`): `at` and `tangent` written out as
  `centre + (u_ref·cos t + (axis × u_ref)·sin t)·radius` (`:449`,
  `:455`), where `Curve3::Circle`'s `eval`/`deriv` say the same.
- `Conic` (`ring_path.rs:537`): `at` (`:1341`) is
  `Curve3::Ellipse`'s evaluation by hand, `a = b` for a circle.
- `Oval` (`ring_path.rs:558`) restates `Curve3::Spiric`'s form.

This is the shape the region skeleton just retired
(`work/chart/the-sphere-ray-and-the-circle-leg-stand-outside-the-region-skeleton.md`,
item (e)): the sphere region door's `Circle::at` and the torus and ruled
doors' restricted carrier enums now read the edge's own `Curve3`. Two
spellings of one carrier's evaluation can differ in their last bits,
and every reading past the evaluation inherits whichever one it took.
Found by that unit's sweep, outside its fence.

## Owed

Read the arc's own `Curve3` (`eval`, `deriv`) where these types
evaluate, and keep only what a ring path adds (the span, the `own`
loop). Re-baseline any bit that moves.

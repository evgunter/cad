---
id: a-half-turn-spine-sweeps-only-off-its-exact-tangents
kind: issue
title: sweep_body refuses a planar half-turn spine with exact end tangents (PathTangentReversal) and builds it only off a float knife edge
status: open
opened: 2026-10-02
priority: P2
cost: M
---

Found by SHOW's `klein-scene-should-adopt-the-one-body-loop-sweep`
review (2026-10-02). The Klein bottle's top loop (`demos/tour/src/klein.rs`,
`sweep_loop`) is ONE `sweep_body` of an annulus along a planar U-turn
spine — a 270° arc, then a 90° arc back onto the axis, a half turn
from start to end — and the scene depends on the behaviour below.
Pinned live as the bottle's wall 9.

## What

`sweep::sweep_places` places every station by ONE minimal rotation
from the BASE tangent (the C6 note above `sweep_geometry` in
`crates/sweep/src/skin.rs`). On a spine whose end tangent is the start
tangent reversed, the last station's rotation has no axis:

- **the exact spine refuses.** The two arcs authored as four rational
  quarter arcs (`NurbsCurve3::new`, degree 2, weights 1, √½, 1, knots
  doubled at the joints), end tangents exactly +z and −z, refuses
  `LoftError::Skin(SkinError::PathTangentReversal { station: N−1 })`
  at 12, 13 and 14 stations (`klein::exact_spine`, wall 9);
- **the same control net sampled through trig builds.** Computing the
  quarter points as `RLOOP·(1 − cos θ)`, … leaves ~2e-16 in one
  control point's x, which tilts the end tangent off −z, and the
  sweep builds;
- **the scene's spine builds.** It is `NurbsCurve3::interpolate`
  through 49 sampled points; its end tangents tilt 3.99e-4 rad in
  mirror image, so |u₀ × u₁| = 3.09e-11 and `sin > 0.0` holds. That is
  the C6 knife edge `review_m5_pr10::review_half_turn_path_builds_on_the_float_knife_edge`
  pins "as executed behaviour, not as intent".

So a planar half-turn spine sweeps only when float noise breaks its
symmetry, and the one spelling with the right end tangents is the
one that refuses. If the frame choice gains the named angular
predicate with a real margin that the C6 note asks for, the klein
scene's loop refuses too.

Nor can a user avoid the knife edge: no public door pins a spine's
end tangents. `NurbsCurve3::interpolate` takes no end derivatives,
and nothing joins two exact arcs into one path, short of writing the
rational control net by hand — which is the spelling that refuses.

## The ask

- **A planar U-turn spine sweeps exactly**: carry the frame ALONG the
  path (station to station — a rotation-minimizing frame propagated
  incrementally, or a planar path's own normal-plane frame) rather
  than from the base, so no station's frame depends on the
  anti-parallelism of two distant tangents.
- Rider asks, each of which would let the scene author the exact
  spine through a door: an interpolation door that takes end
  derivatives, or a curve-join door that composes two exact arcs into
  one `NurbsCurve3`.

Neighbour: `work/frame/path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start`
(the start-frame door's naming, and its interior-station callers).

## Done when

The klein bottle's wall 9 stops refusing — the exact spine sweeps —
and `review_half_turn_path_builds_on_the_float_knife_edge` asserts
the half-turn build as intended behaviour rather than as a knife edge.

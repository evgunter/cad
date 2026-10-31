---
id: a-half-turn-spine-sweeps-only-off-its-exact-tangents
kind: issue
title: sweep_body refuses a planar half-turn spine with exact end tangents (PathTangentReversal) and builds it only off a float knife edge
status: closed
opened: 2026-10-02
priority: P0
cost: M
refs: [sweep-frame-is-a-minimal-rotation-from-the-start-tangent]
rides_with: sweep-frame-is-a-minimal-rotation-from-the-start-tangent
closed: 2026-10-31
pr: 6138
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

## The knife edge is gone; both spines refuse now (2026-10-14, PATHS' `the-sketch-plane-is-its-frame`)

`sweep_places` turns the start frame by `RigidMotion::turned_between`
and decides its divisor `1 + t₀·tᵢ` under the band (`sweep_path_turn`,
ledger row F22) instead of testing `sin > 0.0` exactly. The scene's
interpolant (|u₀ × u₁| ≈ 3.1e-11) now decides that divisor zero and
refuses `PathTangentReversal` at its last station, as the exact spine
does; `review_m5_pr10`'s knife-edge row became
`review_half_turn_path_refuses_at_the_turns_decision`.

The klein scene therefore no longer sweeps its loop. It lofts the same
section through the same stations' normal planes, each frame stated by
the user (`klein::lofted_loop`: local +Z the unit tangent, local +X
the start frame's roll, which is the planar spine's normal), and wall 9
pins `sweep_body` refusing on the exact spine. So the first ask above
(carry the frame ALONG the path, or a planar path's own normal-plane
frame through a door) is now what stands between the scene and
`sweep_body`, and the "Done when" reads: the klein loop sweeps with
`sweep_body` again. Untried here: whether `lofted_loop` builds on
`exact_spine`, which would let the scene put its caps flat on the rims
without any kernel change.

## Raised to P0 (on request in chat, 2026-10-25)

Raised to P0: a planar half-turn is an ordinary sweep path, and the
klein scene's loft through stated frames is a workaround for this
refusal, not a construction anyone should have to write. The scene's
comments and `demos/README.md` now say so, and wall 9's retirement
instruction is the switch back to `sweep_body` on `exact_spine`.

## The fix is `carve/sweep-frame-is-a-minimal-rotation-from-the-start-tangent` (2026-10-25)

Linked on request in chat. That row is the same root, measured on the
square coil: `sweep_places` carries every station by the minimal
rotation from the START tangent, so the frame spins near anti-parallel
and has no axis at it. Its shape, an incremental rotation-minimizing
frame or a stated frame law, is this row's first ask. It is
`design: true` (which frame a sweep follows), so this P0 waits on that
design question. Whichever row lands the frame should close both.

## Closed (2026-10-31, PR 6138, riding with the frame row)

The start-tangent law's single singular point is gone with that law. A planar U-turn spine now sweeps at every station count. The klein loop sweeps `exact_spine` through `sweep_body`, and `review_half_turn_path…` builds.

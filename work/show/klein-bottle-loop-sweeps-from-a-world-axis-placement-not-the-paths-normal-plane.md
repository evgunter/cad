---
id: klein-bottle-loop-sweeps-from-a-world-axis-placement-not-the-paths-normal-plane
kind: issue
title: the klein scene's loop sweep starts from a world-axis placement, not the plane normal to its spine
status: open
opened: 2026-09-15
priority: P3
cost: E
rides_with: klein-scene-should-adopt-the-one-body-loop-sweep
---

## Finding

**Raised by SCALAR's `S393` fix pass** (2026-09-15), from the class the
unit folded elsewhere in the same tour.

`demos/tour/src/klein.rs`'s loop cell (~`:902`) sweeps the bottle's
annulus along an interpolated spine from a placement authored as the
WORLD axes at a point:

```rust
Affine3::from_parts(
    Mat3::from_cols(v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0)),
    v3(0.0, 0.0, ZTOP),
)
```

The local +Z of that frame is world +Z. The spine is a degree-3
interpolant through sampled points, so its start tangent is whatever
the interpolation produced — near an axis, in general not on one. The
section is therefore drawn in a plane slightly off the plane normal to
the path, and `sweep_places` then carries THAT plane along, so the
tilt is present at every station.

**The precedent is the `s_duct` disposition in PR #2466's fix pass.**
That scene had the same shape — `Affine3::identity()` on a spine whose
start tangent is `|n̂.z| = 0.99999911`, 1.3e-3 rad off +Z — and it was
folded onto `geom_core::linalg::frame::path_start_frame`, moving the
body (volume 1.570508274780932 -> 1.570508341333644) and the
tess-budget baseline with it. The same fold is what this row asks for,
with the same consequences to expect: a body that moves, a baseline
re-cut, and render lanes that re-baseline.

Two things to measure before folding, which `s_duct`'s fold measured
first: how far the spine's start tangent actually is from world +Z
here, and whether the ANNULUS's symmetry absorbs the resulting roll
(a circle does, so the rings may be unchanged and only the loft's
v-parameterization move — the `work/blend/` row on the first strip is
the mechanism).

**Where**: `demos/tour/src/klein.rs`, the `bottle` wall's
`sweep_body` call (~902) and the spine built just above it.

**Confidence**: sure (the placement reads as quoted; the spine is an
interpolant).

**Verdict:**

## Closed

Folded with its unit (`klein-scene-should-adopt-the-one-body-loop-sweep`),
which made the loop sweep the SCENE's build: `demos/tour/src/klein.rs`
`sweep_loop` draws the annulus in `path_start_frame(start, start
tangent, tol)`. Measured first, as asked: the interpolated spine's
start tangent is 3.99e-4 rad off +z, and its end tangent the same off
−z in mirror image. The door's +z reference rung decides, so the
frame's local x is world −y — a quarter-turn roll about the tangent,
which the round section absorbs.

The tilt itself is NOT cosmetic, and it is load-bearing twice:

- each cap lies in the plane normal to its end tangent, so it sits
  turned off the bulb's rim it meets, its edge up to 2·r·sin(tilt/2)
  ≈ 1.1e-4 m away — the scene asserts each cap's plane normal is its
  spine end's tangent;
- it is what lets the loop build at all. The spine turns a half turn,
  `sweep_places` carries every station from the base tangent, and the
  EXACT spine (end tangents exactly ±z) refuses `PathTangentReversal`
  (klein's wall 9). The interpolant's mirrored tilts put the last
  station on the C6 float knife edge instead, which builds. No public
  door pins the end tangents, so the tilt can neither be authored away
  nor relied on by design:
  `work/carve/a-half-turn-spine-sweeps-only-off-its-exact-tangents`.

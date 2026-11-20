---
id: zero-area-laminae-whose-traversals-differ-by-rounding-refuse-under-words-for-other-causes
kind: issue
title: mesh: a zero-area lamina whose two traversals differ by rounding refuses PinchWedge, Triangulation, UnresolvedWinding or TrimLoopOrientation, each naming a cause it does not have
status: open
opened: 2026-11-20
priority: P3
cost: M
---

Found by `tesslie/empty-patch` (PR 7196) and its review, 2026-11-20.

## Finding

A lamina is a face bounded by two edges along one locus, traversed there
and back. Tier 3 refuses such a body, and `tessellate` does not
re-validate it. When the two traversals' chart points are bitwise equal,
the triangulator merges them, every boundary side is crossed twice, and
both the planar and trimmed lanes refuse `BoundaryBoundsNoRegion`
(`region::require_a_region`). That refusal names the cause.

When the two traversals' points differ by rounding, the triangulator
shares no side between them. Parity then finds odd sides and passes, and
the face goes on to refuse under a word for another cause.

Measured on PR 7196's branch (default ε, `Tol::witness`; bodies built
through the Euler doors):

- **Re-framed carrier on a flat bilinear NURBS patch.** The second edge
  is the same circle stated with a rotated `u_ref` (rotations 0.1, 0.37
  and 1.3 rad), so its points differ by rounding (endpoint gaps 0 to
  1.1e-16).
  - rotation 0.1: δ = 0.1 → `PinchWedge`; δ = 0.01 → `Triangulation`.
  - rotation 0.37: δ = 0.1 → `PinchWedge`; δ = 0.01 → `UnresolvedWinding`.
  - rotation 1.3: δ = 0.1 and 0.01 → `Triangulation`.
- **The same re-framed pairs on a plane** (measured before PR 7196):
  `Triangulation` or `PinchWedge`. The exception is rotation 1.3 at
  δ = 0.1: there the points came out bitwise equal and the census
  panicked, which is now `BoundaryBoundsNoRegion`.
- **Same carrier, analytic chart.** Unit sphere, tilted circle
  `n ∝ (tx, 0.2, 1)` at height `h`, arc as listed, δ ∈ {0.1, 0.03, 0.01,
  0.003}:
  - tilt 0.3, h 0.5, arc (0.3, 2.0): `BoundaryBoundsNoRegion` at every δ.
  - tilt 0, h 0.5: the same.
  - tilt 0.45, h 0.6, arc (0.5, 1.0): the same.
  - tilt 0.7, h 0.2, arc (0.1, 1.3): `BoundaryBoundsNoRegion` at 0.1 and
    0.03; `TrimLoopOrientation` at 0.01 and 0.003.
  - tilt 1.1, h −0.3, arc (2.0, 4.0): `TrimLoopOrientation` at 0.03;
    `BoundaryBoundsNoRegion` elsewhere.

  The `TrimLoopOrientation` here comes from the sphere arm's sense check:
  the shoelace sum is rounding noise, and its sign contradicts the face's
  sense. It does not come from the `None` arm. I confirmed this with a
  trace.

The review of PR 7196 measured the same class. Its measurements, as
relayed (no exact tilts or offsets were given beyond these):

- **Probe C (analytic sphere, trimmed lane).** A lamina traced twice on
  the SAME carrier gets two pcurves whose UVs differ by rounding.
  - Tilted great circle and tilted small circle, identical carriers:
    `BoundaryBoundsNoRegion` at coarse δ, `TrimLoopOrientation` at
    δ ≤ 1e-2.
  - Re-framed carrier: `PinchWedge`, `Triangulation`,
    `TrimLoopOrientation`.

  The `TrimLoopOrientation` misses come from the sphere arm's sense check
  (above), not from the `None` arm.
- **Probe D (planar).** Re-framed laminae at δ 2e-2 … 2e-3 reproduce
  `PinchWedge`, `Triangulation` and `UnresolvedWinding`.
- **Outside the class, for the record.**
  - A NURBS hemisphere meridian lamina (a zero-width UV polygon) refuses
    `BoundaryBoundsNoRegion` at every δ. Its zero-width bounding box
    passed the lattice without a panic.
  - NURBS rim and latitude laminae refuse `UnsupportedCurve` upstream.
  - Analytic cylinder, sphere and cone laminae along iso circles route to
    the curved lane and refuse `MeridianFreeCurvedFace`.

## What closing it takes

A reading of "the two traversals are one locus" that does not rest on
bitwise equality of chart points. Two candidates:

- Compare the edges' carriers and intervals, the rung `two-coincident-edges-open-two-columns-that-are-one` names for the curved lane.
- Merge chart points within the run's band before the parity read.

Either one moves the decision upstream of the triangulator. Until then
these faces refuse typed, under words that name another cause; none of
them meshes as a hole.

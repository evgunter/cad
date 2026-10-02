---
id: a-torus-seam-graze-needs-the-rim-root-deflated
kind: issue
title: A G1 torus chain declared a Seam stops at the graze of an edge leaving the rim: a torus×torus seam certifies no side
status: open
opened: 2026-10-02
---


## What

The lily's stem chain (`crates/sweep/tests/mate7a_torus_rest.rs`,
`the_g1_tube_chain_declared_a_seam_stops_at_the_crossing_layer`): two
tube segments on rings 5 and 1.1, tube 0.06, meeting G1 along a shared
meridian circle. With the walls declared `Seam` and the junction discs
`Rest`, every seam verifies (`boolean::verify_seam_declaration`). Then
segment A ∪ segment B refuses `CurvedPierceUnsupported` on A's outer
equator circle (radius 5.06), against B's torus wall. That edge leaves
the rim, and it touches B's carrier there with a double root.

## Why the seam's cover does not reach it

C4's one-sided cover needs the parent carrier to lie in one closed side
of the target carrier. A torus × torus seam does not give that: past
the rim, the two tube centrelines diverge quadratically (≈ 0.355 s² at
arc length s), so each tube surface crosses the other's continuation
before s ≈ 0.58. So `boolean::seam_certifies_side` withholds the cover
for a torus with a cylinder or a torus. The crossing found there lies
outside B's face trim, but the cover argues from carriers, not trims.
Forcing the cover (a local experiment) still refuses, on the second
piece (`a-torus-meridian-lying-on-a-torus-is-unsettled`).

## What would build it

The circle × torus root lane (`boolean::circle_torus`) could take the
verified seam as the licence for the double root at the rim end. It
would deflate that root and certify the remaining roots as it does
any other's. That is an edge-local certificate, sound where the global
side is not.

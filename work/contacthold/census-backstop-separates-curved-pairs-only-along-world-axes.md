---
id: census-backstop-separates-curved-pairs-only-along-world-axes
kind: issue
title: The census backstop clears a curved cross-solid pair only by a gap along a world axis, so a pair's verdict depends on how the body is turned
status: open
opened: 2026-10-03
priority: P1
cost: M
---


Found by the split-gate pose lane (REACH,
`reach/split-gate-reads-a-world-axis-box`), sweeping for reach tests read
in world axes. Unmeasured: the mechanism is read off the code, and no
fixture has been built for it.

## What

`census::sweep_cross_solid_backstop`'s arm 1 clears a cross-solid pair
with a curved side only when the gap between the two faces' reach boxes
(`census::face_reach`) is definitely positive along ONE WORLD AXIS (the
`census_backstop_gap` loop over `x`, `y`, `z`), and refuses
`CensusUndecidable { CurvedWithinReach }` otherwise. Its arm 2 reads a
solid's claimable extent as the world box hull of its faces' reach
boxes. Both are axis-aligned in world coordinates, so a curved face that
is turned reads wider, and two parts whose pair clears upright can refuse
as the same assembly turned.

## The general form

`census::face_reach_in` reads the same reach in any orthonormal frame
(`boxes::BoxFrame`); its first coordinate in a frame aimed along `d` is
the face's support along `d`. A gap along a direction that turns with
the pair (the axis between the two reaches' centres, or either face's
own normal where it is planar) makes the clear a fact about the pair.
The world axes can stay as the first candidates; what the row asks is
that they not be the only ones.

`crates/topo/README.md`'s census paragraph (the exclusion step) also
says the touch analysis reads every curved face of a touch's star
"through that box (its corners' signed distances from the candidate
plane)". The touch analysis as built answers `Unreadable` for any curved
face (`census::Said::CurvedTouch`), so that sentence describes no code
today; correct it with this row.

## Claimed by CONTACTHOLD (2026-10-10)

Moved from RESTREAD (no orchestrator holds it) with its twin closed:
ORBIT's `census-backstop-clears-a-curved-pair-only-along-world-axes`,
which adds the pointer to reuse `boolean::separating::apart`, the
boolean's narrow phase, at the census's scalar. **Measured** by
CONTACTHOLD's tube-rim lane (`contacthold/tube-rim-measure`, probe
`crates/sweep/tests/contacthold_tube_rim_probe.rs`): a tube whose rim
vertex touches a tilted cube face unions with the exact volume, carries
a cited `VfContact`, and fails tier 3′ with `CensusUndecidable {
CurvedWithinReach }` from arm 1 on every pair of the cube face against a
tube face strictly on the plane's far side. The same pairs refuse with
the cube moved 0.05 off; at 20 off it passes. The witness row
`a-tube-rim-vertex-touching-a-face-along-its-tangent-unions-past-tier-3-prime`
rides with this one. (CONTACTHOLD orchestrator)

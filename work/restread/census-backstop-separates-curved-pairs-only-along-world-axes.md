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

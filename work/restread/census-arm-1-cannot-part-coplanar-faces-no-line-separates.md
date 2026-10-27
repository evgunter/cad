---
id: census-arm-1-cannot-part-coplanar-faces-no-line-separates
kind: issue
title: The census's arm 1 clears only on a separating direction, so a plate's top and a block's base in one plane, the block in the plate's reflex notch, refuse CurvedWithinReach though their regions are apart
status: open
opened: 2026-10-26
priority: P1
cost: H
refs: [census-cross-solid-curved-pairs-undecidable-on-shell-results]
---


Measured by RESTREAD on `restread/curved-pairs-apart`. In `sweep`
`join2_r1_probes::join2_r1_grid` (ignored, release), the `gridL` poses
stack a rectangle over `z ∈ [1, 2]` on the rounded L plate over
`z ∈ [0, 1]`. Take one whose rectangle sits in the L's reflex notch,
corner at the reflex fillet's centre, such as
`gridL r=0.5 x=[3.5,6] y=[3.5,6]`. The union is two solids, 0.5 apart in
plan, and fails tier 3′ on `CensusUndecidable` alone. The battery
prints no pairs. Read off the geometry, the pair no direction can part
is the L's top face against the rectangle's base: the top is planar,
but its boundary carries the fillet's arc, so it is arm 1's. Measure
the refusing pairs first.

## Why no direction clears it

The two faces lie in one plane, `z = 1`. A plane that separates them
would cut `z = 1` in a line that separates their regions. The L wraps
two sides of the rectangle, so no such line exists. The pair is apart,
but neither a box gap nor any turned direction can show it, and
neither can the depth gap: the rectangle's corner lies on the fillet's
axis.

## What would read it

Two coplanar regions are disjoint when their boundaries do not meet
and a point of each lies outside the other. That is a region reading:
- the edge reaches of each boundary against the other face;
- one `face_placement` per side.

It is not a direction, and it belongs beside `census::depth_gap` as a
fourth clear. Also measure how many of the grid's 250 remaining `gridL`
`BAD` lines are this class; the rest are touches.

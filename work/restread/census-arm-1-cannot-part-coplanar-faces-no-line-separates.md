---
id: census-arm-1-cannot-part-coplanar-faces-no-line-separates
kind: issue
title: The census's arm 1 clears only on a separating direction, so a plate's top and a block's base in one plane, the block in the plate's reflex notch, refuse CurvedWithinReach though their regions are apart
status: closed
opened: 2026-10-26
priority: P1
cost: H
refs: [census-cross-solid-curved-pairs-undecidable-on-shell-results]
closed: 2026-11-02
pr: 5918
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

## Measured

On `restread/arm1-coplanar-regions`, with every `CurvedWithinReach`
refusal and every tier 3′ finding of each grid line's result logged:
all 794 `BAD` lines fail tier 3′ on `CensusUndecidable` alone. Classed
by the rectangle's plan against the lower plate's, 750 touch it (544
`grid`, 206 `gridL`) and 44 are apart: the 22 `gridL` poses in the
notch, union in both orders. No line fails through a coplanar pair
alone: each notch pose refuses the base against the L's top and also
two of the block's walls against it, each standing on the top's plane
along its foot, which no line in that plane parts from the L either.
So the reading covers both: a planar face against the plane of
another, read where it meets that plane (`census::in_plane`). The grid
goes from 794 `BAD` to 750, the 44 apart lines `SOUND`, and no other
line changes.

## Closed

2026-11-02, PR 5918 (`restread/arm1-coplanar-regions`). Census arm 1
clears a planar × planar pair no line parts
(`census::in_plane::apart_in_plane`) by two readings: both faces on one
decided carrier, with every edge pair apart and one vertex per loop
placed `Out`; or one face standing on the other's plane at a slope
decided past sin θ = ½, with no vertex across it, its arcs definitely
off it, and its line edges with an end on it apart from the other
face's edges and those ends placed `Out`. JOIN-2's grid goes 794 → 750
`BAD`: the 44 apart L-notch lines read `SOUND`; the 750 left are
touches, held by D10.

---
id: equator-seam-reauthor-refuses-the-hollowed-elbow
kind: issue
title: The klein elbow's equator seams (RevolvedPoint-declared chart seams) refuse at reauthor once the spiric rims mint - the moved cap displaces the corner off the sketch plane
status: open
opened: 2026-09-14
refs: [c5-plane-torus-cone-cylinder-arms, spiric-carrier-ruling, 2566]
priority: P0
cost: H
---


## What

Measured by CURVED-SPIRIC PR-1a (PR #2566, the closing measurement):
with the rims minted as `Curve3::Spiric`, the elbow's hollow refuses at
`restate`'s `reauthor` on the EQUATOR seams — `TogetherAxialEdge {
edge: EdgeKey(3v1), what: "a revolved point's moved corner stands out
of the family's own sketch plane, so the same rotation does not pass
through it" }` (`offset_axial_reauthor_plane`). The disc profile's two
vertices are revolved as `RevolvedPoint`-declared chart seams; the
moved start cap displaces the seam's corner `t` off the sketch plane,
so the declared rotation no longer passes through it. The design doc's
authority argument (`Derived` rims, no `reauthor`) covered the rims and
not these seams. The sectioned vessel has no such seam and reaches
check 7's `VolumeUncomputable` as predicted.

## Fix shape (the lane's candidate, for the spec)

Re-author the `RevolvedPoint` placement onto the moved corner's own
azimuth and take the span from the moved ends — the seam is still a
circle about the axis; only its declared rotation origin moved. One
arm in `restate`/`reauthor`, a row per elbow suite flipping from this
door to the props door, the old door recorded. E–M / STRUCTURAL. Opens
after PR-1b (the pcurve variant) in the spiric lane; opening
measurement already taken on #2566's head.

## Home

CURVED — the spiric lane (`spiric-rim-carrier`).

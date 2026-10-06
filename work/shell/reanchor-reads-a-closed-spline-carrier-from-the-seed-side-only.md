---
id: reanchor-reads-a-closed-spline-carrier-from-the-seed-side-only
kind: issue
title: replace_face's spline re-anchor reads Newton's foot from the old parameter, so on a closed NURBS carrier an on-carrier target across the seam refuses with a misnamed refusal
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Filed by the `shell/lofted-wall-seam` lane (PR 4117) on its review's C1
NOTE. Measured by the review lane's probe (`p_foot_branch`, local, not
pushed) on a closed rational-quadratic circle as a NURBS carrier.

## Measured

`replace_face::plan_reanchors` reads a spline carrier's new parameter
as `NurbsLane::carrier_foot` — Newton from the endpoint's OLD
parameter, clamped to the knot domain. On a closed carrier:

| seed → target | Newton ends | refusal | truth |
|---|---|---|---|
| 0.99 → 0.02 (across the seam) | clamped at 1.0, gap 0.116 | `ReanchorPastCarrierEnd` | the target is ON the carrier |
| 0.6 → 0.1 | stagnates at the distance MAXIMUM, gap 2.0 | `ReanchorOffCarrier` | the target is ON the carrier |

Both refuse typed, so neither builds a wrong body. Both moves are far
larger than an offset makes, so no shell reaches them today. The
`|δ|` measurement at `plan_reanchors` records how far real offsets
move a spline end.

## Fix

A closed (or periodic) spline carrier needs the re-anchor to try the
seam's other side before refusing — the spline analogue of the circle
arm's anchored branch — or a refusal of its own that says the foot
was read from one side. `ReanchorOffCarrier`'s docs already say the
spline case means "off the carrier, or on a stretch the old parameter
does not lead to".

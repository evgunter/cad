---
id: a-rim-touching-split-escalates-on-the-side-of-plane-band
kind: issue
title: a tube split by a plane whose section touches a rim at azimuth 0.3 escalates on the side-of-plane band
status: open
opened: 2026-10-06
priority: P3
cost: E
---


## What

The tube revolved a full turn about `y` from `(0.5, 0)–(1, 0)–(1, 1)–(0.5, 1)`,
cut by the plane leaning `t = 0.2` whose outer ellipse touches a cap rim
at one point. This is the construction of
`split_across_a_revolve_seam::a_section_touching_a_rim_splits_at_the_closed_form`
(the plane through `(cos a, rim, sin a)` with normal
`ŷ cos t ∓ (cos a, 0, sin a) sin t`). At azimuth `a = 0.3` the split
escalates on the side-of-plane band, margin `5.9e-9`, on main and on
PR 4120's head alike. At the azimuths that row pins (0, π/2, π) it
splits at the closed form.

Unmeasured, a theory only: the touch point is a computed point on the
rim circle, at a rounding-scale distance from the plane, so whatever
the side reading reads there lands in the band. A rim touch is a
tangency of the section conic with the rim circle, and nothing declares
it.

## Where to look

The review reports a side-of-plane band escalation; the raising site
is not traced. Measure first: read the payload and the vertex
it names. The question is whether a rim tangency at an off-vertex
azimuth can be decided structurally, or whether it is a declared
contact the split has no channel for.

## Found by

The review of PR 4120.

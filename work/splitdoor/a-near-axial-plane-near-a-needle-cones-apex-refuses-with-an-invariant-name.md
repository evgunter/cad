---
id: a-near-axial-plane-near-a-needle-cones-apex-refuses-with-an-invariant-name
kind: issue
title: a needle cone cut by a plane off its apex near the seam refuses Join SectionInvariant or Join Euler, identically on main
status: open
opened: 2026-10-29
priority: P1
cost: M
---


## What

Needle cones of half-angle 1° and 3° (profile revolved about y, apex at
(0, 1, 0)), cut by a plane through the point 1000ε along the seam
generator from the apex, well off the apex. The plane's normal is tilted
from the axis by `tilt` about azimuth `spin`. The refusals:

- tilt 1.5, spin 0: `Join(SectionInvariant ..)`;
- tilt 1.2, spin 2.0, at 1°: `Join(Euler ..)`.

Both refusals carry an invariant's name, on ordinary geometry. Both are
identical on main and on PR 5956: the delta review's differential
`review_5956_delta_needle::delta_needle_cone_near_apex_reads_as_main`
(ignored; run it on both heads and diff its `NEEDLE` lines).

## Where to look

The join on a near-axial section of a needle cone near its apex: the
section conic is tiny and steep there. Read the payload and the raising
site before guessing at the cause.

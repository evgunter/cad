---
id: a-plane-touching-a-bore-rim-at-its-seam-refuses-a-stated-interval-zero-in-the-join
kind: issue
title: A plane touching a tube's bore rim at the seam azimuth refuses Join(Euler(Certification { StatedInterval Zero })) under the downward normal
status: dispatched
opened: 2026-10-28
priority: P1
cost: M
---


Split off `a-plane-touching-a-bore-rim-splits-into-a-pinched-side-under-one-normal-and-refuses-under-the-other`
at PR 5951's merge (2026-10-28, SPLITDOOR), which built every other pose of that row.

## What

The bore-rim pose of that row (`split_across_a_revolve_seam.rs`,
`a_plane_touching_a_bore_rim_splits_alike_under_either_normal`'s fixture) at the seam azimuth
`a = 0` under `s = −1` refuses `Join(Euler(Certification { StatedInterval { refused:
Forward(Decided(Zero(..))) } }))`, at ε 1e-9 and 1e-12, at every tilt t ∈ {0.05, 0.2, 0.5}
measured by PR 5951's review. At 1e-6, 17 poses near the seam escalate `SliverSector` instead
(typed). The other 95 azimuths and both normals elsewhere build at the closed form.

## Leads

PR 5951's review saw the same refusal on the hole-rim tangent poses under a mutant that read the
join's touch-point runs in insertion order, so it may be a crossing-run pairing case where the
seam's crossings join the touch run; ordering each run by the crossing before it (`touch_order`)
does not clear it. Unmeasured beyond that.

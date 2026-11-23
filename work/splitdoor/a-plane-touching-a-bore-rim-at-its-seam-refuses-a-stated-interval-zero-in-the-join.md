---
id: a-plane-touching-a-bore-rim-at-its-seam-refuses-a-stated-interval-zero-in-the-join
kind: issue
title: A plane touching a tube's bore rim at the seam azimuth refuses Join(Euler(Certification { StatedInterval Zero })) under the downward normal
status: closed
opened: 2026-10-28
priority: P1
cost: M
closed: 2026-10-29
pr: 6020
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

## Closed (2026-10-29, PR 6020)

The cause was not crossing-run pairing. At a = 0 the touch point is the bore rim's seam vertex, and the bore face holds two null-edge halves there. Under s = −1 the chord joined two distinct below-end copies at the same point, which `(u1 != u2).then_some(p2)` read as a zero span where a whole turn was meant.

`chord_join::same_site` is now the join's one site reading: the same vertex, or a copy tied by null edges. A chord whose two ends are one site is a whole turn, in both the split's `chord_spec` and the Boolean's `bool_planar_chord_spec`; `closed_at` reads through it. The carried lane drops its identity test and refuses off the seat. The seam poses build at the closed form and tiers 1/3/3′ at three ε, and so does the counterbore floor rim's seam.

FULL review: APPROVE. It ran 192 split probes plus Boolean probes; every moved verdict went from a refusal to the correct build, checked by independent quadrature to ≤1e-7. One MINOR, in the carried lane, was fixed.

Filed:
- `a-plane-touching-a-counterbores-floor-rim-ships-sides-tier-three-prime-refuses` (P1);
- `a-boolean-touching-a-bore-rim-at-its-seam-refuses-a-loose-scaffolding-pair` (P2).

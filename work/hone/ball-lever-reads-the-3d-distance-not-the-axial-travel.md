---
id: ball-lever-reads-the-3d-distance-not-the-axial-travel
kind: issue
title: a section row's ball lever is the 3-D distance from the pivot, not the consumed region's axial travel
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

`Reach::lever_from` (`crates/geom-brep/src/extent.rs`) levers a
`Reach::Ball` at `|pivot − centre| + radius`, the farthest a consumed
point stands from the foot. A tilt moves a section's served object by
its sine times the consumed region's AXIAL travel from the foot, and a
ball's centre stands about a radius off the axis, so on the
plane×cylinder and cylinder-pair rows the lever reads `r + ρ` where
the travel is `ρ`. Measured in PR 4280's differential
(`crates/geom-brep/tests/one_sum_differential.rs`): every escalation
of a pose main served correctly on those rows is explained by the
lever's excess over the travel (the reading at the row's own lever
exceeds the band in all of them). The lever is main's own measure and
is conservative on the zero side; on a two-sided row whose definite
side is served it is the over-stated extent `extent.rs`'s module docs
warn about.

## The shape of a fix

Lever a ball at its axial reach from the pivot (`Reach::range_along`
along the axis), keeping the 3-D distance only where a row's
displacement really is radial.

Moved from LEVER to HONE on 2026-11-20, by `git mv`, when LEVER merged into HONE: LEVER held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one HONE carries.

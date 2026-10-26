---
id: an-oblique-caps-inside-reading-charges-a-ring-its-major-semi-axis
kind: issue
title: blend: at an oblique cap the sliver's inside reading charges an elliptic cycle its major semi-axis across the section's minor, so a cycle up to a·(√(1+s²)−1) inside the arc reads a bound
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## Finding

`SectionFrame::outside` (`crates/sweep/src/blend/open/end_face.rs`),
which `CapSliver::ring_inside` reads for each piece of a cycle on a
round end, bounds an ellipse edge's scaled radius by
`scaled(c − center) − major/minor`: the cycle's major semi-axis over
the section's MINOR one, in both of the section's scaled axes. At a cap
leaned `s` along the spine both the section and a bore's or bar's cycle
are ellipses stretched by `√(1 + s²)` along the same axis, so in the
scaled frame the cycle is a circle of radius `a/r` and the charge is
`a·√(1 + s²)/r`: a cycle up to `a·(√(1 + s²) − 1)` inside the arc reads
as a bound (`1.8e-4` at `s = 0.3`, `1.7e-3` at `s = 1`, for `a = 0.004`).

Measured, both material sides alike: a round feature of radius `0.004`
on the bisector `1e-4` inside the fillet's arc is consumed at `s = 0`
and refuses `FaceClearance { bounded: true }` at `s = 0.3, 1` (convex
bore on the leaning prism, concave bar in the leaning pocket);
`3e-4` inside is consumed at `0.3` and refused at `1`; `3e-3` is
consumed at both. Pinned by
`band_concave_bar::a_bar_a_hair_across_the_fills_boundary_crosses_and_one_inside_is_absorbed`
(the fillet's hair inside at a lean reads a bound).

Conservative (refuse, never a silent pass).

## Close

Read the cycle's extent in the section's scaled frame per axis (its
semi-axes along `u` and `w` over `major` and `minor`), or the cycle's
own axes against the section's; flip the pinned rows to absorptions at
the closed form.

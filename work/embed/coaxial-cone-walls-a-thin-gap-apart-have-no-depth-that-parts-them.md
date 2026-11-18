---
id: coaxial-cone-walls-a-thin-gap-apart-have-no-depth-that-parts-them
kind: issue
title: Two coaxial cone walls a thin gap apart propose an axis with no level, so a conical wall's faces are parted by supports alone, in about 3,100 splits
status: open
opened: 2026-11-18
priority: P2
cost: M
---


Found by lane `embed/nested-tori` (2026-11-18), measuring
`nested-tori-a-thin-gap-apart-spend-the-pair-budget-on-box-distance`.

## Evidence

`topo::clearance` asks the census's depth gap
(`census::depth_gap::gaps_in_depth`) of a face pair's root
(`apart_in_depth`). A gap needs a site about which one face's
greatest depth lies below the other's least. A cone face proposes its
axis line (`Cover::about`), but a cone has no level about its axis:
both walls of a conical wall span the same range of distances from
it, so no depth parts them.

`self_intersection_proves_injectivity::a_conical_and_a_toroidal_wall_hold_within_the_default_budget`,
at f64 and Interval, default ε, 1e-6 and 1e-12:

| wall | splits before | splits after |
|---|---:|---:|
| conical, `0.05` thick, 30° off the axis | 3,099 | 3,099 |
| toroidal, `0.05` thick | 6,647 | 200 |

The conical wall's two faces are still parted by the supports along a
direction, cell by cell. It holds, in about 5% of the default budget;
a longer or thinner conical wall costs more, as the nested tori did.

## What would part them

A cone's own residual (`geom_brep::cone_offset`, `ρ·cos α − s·sin α`)
is the distance from its nappe wherever the foot lies on the nappe, so
a coaxial cone of the same half-angle lies at one level of it. That is
a depth about a cone, a site `geom::cover::Around` does not have. The
reading belongs in the cover (`[one-way.places]`), with the census's
depth gap as its first reader.

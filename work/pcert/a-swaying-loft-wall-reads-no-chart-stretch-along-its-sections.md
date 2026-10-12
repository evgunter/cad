---
id: a-swaying-loft-wall-reads-no-chart-stretch-along-its-sections
kind: issue
title: chart_stretch_inf reads a swaying loft wall's v-derivative net along one global direction and returns inf_v = 0, so chart_region's certified arms refuse ArmUnbounded
status: open
opened: 2026-10-10
priority: P3
cost: M
---


## What

`geom_brep::pcurve_cache::chart_stretch_inf` (`nurbs_stretch_inf`,
`net_inf`) bounds each chart speed from below by projecting the
surface's WHOLE derivative net onto one direction, the net's own sum.
On a loft wall whose sections sway sideways, the `v`-derivative net
points back along that direction somewhere, so the bound is 0 although
`|S_v|` never drops near zero. `topo::chart_region`'s `certified_arms`
gates the inf positive first, so every such wall refuses
`ChartRegionError::ArmUnbounded { chart: "NURBS" }`.

Measured with `chart_region_overlap(body, f, body, f, band)` on each
NURBS wall of the loft through four 2 × 2 squares at z = 0, 1, 2, 3,
x offsets `−a, a, −a, a` (`swaying` in
`crates/sweep/tests/a_swaying_loft_corner_meters.rs`):

| loft | `inf_u` | `inf_v` | overlap |
|---|---|---|---|
| `loft_prism` | 1.985–2 | 2 | Ok |
| v-degree 2, a = 0, 0.25 | 2 | 3, 1.565 | Ok |
| v-degree 2, a = 0.5 (builds on main) | 2 | 0 | ArmUnbounded |
| v-degree 2, a = 0.75, 1, 2 | 2 | 0 | ArmUnbounded |
| v-degree 3, a = 0.5 | 2 | 0 | ArmUnbounded |

The `a ≥ 0.75` and v-degree-3 lofts build from
`a-swaying-loft-corner-refuses-as-a-vanishing-span`'s close (PR 4540)
onward; `a = 0.5` at v-degree 2 built before it, so the refusal is not
that PR's.

It is the class the curve meter had: `NurbsCurve3::speed_lower_bound`'s
global chord collapsed on the swaying corner, and the per-span and
piece assemblies (PR 4540) answer there. The same reading per cell, or
per piece of a cell, is the likely shape here. Owner context:
`work/chart/certified-arms-are-an-untagged-inf-rate-beside-a-typed-pair.md`
(the arms' typing).

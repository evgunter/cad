---
id: chart-arm-tables-beside-the-one-sup-door
kind: issue
title: Per-kind chart arm tables written beside chart_stretch_sup and chart_sup_arms_at, and ChartStretchInf mints then strips its sups
status: open
opened: 2026-10-24
priority: P3
cost: E
---


## Finding

CHART's arms-typing PR (5655) gave the chart's per-axis sup arms one
home: `geom_brep::chart_stretch_sup` over the whole chart, its azimuth
column read from `azimuth_lever`, and the window door
`chart_sup_arms_at` (with `chart_sup_arms_over` for a caller whose `|v|`
sup is not one window's reach). Its STYLE review named more members of
the class, outside that PR's fence:

- **`ChartStretchInf` is bare `T`** (`pcurve_cache.rs`,
  `ChartStretchInf` and `nurbs_stretch_inf`). `nurbs_stretch_inf` reads
  `nurbs_stretch_bounds`' `SupSpeed` pair and `.get()`s it at once to
  fill `sup_u`/`sup_v`, so the sup half is minted and stripped. It stays
  bare for a stated reason: its one consumer, `topo::chart_region`'s
  `certified_arms`, gates the per-axis infs as lengths and divides sups
  by infs in its skew assembly, so tagged fields would only be `.get()`
  again there; and `area_inf` is an AREA rate the linear pair does not
  name. That reason belongs in `ChartStretchInf`'s doc, which says
  nothing about it today.
- **`Placement::over`'s `(tube, latitude)` match** (`pcurve_cache.rs`,
  `Placement::over`, the `Levers` it builds): a copy of the v column of
  `chart_stretch_sup` (torus `minor_radius`, sphere `radius`, `1`
  otherwise), spelled by kind beside the door.
- **`topo::pcurves`' `polar_arm` and `v_meter`**: the same v column,
  third spelling (sphere `r`, torus `r_minor`, else
  `chart_stretch_sup_v`). That is CHARTTAIL's ground; it is listed here
  so the table has one row.
- **Candidates the review listed that may not be members**, to be
  checked rather than assumed:
  `pcurve_cache/envelope_lemma_fuzz/projected.rs` (a torus latitude
  circle's radius `R + r·cos v` in a fuzz generator — geometry, not a
  metering arm), and `sweep/src/revolve/tube.rs`'s `arm = R + r` (D4's
  angle lever for a dimensionless decision, CARVE's ground).

## Shape

Each copy reads the door it re-spells, or says in one line why it
cannot. `ChartStretchInf` keeps bare fields and gains the sentence
above.

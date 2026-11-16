---
id: the-spline-incidence-read-dives-where-its-floors-already-decide
kind: issue
title: The spline incidence read dives where its floors already decide, and stops at its cap on a thin sheet
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Filed by `pcert/spline-chart-incidence` (PR 6498, review fix Y5). The spline
chart's incidence test (`geom-brep/src/pcurve_cache/spline_incidence.rs`,
`Net::read`) is a best-first walk of a chart's Bézier cells: a floor (a lower
bound) clears a cell, and the chart's point at a cell's centre (an upper
bound) shows the sample on. Three costs remain.

## What

- **A band-scale sample still dives.** A sample in band of the chart cannot be
  shown off. It is let go once every kept cell's floor is past the zero band
  (the S9 prune, taken in the PR: such a cell holds no point within the zero
  band). Before that, though, the walk has dived about as deep as an on-chart
  sample would. Measured at opt-level 1 on
  `topo::all an_off_iso_straight_edge_takes_its_general_row::an_in_band_seam_takes_the_general_image_at_interval`:
  0.07 s on main, 0.75 s before the prune, 0.55 s after (7.8×). The review
  also measured `each_edge_takes_its_own_image_at_interval` at 1.9× (0.075 s at
  the PR's head).
- **A definite miss on a high-degree, wide-weight patch walks to the visit cap
  at `f64`.** Review measured door reads of 1.4–2.6 s that end `Escalated`
  where the same read at `Interval` ends `Off`: a scalar-split ending, and the
  charge's doing. The PR then replaced the per-level charge (global maxima over
  the least weight) with a per-point bound tracked through every blend, which
  moves 2/3 of the boundary-row family's definite misses from the lever to
  `Off` (1002 → about 2960 of 3000 at band 1e-12). These reads were not
  re-measured after that change.
- **A thin sheet stops at the cap unsettled.** Two equal control rows of
  unequal weights make a sheet a band thin over a width: a sample on it keeps
  about 300 cells per level within the band, none with a floor past zero, and
  4096 visits run out before a centre point comes within the zero band
  (`spline_incidence::rows`'s on-chart row leaves that mode out for this). The
  read ends not shown off, never as a false `Off`.

## Fix

Order the walk by an upper bound that falls with the sample's distance, such as
the chart's point at the cell's centre, which the walk already evaluates on a
visit, rather than the box's gap. Or stop the dive once the kept cells' floors
and points bracket the sample within the band. Re-measure all three rows.

## Re-measured after `pcert/spline-certificate-door`

That unit gave the read a foot: before the walk, each sample's foot on the
chart by Gauss–Newton on an `f64` shadow, from the last sample's foot, its
point evaluated in the scalar and decided as a cell centre is. An on-chart
sample reads on there; one in band arms the S9 prune from the walk's first
visit. At opt-level 1, `--test-threads 1`, the least of two runs:

- `an_in_band_seam_takes_the_general_image_at_interval`: 0.354 s on main
  (`f6e97bd62`, this row's 7.8×) to 0.053 s;
- `each_edge_takes_its_own_image_at_interval`: 0.035 s to 0.023 s.

The first bullet is so far cut below its pre-6498 cost. The thin sheet's
on-chart samples now read on at their foot, so the third bullet's cap is met
only where the foot misses; not re-measured on the sheet itself. The second
bullet (a definite miss on a high-degree, wide-weight patch walking to the
visit cap at `f64`) is untouched: the foot reads only an upper bound. Open
for it.

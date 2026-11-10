---
id: the-projected-frame-box-reads-an-uncharged-f64-restriction
kind: issue
title: The projected image's frame box decides a sector and a period gate on an uncharged f64 restriction
status: open
opened: 2026-11-04
priority: P2
cost: M
---



Filed by `splice/spline-region-charge`, from its sweep for the class "an f64 cut of a control net (knot insertion, de Casteljau, a window restriction, out of homogeneous form) whose box, hull or offsets decide, with no charge for the cut's rounding". That PR charges the spline face placement's two instances (`topo/src/spline_region.rs`: the carrier floor and the trim's pieces), where a point on a sheet `1e5` out read off its carrier and a query on a trim curve was given a side at the 1e-12 row. **This row was read, not reproduced.** Confidence: medium-low that the `f64` slack matters. At rest the sector is
read again on an `Interval`-lifted net (`pcurve_cache.rs` `PieceHull`, ~3222),
and the period gate is not.

## What

`geom-brep/src/pcurve_cache/projected.rs` `ProjectedImage::frame_box_on` (~line
640) boxes `piece_controls` at the caller's scalar: at `f64`, a plain
de Casteljau restriction, uncharged. Its readers decide:

- `piece_sector` (~1018): `positive("pcurve_projected_sector", ..)`, whether a
  piece is clear of a sector or an axis, read in derivation;
- `chart_box` / `sweep_box` (~826, ~841): check 2's period gate
  (`not_over`, ~1256) and the arms through `chart_sup_arms_at`.

## Fix

Charge the box for the restriction's rounding at `f64` (the per-point bound
`spline_incidence` carries), or read it on the `Interval`-lifted net as the
at-rest sector does. Reproduce first. A row at the 1e-12 band: a piece whose
box passes a sector's edge by an ulp of a far placement.

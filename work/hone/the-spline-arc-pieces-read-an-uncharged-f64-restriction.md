---
id: the-spline-arc-pieces-read-an-uncharged-f64-restriction
kind: issue
title: A planar loop's spline arc pieces decide clear and crossings on an uncharged f64 restriction
status: open
opened: 2026-11-04
priority: P2
cost: M
---



Filed by `splice/spline-region-charge`, from its sweep for the class "an f64 cut of a control net (knot insertion, de Casteljau, a window restriction, out of homogeneous form) whose box, hull or offsets decide, with no charge for the cut's rounding". That PR charges the spline face placement's two instances (`topo/src/spline_region.rs`: the carrier floor and the trim's pieces), where a point on a sheet `1e5` out read off its carrier and a query on a trim curve was given a side at the 1e-12 row. **This row was read, not reproduced.** Confidence: high that it is uncharged at `f64`. Not reproduced, so filed at P2, not P1.

## What

`topo/src/splitting/spline_arc.rs`: `SplineArc::net` (~line 94) reads a piece's
Bézier net through `geom_brep::pcurve_cache::projected::piece_controls`, which
at `f64` is the plain de Casteljau of `SpanLocate::restrict_window`
(`geom-core/src/spline/locate.rs`), then out of homogeneous form. Two decisions
read that net uncharged:

- `SplineArc::ball` (~129): the box centre and the farthest control point's
  reach feed `splitting/pieced_arc.rs`'s `decide(rows.clear, |q − c| − reach)`
  (~117), whose `Positive` settles the piece as not touching `q`: an Off.
- `SplineArc::cross` (~153): the control offsets `s_k` from the ray line feed
  the crossing count (no crossing, or exactly one).

At `Interval` the restriction is certified, so the defect is the `f64` lane's.
At a placement or a size of `1e4` and more, ulps of the coordinates reach the
1e-12 band.

## Fix

Charge the piece's net for its restriction's rounding, as the spline face
placement now does, or read the net at an enclosure scalar. A row: a point on
a far spline edge of a planar loop, at the 1e-12 band, never read clear.

**Tried once, at band `(1e-12, 1e-11)`, no false clear** (`splice/spline-region-charge`'s
fix pass). There were 2000 points exactly on far straight rational arcs:
- degree 2 to 4, with up to three interior knots;
- `1e3` to `1e6` long, placed up to `8e4` out;
- weights up to eight decades apart.

Each point was read through `PiecedArc::contact` with the spline rows, and none read `Off`. The ball's reach is
the farthest control point's distance from the box centre, so a point inside a piece has slack
far beyond an ulp. The exposure left is a point at a piece's own end point, where two pieces'
separate restrictions round apart, and the crossing count. Hence P2.

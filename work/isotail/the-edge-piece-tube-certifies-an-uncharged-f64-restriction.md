---
id: the-edge-piece-tube-certifies-an-uncharged-f64-restriction
kind: issue
title: The rung-3 edge piece's tube certifies an f64 restriction of the piece, not the carrier
status: open
opened: 2026-11-04
priority: P2
cost: M
---



Filed by `splice/spline-region-charge`, from its sweep for the class "an f64 cut of a control net (knot insertion, de Casteljau, a window restriction, out of homogeneous form) whose box, hull or offsets decide, with no charge for the cut's rounding". That PR charges the spline face placement's two instances (`topo/src/spline_region.rs`: the carrier floor and the trim's pieces), where a point on a sheet `1e5` out read off its carrier and a query on a trim curve was given a side at the 1e-12 row. **This row was read, not reproduced.** Confidence: medium. The `f64` entry was seen in tests and through generic
signatures, and no production caller at `f64` was traced.

## What

`geom-brep/src/edge_nurbs.rs` `edge_piece` (~line 2248) is
`piece_controls` at the caller's scalar. `analytic_rung3` (~1831) passes it to
`ssi::certify::certify_branch` (`Limbs::Tube`). The tube certifies the piece
it is given, lifting each control point to a point interval
(`refined_net`). At `f64` that piece is the restriction **rounded**, so the
uniqueness tube's verdict (transversality, the one-arc claim) is about a curve
up to an ulp of the coordinates away from the carrier. The doc claims the
enclosure only "at an enclosure scalar", and `f64` implements
`CertifiedEnclosure`.

## Fix

Either refuse `analytic_rung3` at an inexact scalar, or widen the lifted
net by the restriction's rounding (a per-point bound, as
`topo/src/spline_region.rs`'s `point_rounding` carries). First trace
whether any production path reaches it at `f64`.

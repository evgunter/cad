---
id: non-separable-rational-interior-column
kind: issue
title: An interior column of a chart whose weight net varies along both parameters has no exact class - the composite bound without a tube
status: open
opened: 2026-09-04
---


## The residue

TRIM-1's extractor (`crates/geom-brep/src/nurbs_iso.rs`,
`interior_iso_u`) collapses a described chart's net at an interior
`u*` into a curve in the chart's own `v` space. The seam class's
control-difference hull then needs the collapsed row's weights
`W_j = Σᵢ Nᵢ(u*)·wᵢⱼ` to be STRUCTURE a stored carrier can share
bitwise (C6: weights are `f64` forever). That holds exactly when the
weight net factors in a way an exact `f64` test can see — constant
along `u` (row 0's weights are `W`) or constant along `v` (`W` is
constant in `j` and cancels; the row is polynomial). Both cover every
wall this kernel builds and every imported cylinder wall.

A net varying along BOTH parameters has a computed `W`, so no carrier
shares its rational space and the seam class's hypothesis genuinely
fails. The extractor refuses `IsoRowError::WeightsNotSeparable`; the
certifier turns it into `PcurveCertifyError::IsoUnsupported` naming
the weight net (`pcurve_cache.rs`, the seam class's interior route;
pinned by `geom-brep/tests/interior_iso_column.rs::a2b_*`).

## The honest route, not built

A composite bound: `EnvelopeStatement::MapResidualComposite` through
`geom_core::spline::compose::tensor::surface_curve_residual` (today
consumed only by `ssi/certify.rs`), stated WITHOUT a uniqueness tube —
a `Chart` description names one surface, so there is no operand pair
for a tube to be about. That is a new certificate statement, not a
widening of the iso hull, and it needs its own spec. No construction
in the tree mints such a column today.

## Home

TRIM's: `pcurve_cache.rs` and `nurbs_iso.rs` are in this program's
territory. Not scheduled.

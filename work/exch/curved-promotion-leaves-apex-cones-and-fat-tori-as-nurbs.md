---
id: curved-promotion-leaves-apex-cones-and-fat-tori-as-nurbs
kind: issue
title: Curved surface promotion leaves a cone patch that reaches its apex, and a fat torus patch whose samples see its inner equator, as NURBS
status: open
opened: 2026-11-20
priority: P3
cost: M
---

Found while landing whole-patch promotion to cone, sphere and torus
(`exch/promote-torus-sphere-cone`). Both are incompleteness, never a wrong
promotion: the patch stays NURBS and imports as it did before.

## A cone patch reaching its apex

`geom_brep::patch_distance_sup` (`crates/geom-brep/src/patch_carrier.rs`)
converts the cone's `|f|` to metres through `Meters::certified`
(`pcurve_cache/projected.rs`), whose lever is `ρ cos α + z sin α` over the
cell's control box. On a cell holding the apex the lever's lower bound is
zero, so the cell reads the apex root `√|f|`. On exact geometry `|f|` is
rounding-width, about `1e-16·L²`, so the root is about `1e-8·L`: a
metre-scale cone that runs to its apex (a cone cap, a countersink to its
point) does not certify at ε_in = 1e-9 or finer. Refining the apex cell (a
sub-cell whose box keeps off the apex reads the lever) or a bound in which
`|f|/|Q|` stays finite at the apex would close it.

## A torus with `R < 2r` sampled near its inner equator

`recognize.rs` `estimate_torus` takes the larger-magnitude principal
curvature at each diagonal sample as the meridian's (`1/r`). On a ring
torus with `R < 2r` the parallel's curvature, `cos v/(R + r cos v)`,
outgrows `1/r` where `cos v < −R/(2r)`, near the inner equator; a sample
there proposes a wrong core circle and the patch stays NURBS. Choosing,
per sample, the curvature whose radius agrees across the samples (a fixed
enumeration of the eight assignments) would close it.

## The spindle sheet

A torus patch with `R ≤ r` (`Surface::Spindle`) is not proposed at all:
`estimate_torus` returns no candidate. The conversion exists
(`Meters::Spindle`), so promotion to a spindle sheet is the estimator and
its tests.

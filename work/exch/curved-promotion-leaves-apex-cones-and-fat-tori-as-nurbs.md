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
converts the cone's `|f|` to metres through `Meters::part_bound`
(`pcurve_cache/projected.rs`) read against the DOUBLE cone: the lesser of
the generator-lever reading (`|f|` over `ρ cos α + z sin α`) and the apex
root `√|f|`. On the cell holding the apex the lever's lower bound is zero,
so that cell reads the root alone. On exact geometry `|f|` there is
rounding-width, about `1e-16·L²`, so the root is about `1e-8·L`: a
unit-scale cone that runs to its apex promotes at ε_in = 1e-6
(`recognize.rs` P14) and stays NURBS at 1e-9 and finer. Refining the apex
cell (sub-cells whose box keeps off the apex read the lever, and the root
over a shrinking cell shrinks with it), or a bound in which `|f|/|Q|`
stays finite at the apex, would close the fine rows.

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

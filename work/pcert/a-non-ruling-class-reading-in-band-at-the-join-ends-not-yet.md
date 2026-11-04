---
id: a-non-ruling-class-reading-in-band-at-the-join-ends-not-yet
kind: issue
title: The join's in-band class reading on a sphere, cone or torus ends 'no way through yet', not on its lever
status: open
opened: 2026-11-04
priority: P2
cost: M
---


## What

`crates/geom-brep/src/pcurve_cache.rs`, `chart_iso_family`: a class
reading in band comes back `IsoFamilyRefusal::Undecided` only when its
predicate is `pcurve_chart_radial_moving` (the cylinder's ruling). Any
other class gate in band (the sphere's, cone's or torus's axial,
centred or meridian gate) falls to the last arm and comes back
`IsoFamilyRefusal::Image(harmonic_refusal(..))`, a lane choice's "not
yet".

The join consumes the reading as a structural act (D3
`[verdicts.consumers]`: "in band refuses with the decision's own
lever"), and `crates/topo/src/boolean/edge_join.rs` renders the
`Image` arm by its own ending. Measured (pcert E3, a sphere cap whose
rim's second arc is tilted 3ε off the parallel, the
`class_zero_rows` fixture at `tilt = 3ε`), both the boolean and the
join door end at a build with:

> two edges meeting at a vertex on one surface read their family off a
> chart image that did not derive: pcurve certification: ChartClass at
> sample 0 escalated: margin 2.598e-9 lies inside the ambiguity band
> (1e-9, 1e-8). There is no way through yet

The pose has a lever (move the arc onto the parallel or clearly off
it); "no way through yet" is the wrong ending for a structural act.
pcert E3 did not edit `chart_iso_family` (the one-ruling-reading lane
was editing its arms).

## Done when

Every class gate's in-band reading reaches the join as `Undecided`, and
the join ends it on its lever.

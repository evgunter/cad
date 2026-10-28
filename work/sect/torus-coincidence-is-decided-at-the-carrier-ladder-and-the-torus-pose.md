---
id: torus-coincidence-is-decided-at-the-carrier-ladder-and-the-torus-pose
kind: issue
title: Two tori decided one surface are spelled twice: the carrier ladder's torus rows and the torus pose's tt_coincident
status: open
opened: 2026-10-28
---


Found by `sect-toruspose` (branch `sect/torus-pose`) sweeping for every
site that decides a torus pair's tangency or coincidence.

## Measured

`crates/topo/src/boolean/carrier_eq.rs` `torus_data` decides two torus
carriers one surface on four rows (`carrier_torus_axis_parallel`
levered, `carrier_torus_center`, `carrier_torus_major_radius`,
`carrier_torus_minor_radius`), each on its own, then the sum where all
are in band (`declared_reading`). `crates/geom-brep/src/pair_section/torus.rs`
`TorusTorusPose::decide` decides the same question for a coaxial pose on
one row, `tt_coincident` (`|R − R₂| + |r − r₂| + |z₂|` read across the
coaxial slack), and refuses `CoincidentSurfaces` before the gap. Two
spellings, over different extents (the ladder's consumed extent, the
smaller torus's ball), so a pair in band of one can read definite at the
other.

## What a fix has to supply

One decision of "two tori are one surface", read by both the glue door
and the section value, or an argument that the two questions differ.

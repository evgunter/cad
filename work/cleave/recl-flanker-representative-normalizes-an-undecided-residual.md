---
id: recl-flanker-representative-normalizes-an-undecided-residual
kind: issue
title: recl's flanker representative normalizes an undecided residual against the common line
status: open
opened: 2026-10-01
---


## What

`crates/topo/src/boolean/recl.rs` `resolve_edge_edge` builds each
flanker's representative as `((v - axis * v.dot(axis)).normalize(),
reach)` — a hand Gram–Schmidt residual against the common line, with
its length neither decided nor refused. `axis` itself is
`a_sectors[fa_s].start.normalize()`, also undecided.

The doc calls the representative the flanking sector's NONCOPLANAR
bound, which would make the residual nonzero if that bound were
decided off the line somewhere upstream; this filing did not find
where that is decided, and did not build a fixture for it. A bound
within the band of the common line normalizes to a direction made of
rounding, and the membership test then reads a definite side off it.

## Shape

`geom_core::OrthoFrame::from_aim_and_reference` (or `UnitVec3::new`
on the residual, under a `bool_*` K name) decides the length and
refuses typed. Measure first: whether a body reaches a bound inside
the band of the line through the public boolean doors.

Found by the `linalg/decided-not-minted` sweep for the hand
Gram–Schmidt shape.

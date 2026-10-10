---
id: sketch-segment-restrict-re-derives-endpoints-per-split
kind: issue
title: SketchSegment::restrict re-derives its endpoints through eval at every split, so a nested split's stored arc endpoints compound the rotation's enclosure — the sibling of MappedCurve's per-split composition
status: open
opened: 2026-10-09
refs: [mapped-curve-restrict-composes-placements-per-split]
priority: P2
cost: M
---


`SketchSegment::restrict` (`crates/geom-brep/src/mapped.rs`) stores
the sub-segment's endpoints as `a: self.eval(s0)`, `b: self.eval(s1)`,
so every split re-derives the stored start point through
`Arc2::point_from` (an arc) or `lerp` (a line) and the next split
evaluates from that already-rounded point. Its own doc says so ("successive
splits compound it"). At `T = Interval` the stored width therefore grows
with the split count at the coordinates' scale — the shape
`MappedCurve::restrict` had for `RevolvedPoint` / `ExtrudedPoint` until
`nurbs/restrict-in-the-parameter` moved those onto a `SweepRange`
(restrict in the parameter, keep the placement and the authoritative
point).

Measured (local probe, not committed; widest of `eval` at `s = 0, ½, 1`
on a half-turn unit arc `a = c + (1, 0)`, `sweep = π`, nested
`restrict` 64 times):

| centre | split | 0 | 1 | 8 | 64 |
|---|---|---|---|---|---|
| (0, 0) | (0, ½) | 2.89e-15 | 3.55e-15 | 2.78e-15 | 2.14e-14 |
| (0, 0) | (0.3, 0.7) | 2.89e-15 | 5.88e-15 | 4.88e-15 | 2.29e-14 |
| (1000, −700) | (0, ½) | 2.27e-13 | 7.96e-13 | 2.16e-12 | 1.48e-11 |
| (1000, −700) | (0.3, 0.7) | 2.27e-13 | 7.96e-13 | 2.50e-12 | 1.51e-11 |

At the far centre the growth is ~2.3e-13 per split — two ulps of the
coordinates each time — even for `(0, ½)`, where the parameter
composition is exact and nothing but the re-derived endpoint moves.

The parameter form is the candidate fix: an arc keeps its authoritative
`a` and carrier and carries the sub-range of its sweep (`Arc2`'s sweep
plus a start offset, or a `SweepRange` of angles about the centre), so
`eval` turns the ORIGINAL `a` once for any split count; a line keeps
`a`, `b` and a parameter window. The design question is that
`SketchSegment::Arc` is the profile's canonical segment form (`a`, `b`
verbatim plus `Arc2`, shared with the `profile` crate and read by
`sweep::skin::segment_curve`, certification and the topo description
readers), so a window on it changes what "the segment's endpoints" means
for every reader of `a`/`b`; the readers are listed in the type's doc
(`SketchSegment`'s "every reader of the locus reads the same fields").

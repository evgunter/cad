---
id: an-anchored-cone-section-encloses-wider-at-interval
kind: issue
title: An anchored cone section's enclosures are a third wider at Interval than the centre-born one's
status: open
opened: 2026-11-20
priority: P2
cost: M
refs: [the-section-table-admits-a-curve-on-its-resolution]
---


## Finding

`plane_cone_section` mints its ellipse anchored at its near vertex
(`crates/geom-brep/src/intersect.rs:2589`). On a section whose centre is
near, at `Interval`, what is read off that ellipse encloses about a third
wider than it did born with its centre. Measured on the cone ring's two
volume rows (`crates/sweep/tests/a_ring_on_a_cone_face.rs`, both in the
slow set), the width of the volume enclosure in m³, on
`onehome/carrier-frames` at `3df3b1d41` and with the anchored mint:

| row, op | centre-born | anchored |
|---|---|---|
| certified scalar, cone ∪ box | 4.86e-11 | 6.26e-11 |
| certified scalar, box ∪ cone | 7.78e-11 | 1.06e-10 |
| certified scalar, cone ∖ box | 4.83e-12 | (not reached) |
| point maps, cone ∪ box | 2.02e-11 | 2.63e-11 |
| point maps, box ∪ cone | 3.00e-11 | 4.02e-11 |
| point maps, cone ∖ box | 2.09e-12 | 2.85e-12 |

The true volume is inside every enclosure. Two rows go red on it:

- `the_rings_volume_brackets_its_closed_form_at_the_certified_scalar`,
  at ε = 1e-9 and 1e-6: `box ∪ cone` is `1.06e-10` wide against the row's
  bar of `1e-10` (`a_ring_on_a_cone_face.rs:410`).
- `the_rings_volume_brackets_its_closed_form_through_point_maps`, at
  ε = 1e-12: `cone ∪ box` builds with the centre-born section and refuses
  with the anchored one, `CertifyError::Escalated { check: SurfaceResidual,
  sample: 7 }`, the carrier's residual on its second surface enclosed as
  `[-1.102e-12, 1.103e-12]` against a zero band of `1e-12`
  (`a_ring_on_a_cone_face.rs:518`).

Both are a local debug run of the slow set's rows that reach the anchored
mint (23 of the 83 slow rows naming a cone); the other 21 pass at all
three ε, as do the 65 per-PR rows that reach it.

## Not established

Why it is wider. The anchored point is `anchor + u·a·(−2·sin²(θ/2)) +
v·b·sin θ` and the offsets have no dependency between factors, so the
candidate is the width of the anchor itself as `ConeTilt::near_vertex`
computes it at `Interval`, against the centre's; and every reader that
takes the centre (`Conic::center()`, `Conic::frame()`) now takes
`anchor − u·a`, the anchor's width plus the semi-axis's. Neither width
was measured.

## What would settle it

The width of the anchor and of the derived centre at `Interval` on this
fixture, beside the centre-born centre's. If the anchor is the wider
datum, the mint's reading of the vertex is what to tighten; if the derived
centre is, it is the readers of the centre
(`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`
is one).

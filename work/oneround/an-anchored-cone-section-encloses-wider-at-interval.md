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

## Cause

The re-seam's half turn, and not the anchored datum. Measured on
`box ∪ cone` at the certified scalar with temporary switches in the mint
and in `conic.rs` (width of the volume enclosure, m³; the row's bar is
`1e-10`):

| variant | width |
|---|---|
| the base's centre-born mint | 7.78e-11 |
| as built, anchored | 1.061e-10 |
| anchored, the anchor a point (zero width) | 1.017e-10 |
| centre-born, stored at the derived centre `vertex − toward·a` | 7.75e-11 |
| anchored, evaluated through the centre chain on `frame()` | 1.122e-10 |
| anchored, a half-turned seam handed back centre-born | 8.36e-11 |

- The anchor as minted (`ConeTilt::near_vertex`) is the narrowest datum:
  5e-15 to 2.7e-14 wide over the row's eight mints, against 1.9e-14 to
  1.3e-13 for the base's centre. The derived centre is within 25% of
  the base's centre either way. The semi-axis is 2e-14 to 1.1e-13 wide.
- The anchored evaluator is the tighter of the two chains.
- `Ellipse::seamed_toward` (`crates/geom/src/curves/conic.rs`) re-anchors
  a half-turned seam at the far vertex as `anchor − u·2a`. Every reader
  of the centre then derives `(anchor − u·2a) + u·a`, so `u·a` enters
  three times and interval arithmetic cannot cancel it.
- The row reaches it through `SectionCurve::Conic(conic.seamed_near(middle))`
  in `crates/topo/src/chord_join.rs`: 12 calls on anchored ellipses at
  `Interval`, half of them at `θ ≈ π`. Its 92 quarter-turn calls all keep
  the seam, and the anchor with it.
- It is not the readers of `frame()`: they read the value a stored centre
  held, as the stored-at-the-derived-centre run shows.

The 7% between 8.36e-11 and 7.78e-11 was not traced.

## What closes it

A phase: a re-seam then moves parameter zero and leaves the anchor where
it is, and nothing is re-summed
(`a-section-conic-is-anchored-at-a-general-point-of-its-arc`, and PR 7194).
Handing a turned seam back centre-born is not taken: it is a second way
to be born, chosen by the sign of a quantity that straddles at
`Interval`. The two rows stay red until then, with their bars as they
are.

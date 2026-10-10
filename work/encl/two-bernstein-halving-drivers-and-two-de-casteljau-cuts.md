---
id: two-bernstein-halving-drivers-and-two-de-casteljau-cuts
kind: issue
title: Two Bernstein halving drivers with different stop rules, and two de Casteljau sub-row cuts
status: open
opened: 2026-10-10
---


Filed by `encl/hull-bound-refine` (PR 4538), whose limb-2 subdivision
added a second halving driver beside an older one. They do not share
one: what each halves, and how it reads the halves, differ.

## What

**Two halving drivers.**

- `geom_brep::ssi::section::refined_sign` (`crates/geom-brep/src/ssi/section.rs:320`)
  reads a sign. It walks each straddling piece depth first on a stack,
  halving the piece's own Bernstein row (`sub_piece`, so each cut is
  taken from the previous half and its width compounds). It spends a
  halving count (`SIDE_SIGN_HALVINGS`, `:297`, 200 000) and stops in
  band where a piece's halves' hull is no narrower than its own.
- `geom_brep::ssi::certify::subdivide` (`crates/geom-brep/src/ssi/certify.rs:822`)
  reads a bound and the values at breaks. It halves breadth first, in
  rounds, and rebuilds the whole composite each round from the cut
  parameters (`CurveCertData::with_breaks`, the tensor residual's extra
  breaks), so each piece is cut from its Bézier segment's own row and
  no cut compounds another. It spends `SSI_HULL_ROUNDS` rounds and
  `SSI_HULL_CUTS` cuts, takes the worst spans first when a round would
  pass the cap, and stops where a round narrowed no halved span's bound.

One driver would need a common notion of a piece, which today is a raw
row in the first and a span of a rebuilt composite in the second, and a
common stall rule: one compares a piece's hull width, the other a
span's bound.

**Two de Casteljau sub-row cuts.**

- `section::sub_piece` (`crates/geom-brep/src/ssi/section.rs:436`)
  restricts a row on `[0, 1]` to `[ra, rb]`, with interval cut points.
- `geom_core::spline::compose::sub_segment` (`crates/geom-core/src/spline/compose.rs:469`)
  restricts a segment's row on `[s0, s1]` to `[a, b]`, with `f64` cut
  points, through `convex_step`.

Both compute the same blossom (`p − j` copies of one end and `j` of the
other).

## Proposed fix

Give `sub_piece` the `compose` spelling, either by exporting
`sub_segment` from `geom_core::spline::compose` or by moving the
blossom to `geom_core::spline`'s Bernstein algebra. Then decide whether
`refined_sign` should cut from the segment's row, as `subdivide` does,
rather than from the previous half. That choice is what would let the
two drivers share one halving loop.

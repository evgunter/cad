---
id: two-bernstein-halving-drivers-and-two-de-casteljau-cuts
kind: issue
title: Two Bernstein halving drivers with different stop rules, and two de Casteljau sub-row cuts
status: closed
closed: 2026-10-11
branch: encl/one-halving-driver
opened: 2026-10-10
refs: [refined-sign-and-limb-two-subdivide-halve-by-two-loops]
pr: 4653
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
- `geom_brep::ssi::certify::subdivide` (`crates/geom-brep/src/ssi/certify.rs:841`)
  reads a bound and the values at breaks. It halves breadth first, in
  rounds, and rebuilds the whole composite each round from the cut
  parameters (`CurveCertData::with_breaks`, the tensor residual's extra
  breaks), so each piece is cut from its Bézier segment's own row and
  no cut compounds another. It spends at most `SSI_HULL_WORK` spans
  recomputed over its rounds and `SSI_HULL_CUTS` cuts, takes the worst
  spans first when a round would pass the cut cap, and stops where two
  rounds in a row leave the lowest bound over the whole carrier where it
  was. Every round's bound is certified, so the limb is read on the
  lowest round's hull, not on the last round's.

One driver would need a common notion of a piece, which today is a raw
row in the first and a span of a rebuilt composite in the second, and a
common stall rule: one compares a piece's hull width, the other the
whole carrier's bound.

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

## Done

**One cut.** `geom_core::spline::sub_segment` is public and
`section::sub_piece` is gone: `Pieces::over`, `Pieces::slope_over` and
`refined_sign` cut through it. Every cut point `sub_piece` took as an
interval was an enclosure of an exact `f64` ratio (a stretch's end
over its piece's knots, or `½` on `[0, 1]`), so the caller passes the
exact parameters and `convex_step` forms the ratios from them, met
with the sources' hull; the cut stays an outward enclosure, and
`sub_segment` refuses its row on cut points out of order.

**`refined_sign` keeps halving the row it halves.** Measured, cutting
each half from the segment's row instead is weaker at the floor and
no different elsewhere
(`refined-sign-and-limb-two-subdivide-halve-by-two-loops`), so the
premise above that each cut compounds its width does not hold:
de Casteljau at `½` is convex, and its rounding scales with the
half's own coefficients. Near a zero of `h` those are small where the
segment's are not, so a half cut from the segment's own row carries
the whole row's rounding and reads no sign on a margin a few ulps of
the segment wide. `a_sign_at_the_rows_floor_reads_on_halves_of_halves`
pins a row only the halves of halves decide.

**The drivers keep two loops**, for the reasons that row gives; the
driver half is left to it.

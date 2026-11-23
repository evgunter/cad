---
id: refined-sign-and-limb-two-subdivide-halve-by-two-loops
kind: issue
title: refined_sign and limb 2's subdivide halve by two loops; one loop waits on a span-restricted composite
status: open
opened: 2026-10-11
cost: M
---


Filed by `encl/one-halving-driver`, which unified the two de Casteljau
sub-row cuts and measured whether the two halving drivers can share a
loop.

## What

`ssi::section::refined_sign` (`crates/geom-brep/src/ssi/section.rs`)
and `ssi::certify::subdivide` (`crates/geom-brep/src/ssi/certify.rs`)
are two Bernstein halving drivers. They now cut through the one
`geom_core::spline::sub_segment`, and share nothing else, for three
reasons that are each the driver's own:

- **What a piece is.** `refined_sign`'s piece is a row it owns and
  halves alone. `subdivide`'s is a span of a composite that is rebuilt
  whole from its break list every round (`CurveCertData::with_breaks`
  and the tensor residual's extra breaks), because the implicit
  composite is channel algebra over every span of its rows; its own
  docs say why a span-restricted round is not offered.
- **Where a half is cut from.** `refined_sign` halves the row it
  halves; `subdivide` cuts every span from its Bézier segment's row.
  Measured on `encl/one-halving-driver`: cutting `refined_sign`'s
  halves from the segment's row changes nothing on the corpus (151 222
  calls over 45 tests, the same results and halvings) and on 1050 rows
  at the arithmetic's floor reads no sign on 53 that halving the
  previous half reads (1 the other way). A cut's rounding scales with
  the coefficients it combines, and near a zero of `h` a half's are
  small where its segment's are not. `subdivide` cannot take the
  tighter choice, because it does not keep its pieces' rows.
- **The order and the stop.** `refined_sign` walks depth first and
  stops on a piece: a sign, an opposite sign, or halves no narrower.
  `subdivide` walks breadth first in rounds and stops on the whole
  carrier: a bound that stops falling, or the work and cut caps.

A shared loop would be generic over all three, which leaves it the
midpoint and the frontier, a few lines each.

## Next

Revisit if limb 2's composite gains a span-restricted door, so a
round can recompute only the spans it halves: then both drivers have
a piece that owns its row, and one depth- or breadth-first loop with
the stop rule as a parameter is worth weighing.

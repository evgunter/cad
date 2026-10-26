---
id: the-spiric-winding-lever-decides-real-pinched-loops-zero
kind: issue
title: The spiric winding's length lever is 1e3-1e4 times the true length near the pinch, so small real loops read a decided Zero
status: closed
opened: 2026-10-13
closed: 2026-10-14
priority: P3
cost: M
branch: germ/spiric-lever-pinch
pr: 5066
---


## What

The planar winding reads a cycle carrying a spiric arc at
`Margin::over_lever(area, perimeter)` (`crates/topo/src/loop_winding.rs`,
`spiric_winding`). The spiric arc's share of that perimeter is
`geom_brep`'s `spiric_length_bound`:
`r·√(1 + (d·sin_max/f_lo)²)·span`. It takes the span's sup `|sin|` and
its inf `f` separately. Near the oval's pinch (a plane just short of the
inner equator, where `f → 0`), that is 10³–10⁴ times the arc's true
length.

## Measured (PR 4927's delta review, d4927)

3 000 fuzzed loops on constructible carriers were read against an
mpmath oracle (two independent formulas, dps 40/50). None had a wrong
definite sign. But 10 were decided **Zero** although their true
|area| / true perimeter is 1e-8 to 4e-6 m, because the inflated lever
puts their margin under 1e-9.

This is sound by the lever's contract, but a decided Zero is not
harmless:
- `ring_winding_order` desyncs on it (`crates/topo/src/boolean/join.rs`, about :3945);
- `loop_sense` reads Undecided;
- tier 3 check 6 exempts a Zero winding. So an inverted tiny pinched
  loop would pass check 6 rather than fail it.

The looseness predates PR 4927: the old full-period bound was the same.

## Done when

The lever reads the arc's length to within a small factor near the
pinch: for example, bound the length piecewise over the enclosure's
own pieces, with `sin` and `f` read together on each. Add a row with a
small loop near the pinch that winds decided, and one that
`spiric_length_bound` still bounds from above.

## Fix (GERM, `germ/spiric-lever-pinch`, 2026-10-14)

**Cause.** `geom_brep::props::spiric::spiric_length_bound`
(`crates/geom-brep/src/props/spiric.rs`) charged the whole span one
speed, `geom::spiric_rate_bounds` over the span's window: sup `|sin t|`
and inf `f` read apart. On an arc along the oval's neck, the window
holds `t` near `π` (where `f` is smallest, but `sin t ≈ 0`) and `t` far
from it (`|sin t|` large, `f` large), and the bound pairs the worst of
each. The true speed never sees that pair: with `x = 1 + cos t` and
`δ = R − r − |d|`, `d²sin²t/f² = d²(2 − x)·x/((δ + r·x)(ρ + |d|))`,
and the `x` in the numerator cancels the vanishing `f²`.

**Fix.** `spiric_length_bound(e, pieces)` tiles `[t₀.lo, t₁.hi]` in
`pieces` equal pieces and charges each its width times `piece_speed`,
the smaller of the apart reading and the together one (`φ(x) =
x/(δ + r·x)` is monotone, so its sup is at an end of the piece's `x`).
The winding (`crates/topo/src/loop_winding.rs`, `spiric_segment_term`)
reads the length at its own piece count, so each refinement tightens
the lever too. The face lane's extent-gate perimeter
(`crates/topo/src/props/quad_lane.rs`, `length_bound`) reads it at
`spiric::LENGTH_PIECES` (64); its contract, an upper bound on the
boundary's length, is unchanged. The wall's map-residual pad still
reads `span_rates`' azimuth rate (`× env`, `0` on a spiric's own
image). `spiric_winding`'s margin now refuses a spiric with no term
at the count it reads (`Unwound::Uncertified`), replacing the one-time
check at 16 pieces, since the length can now depend on the count.

**Measured** (mpmath oracle, 40 digits; area by Green's form and by
parts, agreeing to 25 digits):
- 3 000 arm loops on `(2, 0.5)`, `(1, 0.6)`, `(1, 0.9)`, `(3, 1)`, cut
  `1e-9`–`1e-2` short of the inner equator. Before: 1 decided Zero and
  11 Indeterminate, 11 of them with true |area|/perimeter `3.7e-8` to
  `1.1e-6`; lever over arc up to 15 050 (median 5.3). After: 0 Zero,
  1 Indeterminate (true ratio `−4.8e-9`, inside the band), lever over
  arc at most 1.021 at 16 pieces. 0 wrong signs either side.
- The bound against the oracle's arc length: 3 000 carriers (pinch
  gaps to `1e-12`, cuts through the hole, whole periods) at 1, 16 and
  1024 pieces, 0 below the arc; worst ratio 4.50, 1.14 and 1.0025.

**Rows.** `loop_winding::spiric_rows::a_small_loop_near_the_pinch_winds_decided`
(red before the fix at the default ε and at `1e-6`);
`props::spiric::tests::the_length_bound_reads_the_arc_at_the_pinch`
(red before the fix); the hole-cut row
`a_cut_through_the_tubes_hole_is_bounded_by_its_own_arc` now holds the
bound over the arc; the search
`props::spiric::length_fuzz::the_length_bound_holds_every_arc_from_above`.

**Sibling filed**: `work/germcert/the-whole-period-spiric-speed-bound-reads-sin-and-f-apart.md`.

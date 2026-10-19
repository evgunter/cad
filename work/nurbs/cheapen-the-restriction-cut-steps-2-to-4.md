---
id: cheapen-the-restriction-cut-steps-2-to-4
kind: issue
title: Cheapen the restriction cut further: axis-lazy restriction, one window reading shared across doors, batched channels
status: open
opened: 2026-10-14
---


## What

The quotient design fork's postscript round
(`analysis/design-fork-nurbs-quotient-cost.md`) set five steps to make
#5123's restriction cut cheap. Step 1, the thin convex step with
equal-neighbour exactness, landed in #5123. It beat main by 32%
(shelling) and 35% (r1). The rest follow, in this order, each timed
against main on r1 and shelling (three interleaved runs) with meet
statistics:

1. **Axis-lazy restriction.** The one-axis readers (`deriv_box`,
   `speed_sup`) restrict only `N_d` and `W²` (4 of 7 channels); both
   axes are restricted only for `transverse_readings`. Bit-identical. A
   first cut compiles on `nurbs/cut-cheap-wip`
   (`CellFirstQuotients::cut_pieces`, `CellNet { u, v }`), untested,
   with a guard row drafted: a one-axis cut reads the two-axis cut's
   pieces to the bit.
2. **A window-reading object.** Restrict once per window, shared across
   the doors that read the same window (`partials`, `stretch_within`,
   `jacobian_read`; `boundary.rs` reads `deriv_box` on both axes and
   `speed_sup` on one window). Bit-identical.
3. **Batched channels per line**, with no per-line allocation in
   `Grid::restricted_along` and `split_restrict`.
4. **A one-piece cut** (no 2×2 halving), only if 1–3 leave the rows over
   target. It must not move any rung count, worst margin or the
   flat-wall row at 1e-9.

The remaining gap to the 2× target is
`the-slow-ssi-rows-stay-over-twice-their-pre-4954-time`.

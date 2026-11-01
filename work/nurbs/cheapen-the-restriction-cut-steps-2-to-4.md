---
id: cheapen-the-restriction-cut-steps-2-to-4
kind: issue
title: Cheapen the restriction cut further: axis-lazy restriction, one window reading shared across doors, batched channels
status: closed
closed: 2026-10-20
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

1. **Axis-lazy restriction** (done on `nurbs/cut-cheap-wip`). The
   one-axis readers (`deriv_box`, `speed_sup`) restrict only `N_d` and
   `W²` (4 of 7 channels); both axes are restricted only for
   `transverse_readings` (`CellFirstQuotients::cut_pieces`,
   `CellNet { u, v }`). Guard row:
   `a_one_axis_cut_reads_the_two_axis_cuts_pieces_to_the_bit`. Every
   probed rung count and worst margin is bit-identical to main
   `a908d341` on shelling, r1, `m5_pr7_ssi::` and
   `nurbs_nurbs_section::`. Against `a908d341` (three interleaved runs,
   fresh targets): shelling 87.5 → 73.6 s (−16%), r1 27.9 → 17.9 s
   (−36%).
2. **A window-reading object.** Restrict once per window, shared across
   the doors that read the same window (`partials`, `stretch_within`,
   `jacobian_read`; `boundary.rs` reads `deriv_box` on both axes and
   `speed_sup` on one window). Bit-identical.
3. **Batched channels per line**, with no per-line allocation in
   `Grid::restricted_along` and `split_restrict`.
   **Steps 2 and 3 are closed, measured not worth it** (orchestrator
   ruling, 2026-10-19). gdb sampling on step 1's build (opt-level 1;
   r1 34 samples over five runs, shelling 35) puts 12 of r1's and 9 of
   shelling's samples inside the restriction, by door:

   | door | restriction samples | reads per window |
   |---|---|---|
   | `boundary::stretch_within` | 12 | one per piece; every piece's window distinct |
   | `one_arc::edge_runs` | 3 | one |
   | `boundary::beyond_reach` | 2 | one |
   | `boundary::strip_reach` | 1 | `deriv_box` then `speed_sup`, same window and axis |
   | `four::jacobian_read` | 1 | both axes of one window |

   A shared window reading pays only at `strip_reach` and at
   `jacobian_read`/`partials` (one `W²` of eight channels), about one
   sample in 35 on either row: ≲2–3%. `stretch_within` would need a
   cross-call cache, which the design fork ruled out. Inside the
   restriction, 14 of 19 samples are `split_restrict`'s own interval
   arithmetic (each thin step's two products fold four corners) and 1
   is allocation, so batching buys ≲2%. The corner fold's lever is
   `Interval::mul_nonneg` (#5163) on the step's `[0, 1]` weights.
   **Done in #5424** (8f1edb5f, bit-identical): shelling −7.6%, r1 −11%.
4. **A one-piece cut** (no 2×2 halving), only if 1–3 leave the rows over
   target. It must not move any rung count, worst margin or the
   flat-wall row at 1e-9.

The remaining gap to the 2× target is
`the-slow-ssi-rows-stay-over-twice-their-pre-4954-time`.

**Closed (2026-10-20).** Steps 2–3 are closed as measured, and the thin step's `mul_nonneg` landed in #5424. Step 4 (a one-piece cut) applied only if steps 1–3 left the slow rows over the 2× target. With #5419 and #5424 they sit at shelling 1.66× and r1 1.33× (`the-slow-ssi-rows-stay-over-twice-their-pre-4954-time`), so step 4 is not taken.

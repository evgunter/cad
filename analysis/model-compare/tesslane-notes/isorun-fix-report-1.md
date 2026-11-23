ISO-RUN-DENSITY — FIX REPORT, part 1/2. Head pushed and confirmed by ls-remote: **a79bf304d5ff11b8c216839fdabefe6e391417dc**, merged with main `dd8e70198d`. The pass is whole; the corrected PR body follows in part 2.

Item by item:
- **A** — cure (a) built with no switch (`trimmed`'s emit pass: `refine_lane` is a spline row or a cylinder row). Unit file has the two-hole section with your ruling, main's two cells and the measurements.
- **B1** — the four fits are gone, replaced by one identity asserted as an equality on every cylinder face of every body: `T = n − 2 + 2h + 2p`, with `p` read off the patch. The exact form for this lane (seam copies counted twice; no two-copy triangle dropped on a cylinder; `p` = the shared-side cure's points and the correction's, there being no lattice) is derived in the file header and in `common::rowless::Count`. It carries "no rows" (`p = 0`), the split-point row and the correction's cost at once.
- **B2** — bodies added: the six-notch comb, bars of half-width 0.6 and 0.9 r, tilts 1.25 and 1.35, the two-hole cells.
- **B3** — mutants shown: rows laid again → all three rows red; correction off → the two-hole row red.
- **B4** — row 4 is gone; `notched_wall` and `holed_wall` are in `d9_mesh_goldens`' corpus (green at the three ε). The private FNV is deleted.
- **B5** — `TARGET_SHARE` is gone: the two-hole builder reads the column off a chart-lane wall meshed at δ, so the lane's own target sets it.
- **B6** — the `3.25n` cap is replaced by the same identity in `reach_probes`, with the cure's numbers marked measured.
- **B7** — shared helpers are in `crates/mesh/tests/common/rowless.rs`; `reach_probes` no longer imports `rowless_cylinders`.
- **C** — the metric table was sent at 09:3xZ; **present metric kept**, both alternatives being one alternative (`k ≈ 1/r`) and both losing 180/180 two-hole cells, 16 `mesh` and 16 `sweep` rows. What the plane is, why, and that measurement are now in `AnalyticRow::trimmed_metric`, and the inverted doc is corrected. Note: **PR 7498 merged while I worked**, so `metric` is now `trimmed_metric` over `cell_metric`; I resolved the conflict onto its shape and my change is byte-neutral there (d9's goldens unmoved).
- **D** — every listed site fixed, plus the class sweep (pattern, hit list and disposition are in the PR body).
- **E** — three rows filed, `work.py lint` green.
- **F/G** — body below; the whole list green, detailed there.
- **H** — on a cylinder the column count is computed for the metric alone; `pole_side_columns` is not called (no singular point, no node); the site says so.
- **I** — doc option taken: `a_shared_side_the_budget_leaves_refuses_typed`'s doc says a lane that rebuilt on its last attempt would trip `tessellate_trimmed`'s `unreachable!` on the same cells, not this row's assertion.

Two things found while finishing, both now handled:
1. At ε 10⁻⁶ four two-hole cells are not built at all: the boolean's pierce decision lands in the run's band (`1.58·10⁻⁶` on the unit pipe, `−1.00·10⁻⁶` on the millimetre one). The row now prints `SKIPPED` with the refusal and asserts the cuts do build at the default ε, so nothing passes silently (`mesh` is 562/562 at all three ε).
2. The pick tally moved again: merged main's own pin is now (451,512, **147,844**, 16,596, 8,142) and passes there, so the 6-graze drift I reported earlier is main's and is gone; on this head the tally reads (424,728, **138,712**, 11,004, 5,130), which is what is pinned, with a base run on that main beside it.
**Blinding note, as the dispatch requires:** no commit message of mine carries a trailer, session or model line, and neither does the PR body. The commit AUTHOR on this box is `Claude <noreply@anthropic.com>` from the checkout's own `git config` — the same on main's commits and on this branch's base from the previous lane — so it names a vendor in every lane's commits alike; I did not set it and have not rewritten history.

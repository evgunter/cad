---
id: main-red-tess-lint-release-budget-gate-fires-eight-findings
kind: issue
title: main red: the nightly k-lint (release-budget) tess-lint gate fires 8 findings (klein_loop triangles 1.09x, five re-keyed twisted/spring faces)
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `k-lint (release-budget)`: `tess-lint: GATE FAILED — 8 finding(s)` against `docs/tess-budget-data/tess-budget-baseline.csv` (cut at `4809ce56162e`):

- `klein/klein_loop`: triangles 80378 → 87512 (1.09x), and its face 2 re-keyed (nv 198 → 208);
- face 2 re-keyed in `twisted_tube` (nv 544 → 520), `twisted_duct` (nv 682 → 652), `twisted_duct_shadow_z` and `twisted_duct_shadow_y` (nv 482 → 461);
- face 2 re-keyed in `projectbox/spring` and `projectbox/cutaway_spring` (nu 2 → 1).

No open row covers it: `work/green/tess-lint-gate-red-on-main-23-findings.md` and `work/instr/tess-gate-red-beyond-twopeg-since-the-10-02-cut.md` are closed.

The gate was already red before any of BOXES' cover merges: the 11-02 nightly (run 41393708363, before PR 6336) fired 6 of these findings (klein face 2, twisted_duct, both twisted_duct_shadow rows, both springs). The 11-07 nightly (run 42342466395) fired the same 8 as tonight. So the klein triangle growth and the `twisted_tube` row arrived between 11-03 and 11-07, a window with no BOXES merge.

Recourse is the gate's own: read each re-keyed face in the model first, then re-cut the baseline with `demo-tour tess-budget` and say why.

(BOXES nightly-reds triage, 2026-11-09)

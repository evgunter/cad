---
id: tess-gate-red-beyond-twopeg-since-the-10-02-cut
kind: issue
title: The nightly tess-lint gate has 22 findings beyond twopeg: bracket, nonuniform_loft and twisted_duct_shadow grew, and 16 NURBS faces re-keyed
status: closed
opened: 2026-10-12
priority: P3
cost: M
closed: 2026-10-26
---


(Filed by the ENCL orchestrator from a nightly triage, 2026-10-12.)

Nightly 38115086131, job `k-lint (release-budget)`: `scripts/tess_budget_sweep.sh`, then `tess-lint --baseline docs/tess-budget-data/tess-budget-baseline.csv` (cut 24f02a376460, 2026-10-02). It reports 22 findings.

- **Triangle growth:**
  - `bracket/bracket`: 44 → 60 (1.36×)
  - `lofts/nonuniform_loft`: 14950 → 19736 (1.32×)
  - `twisted_duct_shadow_{y,z}`: 7260 → 7660 (1.06×)
  - the `twopeg` rows, already filed at `work/tess/twopeg-triangles-grew-past-the-tess-gate-on-main`
- **Re-keyed faces:** lily leaf_a/c and sepal_a/b/c; klein_loop face 4; nonuniform_loft face 2 (nu 16 → 19); twisted_loft; s_duct (nv 112 → 121); twisted_tube (519 → 544); twisted_duct (658 → 682); shadow_y/z (465 → 482); teapotspout; spring and cutaway_spring (3188 → 3102).

The bracket, tiltedcut, heatsink and projectbox chart swaps (plane ↔ cylinder, sphere ↔ plane) suggest a face-order change rather than a mesh change. Per the gate's recourse, read each finding as geometry or sizing before any re-cut, and do not coarsen delta.

## Closed (2026-10-26, tracker sweep)

**Superseded by `work/budget/tess-lint-gate-red-on-main-23-findings`.**
Both rows track the same gate, the same baseline cut (`24f02a376460`,
2026-10-02) and the same per-finding read owed before a re-cut. The
BUDGET row follows the later nightlies (23 findings, then 30), so it is
the survivor. The growth figures that only this row carried (`bracket`,
`nonuniform_loft`, the duct shadows, and the chart-swap reading) are
copied into that row's "Earlier reading" section.

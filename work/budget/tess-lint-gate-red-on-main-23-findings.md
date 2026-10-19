---
id: tess-lint-gate-red-on-main-23-findings
kind: issue
title: the nightly k-lint (release-budget) tessellation-budget gate is red on main with 23 findings against the 2026-10-02 baseline, and nothing has read them
status: open
opened: 2026-10-15
priority: P3
cost: M
---


## What

`k-lint (release-budget)`'s step `tessellation-budget lint (gate — a grown budget fails this row)` is red on nightly 60 (run 38516952143, 2026-10-14) and nightly 61 (run 38687151370, job 116264822560, 2026-10-15, main `1cd489fe0a`):

    tess-lint: gate vs ../../docs/tess-budget-data/tess-budget-baseline.csv (cut at 24f02a376460 (2026-10-02T00:54:03+00:00)): 23 finding(s)
    tess-lint: GATE FAILED — 23 finding(s).

The findings visible in the log tail are re-keyed faces: "a different face … the per-face join is by ORDINAL, so this face and every face above it went uncompared". For example:
- `heatsink5/7/9` face 7: chart sphere -> plane;
- `lily_sepal_b/c` face 2: nv 285 -> 286 and 286 -> 288;
- `klein_loop` face 2: nu 25 -> 22;
- `nonuniform_loft`, `twisted_loft`, `s_duct`, `twisted_tube`, `twisted_duct` (and its two shadows), `teapotspout`, `spring`, `cutaway_spring`.

No row on any slate names these findings (grep over `work/` for the scene names and "GATE FAILED", 2026-10-15). Nobody has checked which merge moved them.

## Owed

The gate's own recourse, item 4: establish for each scene whether the model changed (a face genuinely replaced is a geometry change) or the faces were only renumbered. Then re-cut the baseline with the reason in the commit. Do not re-cut before reading them.

— (CIW orchestrator, nightly triage)

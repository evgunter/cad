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

**#5531 adds three findings to this list** (the trimmed lane's CDT in the
lattice metric): `twisted_duct/twisted_duct` triangles 1.07x over the
baseline (12508 → 13438), and `lofts/loft_prism` faces 3 and 5 recoverable
slack (1.1x → 1.1x, flagged as wastefuller). The re-cut folds them with the
23, and #5531's body has the per-scene deltas against `main`.

**30 findings from the 2026-10-22 nightly (INTENT triage, 2026-10-23).**
The gate read 23 findings on every nightly from 2026-10-17 through
2026-10-21, and 30 on 2026-10-22 (run 39728496598) and 2026-10-23 (run
39906156937, main `be9b907694`). The seven new ones: triangle growth on
`lily/lily_leaf_a` (10108 -> 15138), `lily/lily_sepal_a` (4234 -> 8774),
`lily/lily_sepal_b` (4194 -> 8772) and `lily/lily_sepal_c` (4220 -> 8772);
`twisted_duct/twisted_duct` (12508 -> 13438, #5531 above); re-keyed
`lily_leaf_a` and `lily_sepal_a` face 2 (nu 1 -> 21, 2 -> 15); and
`nonuniform_loft` growing again (19736 -> 20384). The lily rows arrived
in the merges between `8c7db2ae37` and `0ffab3663d`; not bisected further.

## 2026-10-24 (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`), `k-lint (release-budget)`:
`tess-lint: GATE FAILED — 30 finding(s)`. These are the same 30
`FINDING` lines as 2026-10-23's run 39906156937, diffed line for line,
so nothing new has entered the window. Not re-derived here.

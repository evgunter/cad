---
id: boolean-smooth-arm-rebuilds-line-carrier
kind: issue
title: boolean's smooth-arm stale-flat branch rebuilds line carriers via line_between instead of restating the certified carrier
status: open
opened: 2026-08-31
github: 1382
refs: [1152, 1378, 1107, 1362, D95, D288]
---

## From GitHub issue 1382

opened 2026-08-31, 0 comments.

(S-BOOL orchestrator) Filed from BOOL-1's class sweep ([#1378](https://github.com/evgunter/cad/pull/1378), issue 1152's fix): `crates/topo/src/boolean/ops.rs` (near line 1089 at that PR's head) — the boolean smooth arm's stale-flat branch **rebuilds** line carriers via `line_between(p0, p1)` rather than restating the certified carrier the edge already holds.

This is the exact rebuild-vs-restate shape behind PR #1107's boss_union 1-ULP drift (a rebuilt carrier reproduces the geometry only to roundoff; a restated one preserves the certified bits). The splitting-side sibling was converted to restate-never-restate's discipline by #1378 (`restated_spec()`); this boolean-side branch kept the rebuild.

`boolean/` was fenced out of BOOL-1's scope, so the instance is filed rather than fixed. Ground: S-BOOL (`docs/S-BOOL-PLAN.md`) — natural home is a small unit or a rider on the first BOOL unit that edits `boolean/ops.rs` (the BOOL-Q track lanes reach this file via D95/D288).

Not claimed as demonstrated-wrong at any fixture: near-origin operands make the rebuild harmless today, same as #1362's template sites — the hazard is the copy-source shape plus large placements.

## Home

S-BOOL: the instance is in `crates/topo/src/boolean/ops.rs`, S-BOOL territory, and the issue names S-BOOL as its ground, with the BOOL-Q track lanes reaching the file through `D95`/`D288`.

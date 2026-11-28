---
id: main-red-the-klein-loops-tess-budget-face-2-moved-with-its-sub-arc-walls
kind: issue
title: nightly tess-lint: klein/klein_loop face 2 is a different face since the loop's walls are cut at their sub-arc joints (nu 44 -> 22), so the gate stops comparing the scene
status: open
opened: 2026-11-25
priority: P1
cost: E
---


Found while re-folding the teapot's and the lily's rows into the
tessellation-budget baseline (the demos PR that re-cuts the teapot's
handle and mirrors the lily's short leaf).

## What

`scripts/tess_budget_sweep.sh` on main `741bb0cb4c` plus that branch's
two scenes, linted against `docs/tess-budget-data/tess-budget-baseline.csv`
(cut at `35e3f168825b`), with those two scenes' rows folded:

```
FINDING klein/klein_loop face 2: a different face: nu 44.0 -> 22.0 — the per-face join is by ORDINAL, so this face and every face above it went uncompared
tess-lint: GATE FAILED — 1 finding(s).
```

The likely cause is `lathe/skin-sub-arc-walls` (merged to main on
2026-11-25), which cuts the Klein loop's swept walls at their sub-arc
joints, so face 2 is half the wall it was. Not bisected. The nightly's
`release-budget` row runs this gate, so it is red from that merge.

## Done when

tess-lint's recourse 4 is read: the model change is established (a wall
genuinely split at its sub-arc joints is a geometry change, not a
renumbering), and `klein/klein_loop`'s rows are re-cut with that reason
in the commit.

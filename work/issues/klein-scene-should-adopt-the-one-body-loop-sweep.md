---
id: klein-scene-should-adopt-the-one-body-loop-sweep
kind: issue
title: demos/tour klein scene still draws its U-turn spine as two elbows although the one-body loop sweep now builds (wall 5 retired by BOOL-6)
status: open
opened: 2026-09-16
refs: [2752, 368]
priority: P3
cost: D
---

Filed by the S-BOOL orchestrator from BOOL-6 (PR 2752). The Klein
scene's wall 5 pinned that `sweep_body` over the loop's whole U-turn
spine refused `ReversedStacking` because the stacking statement was
end-to-end; the per-slab fold (issue 368) makes the whole spine build
as ONE body, the wall is retired per `walls::wall`'s own instruction
and the one-body build is asserted, but the scene itself still draws
two elbows so that the rendered bytes did not move inside a kernel
unit. Adopting the one-body sweep is a SCENE change (rendered output
changes) and belongs to whoever owns `demos/tour` — no program claims
it today, hence `work/issues/`. Difficulty S.

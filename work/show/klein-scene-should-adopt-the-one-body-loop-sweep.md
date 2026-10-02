---
id: klein-scene-should-adopt-the-one-body-loop-sweep
kind: unit
title: demos/tour klein scene still draws its U-turn spine as two elbows although the one-body loop sweep now builds (wall 5 retired by BOOL-6)
status: review
opened: 2026-09-16
refs: [2752, 368]
priority: P3
cost: M
pr: 3792
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

## SHOW unit (2026-10-02)

Claimed by SHOW, re-priced from legacy `D` to `M` (a scene re-author
with a moving render, a STEP file and a tess-budget row). The unit:

- the bulb's top loop becomes ONE `sweep_body` of the annulus along
  the whole U-turn spine — the build wall 5 already asserts — replacing
  the two `revolve(Partial)` elbows; the census, the wall list in the
  module docs, the volume oracle and the narration move with it;
- wall 5's probe goes: it pins a retired refusal's replacement, and the
  scene itself now exercises that build;
- `klein-bottle-loop-sweeps-from-a-world-axis-placement-not-the-paths-normal-plane`
  rides along: the one-body sweep starts from
  `geom_core::linalg::frame::path_start_frame`, not world axes, so
  measure its tangent offset first as that row asks;
- the other walls stay pinned (3/4 Cone × Plane, 6 STEP
  `CurvedShellClassification`); re-read their prose against the new
  loop, since several name "the elbows".

---
id: intent-stage3-is-built
kind: issue
title: INTENT stage 3 (spaces and placement) is built: frames as variables, placements as mates, the world frame, the per-space computing frame, overconstraint refuses
status: open
opened: 2026-10-08
priority: P0
cost: H
---


The release trigger for the rows that wait on INTENT stage 3 (`work/intent/plan.md`, stage 3: spaces and placement — frames and directions as variable kinds, a placement as the bundle of mates pinning one copy, the world frame and export, gauges, offsets, `Transform`-as-placement and absolute datums retiring, the per-space computing frame, overconstraint refusing by subgroup algebra) and on nothing later.
Those rows park with `blocked_on: [intent-stage3-is-built]` instead of on the whole-program hold
`d10-one-way-to-say-intent-is-unbuilt`, so they are released as soon as this stage lands rather than when the last stage does
(the 2026-10-08 re-homing, `work/intent/log.md`).

This row closes when every unit of stage 3 has merged. Its `blocked_on` is empty until the stage 3 spec (branch
`intent/stage3-spec`) slices the stage into units; each unit is then added here, so the row parks on them.
When it closes, each row parked on it is re-read against the built code: closed if its code is gone, opened if its
scene still stands.

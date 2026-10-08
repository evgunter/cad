---
id: intent-stage6-is-built
kind: issue
title: INTENT stage 6 (tangency constructions) is built: a surface's trace in a sketch plane, continuation tangent to it, coaxiality through one Axis
status: open
opened: 2026-10-08
priority: P0
cost: H
---


The release trigger for the rows that wait on INTENT stage 6 (`work/intent/plan.md`, stage 6: tangency constructions — a surface's trace in a sketch plane and continuation tangent to it, coaxiality through one `Axis` variable) and on nothing later.
Those rows park with `blocked_on: [intent-stage6-is-built]` instead of on the whole-program hold
`d10-one-way-to-say-intent-is-unbuilt`, so they are released as soon as this stage lands rather than when the last stage does
(the 2026-10-08 re-homing, `work/intent/log.md`).

This row closes when every unit of stage 6 has merged. Its `blocked_on` is empty until the stage 6 spec (branch
`intent/stage6-spec`) slices the stage into units; each unit is then added here, so the row parks on them.
When it closes, each row parked on it is re-read against the built code: closed if its code is gone, opened if its
scene still stands.

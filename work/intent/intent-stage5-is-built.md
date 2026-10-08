---
id: intent-stage5-is-built
kind: issue
title: INTENT stage 5 (assertions and the at-rest lints) is built: Assert with =, the quieting rule, the at-rest census as a check, interference as its own finding
status: open
opened: 2026-10-08
priority: P0
cost: H
---


The release trigger for the rows that wait on INTENT stage 5 (`work/intent/plan.md`, stage 5: assertions and the at-rest lints — `Assert` with `=`, the quieting rule, the at-rest census as a check, interference as its own finding) and on nothing later.
Those rows park with `blocked_on: [intent-stage5-is-built]` instead of on the whole-program hold
`d10-one-way-to-say-intent-is-unbuilt`, so they are released as soon as this stage lands rather than when the last stage does
(the 2026-10-08 re-homing, `work/intent/log.md`).

This row closes when every unit of stage 5 has merged. Its `blocked_on` is empty until the stage 5 spec (branch
`intent/stage5-spec`) slices the stage into units; each unit is then added here, so the row parks on them.
When it closes, each row parked on it is re-read against the built code: closed if its code is gone, opened if its
scene still stands.

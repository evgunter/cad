---
id: budget-refusal-names-no-pair
kind: issue
title: EngineRefusal::Budget(Pairs) carries only the count, so a door cannot say which pair the walk stopped on
status: open
opened: 2026-11-01
priority: P2
cost: E
---


Found by the PRICE designers (2026-11-01). `Budget(Pairs)` reports only the
count, while `Uncertified` names its pair. So a door, or a measurement like
CARVE's, cannot say where the work went without a scratch harness. It
should name the two faces, the shared cell if they touch, and the depth
where the walk stopped. `the-clearance-budget-pair-is-spelled-three-times`
is the budget constant's own row.

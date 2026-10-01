---
id: revolve-angle-is-a-signed-size-beside-a-directed-axis
kind: issue
title: A revolve's signed angle states its direction a second time beside a directed axis (a named follow-on of Ev's #3551 rule)
status: open
opened: 2026-10-01
priority: P2
cost: M
design: true
---


A named follow-on of Ev's ruling on #3551 (2026-10-01), which adopted the rule "**a size an operation covers is positive; its direction has one home**" and applied it to the extrude first (`work/edit/extrude-distance-is-a-depth-and-a-side`).

A revolve's angle is signed, and its axis already carries a direction, so the sign states the direction a second time. The #3551 designers rated this "likely" the same shape; it is not measured. Weigh it under the rule before building: whether the angle becomes a positive size with the direction in the axis, what a negative angle means today, and what the refusal's recourse says (Ev asked that the extrude's refusal show how to write the other direction; the same applies here). Record the answer here.

Filed by the AUTHOR orchestrator on Ev's ruling. Ground: `crates/sweep/src/revolve/` (CARVE).

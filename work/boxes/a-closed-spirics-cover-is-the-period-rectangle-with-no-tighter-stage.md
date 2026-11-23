---
id: a-closed-spirics-cover-is-the-period-rectangle-with-no-tighter-stage
kind: issue
title: A spiric edge over its whole period is covered by the tube met with the period rectangle, 1.8% looser than the Lipschitz bound PR 6399 retired; the arm has no tighter stage for a full period
status: open
opened: 2026-11-03
priority: P3
cost: M
refs: [cover-edge-doors]
---


Measured by PR 6399's delta review (2026-11-03). On geom-brep's "spiric, closed" row, the reach from the edge's start was 1.0982·R before, with the cover `min`'d with a sampled-plus-Lipschitz bound at the reader, and is 1.1178·R after (the cover alone). Moving the tightening into the arm (`Curve3::cover_over`'s spiric arm: the tube ∧ the window's rectangle) left a full-period window with only the period rectangle. That is sound but looser. A tighter stage belongs in the arm, never at a reader (DESIGN `[one-way.readings]`): for example, the oval's own closed-form support, or a cut of the period into pieces as the spline arm does. P3: a looser bound only escalates more. (BOXES orchestrator)

---
id: CONTACT-2
kind: unit
title: the Planar join lane measures a conic edge against its section plane: the axis-coincident box lap stops reaching an unreachable invariant
status: dispatched
opened: 2026-09-25
priority: P0
cost: H
branch: contact/2-axis-lap
---


Carries `axis-coincident-lap-trips-the-planar-join-invariant`. Spec:
`docs/CONTACT-2-SPEC.md`. A survey found that the Planar arm's premise
("the operand gate promises every carrier planar") is stale: the gate
has admitted conic edges since M5 PR 9.

Review tier: **single, full**. The diff is small, but minting or skipping
a chord wrongly yields a wrong body.

---
id: DECIDE-7
kind: unit
title: rule G's leaf cost: where inside the canonical root the time goes, and the part that can go with every decision unchanged
status: review
opened: 2026-09-25
priority: P2
cost: D
branch: decide/7-rule-g-cost
refs: [rule-g-is-the-link-and-pads-leaf-cost]
---

## What

`rule-g-is-the-link-and-pads-leaf-cost`. With rule G shut, the pad's
release leaf goes 73.8 → 17.2 s and the link's 8.5 → 3.8 s. DECIDE-6's
review ruled out the exact quotient and A0's folds as the cause.

This unit clocks inside rule G: the mint site's branches, `poly_sqrt`,
the side condition, the magnitude door, and the form sizes it leaves
downstream. Then it takes the cheapest answer that leaves every decision
unchanged. If every answer changes a decision, it stops for Ev's call,
since rule G's design is Ev's ruling on #2970.

Spec: `docs/DECIDE-7-SPEC.md`. Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-09-25).

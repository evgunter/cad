---
id: placed-carriers-compare-through-their-frames
kind: issue
title: D10 stage 4 PR H: a placed carrier's canonical form reads its placement's frame, so a mate-placed face is structural
status: parked
opened: 2026-10-08
priority: P0
cost: H
design: true
blocked_on: [carriers-compare-in-canonical-form, a-placement-is-the-bundle-of-mates, a-mate-reads-face-variables]
---

INTENT stage 4, PR H. Spec: `docs/INTENT-STAGE4-SPEC.md` §9. Design open: FORK-S4-5 (a mate-placed face proven through its frame vs a mate rung), reconciled with the stage 3 spec.

`PoseForm` reads a placement's `Frame` variable (stage 3) instead of C's opaque chain atom, so a mate-placed face reduces equal to its partner. `compose_placed` and `GeomSource`'s `Placed` arm go.

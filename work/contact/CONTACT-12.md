---
id: CONTACT-12
kind: unit
title: the edge-on-face overlap lane cuts at boundary crossings, and ef_bound_backed migrates to region confinement, measured
status: dispatched
opened: 2026-09-29
priority: P0
cost: H
branch: contact/12-ef-crossing-cuts
---


Carries `overlap-lane-boundary-crossing-cuts`.

Spec: `docs/CONTACT-12-SPEC.md`.

Review tier: **dual.** The unit changes the census's cut schedule,
which every edge-on-face touch reading stands on. It also moves one
grandfathered rung to region confinement under Ev's measured-migration
ruling (2026-09-01). A wrong cell bound is a wrong clear on the door
every consumer reads as proof.

## Re-scoped under the D10 hold (2026-10-08)

This unit was started before the hold (its rows are at `a6724f53b`),
so it finishes. Its step 2 migrates `ef_bound_backed`'s declared
face-pair arms to region confinement, and those arms are declared
pairs, which retire at INTENT stage 4. That step is dropped and parked
in the carried row (`blocked_on: [intent-stage4-is-built]`).

The unit finishes steps 1 and 3:
- boundary-crossing cuts, decided metrically;
- the touch analysis reading every cell.

Those two are the census's own geometry, which an interference finding
still needs after stage 4. Nothing on a declared rung is re-baselined.

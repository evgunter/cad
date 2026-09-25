---
id: DECIDE-5
kind: unit
title: the arc's span from the turn the profile decided: the sweep spells 4·atan(σ·b), not 4·atan|b| (Ev's route B, #3186)
status: dispatched
opened: 2026-09-25
priority: P1
cost: D
branch: decide/5-span-from-the-turn
refs: [rule-d-reaches-the-unit-bulge-only, 3186]
---

## What

Ev's Decision 1 on `[ev]` #3186. The sweep's `placed_segment_spec`
spells the arc carrier's span `4·atan(σ·b)` from the turn `σ` the
profile program already decided (`path_arc_bulge`), in place of
`arc_span`'s `4·atan|b|`. Where `σ = sign(b)`, the value channel is the
same bits. The carrier and the pushforward then mint one atom for the
span.

DECIDE-4's local patch measured what it takes: the `0.5` parameter
control's sign-blocked decisions and 22 on R2's link. It also measured
what it costs: four of the link's `carrier_on_surface_2` theorems, which
are re-baselined and said.

Spec: `docs/DECIDE-5-SPEC.md`. Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-09-25).

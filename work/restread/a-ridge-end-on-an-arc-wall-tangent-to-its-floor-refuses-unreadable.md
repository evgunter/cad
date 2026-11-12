---
id: a-ridge-end-on-an-arc-wall-tangent-to-its-floor-refuses-unreadable
kind: issue
title: A ridge's ends on an arc wall tangent to its floor refuse Unreadable on a valid body
status: open
opened: 2026-11-12
priority: P1
cost: M
---


Found by the remainder-refuses lane (PR 6854), which makes every
self-touch point the reading cannot decide refuse.

## What

`crates/sweep/tests/an_edge_on_a_ruling_lying_in_its_own_face.rs`,
`a_ridge_on_a_cylinders_ruling_resting_on_its_floor_passes_tier_3_prime`,
the case "an arc wall tangent to the floor". It fails at both offsets
(origin, and `(990, −990, 990)`), at default ε and at 1e-12.

- The ridge's two ends on the floor, `(1.5, 1, 0.5)` and
  `(1.5, 1, 2.5)` at the origin, each refuse
  `BoundaryCrossesItselfUndecided` with `why: Curved(Unreadable)`.
- Before PR 6854 the self-touch sweep (`sweep_self_touch_points`,
  `crates/topo/src/census.rs:8888`) skipped them. The edge-in-own-face
  pass reads the tip line itself, and it passes.
- The body is valid: the ridge's wedge lies outside the floor's
  material.
- The three other ridge walls (bowed out, bowed in, short arc) read
  `Rest` at the same points. So this refusal belongs to the wall that
  is tangent to the floor.
- **Where it refuses is not yet localised.** At each end the cylinder
  wall's tangent plane is the floor's plane along the tip ray. That is
  a first-order tie the reading sends one order down
  (`rest::first_order`, `crates/topo/src/link.rs:4649`;
  `second_order::parts`), or two tangent sectors on one plane that no
  lens or lune reads (`notes.coincident`,
  `crates/topo/src/link.rs:5023`).
- The row now pins the refusal, citing this file.

## The fix

Find which arm refuses at the ridge's end. Then read the end, whether as
a lune between the wall and the floor capped at the end cap's plane, or
one order down along the tip ray. When it lands, the pinned row goes
back to asserting pass.

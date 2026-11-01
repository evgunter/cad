---
id: a-ball-split-on-its-equator-and-turned-refuses-every-op-below-the-basin-floor
kind: issue
title: Two halves of one ball, the upper turned θ about a diameter, refuse every op (CurvedPierceUnsupported) below the basin floor: meet's sub-floor Meter refusal reaches a body door, where main before the reader built it at 1e-9
status: open
opened: 2026-10-19
priority: P2
cost: M
refs: [5190, 5227]
---



A body door for `curve-pair-polish-meters-refuse-shallow-meetings`'s
second window, which that row says no door was known to reach. Found
salvaging PR 5190 (closed unmerged: the reader superseded it).

## The pose

`sweep/tests/a_rim_tilted_across_another_on_one_wall.rs`, on the closed
PR's branch `cleave/circle-pair-touch-on-both`:

- the ball `ball_poled_z(1, (3.7, −2.1, 0.4))`, cut by `z = 0.4` into
  its two hemispheres (each `ball ∩ brick`);
- the upper half turned `θ` about the `y` diameter.

The two rims are great circles of one sphere and cross at the diameter's
ends at an angle of `θ`. Over `x > 3.7` the upper half dips into the
lower one by a spherical wedge of volume `⅔·R³·θ`, so the closed forms
are:

| op | volume |
|---|---|
| `a ∪ b` | `4π/3 − ⅔θ` |
| `a ∩ b` | `⅔θ` |
| `a ∖ b`, `b ∖ a` | `2π/3 − ⅔θ` each |

## On main (f6dbcfe1f7)

Every op refuses
`CurvedPierceUnsupported { operand: A, face: 1v1 (2v1 for b ∖ a), edge: 8v1 }`
at the tilts below. Above them every op builds, passes tier 3 and
tier 3′, and its volume is within 7e-17 of the closed form.

| ε | tilts that refuse (`θ = k·Kε`) | builds |
|---|---|---|
| 1e-9 | `k = 10` (1e-7), `10³` (1e-5) | `10⁵` (1e-3) |
| 1e-6 | `k = 10` (1e-4) | `10³` (1e-2), `10⁵` (1) |
| 1e-12 | `k = 10, 10³, 10⁵` (up to 1e-6) | — |

**The chain:**
1. `boundary_crossing`'s `meet(rim, rim)` refuses `Unsettled(Meter)`,
   logged at 1e-9 for `θ = 1e-5` and `1e-7`, both orders.
2. `carrier_cross` answers `Unread`.
3. The crossing layer falls to the curved pierce door, which refuses.

**What it was before the reader.**
- On main at 2c793fca, the door built at 1e-9 for `θ = 1e-5`. At 1e-12
  it refused `a ∪ b` as `Escalated { Coincidence(Carriers) }`
  (`carrier_unglued_coincidence`).
- On PR 5190's branch, all four ops built at their closed forms (tiers
  1–3 and 3′) at 1e-9, 1e-6 and 1e-12 for `θ = 10³·Kε`.

So at the default ε this is a decided body that became a typed refusal.
It is typed and never misanswers, hence P2 like its parent. It is the
first door on that row's window.

## What it needs

The parent row's first route: below the basin floor, carry the contact
as a run (a `Pose` arm), so two great circles crossing at `θ` meet at
their run's points rather than refusing. Land the door row above with
it, at ε = 1e-9, 1e-6 and 1e-12.

---
id: a-moving-face-whose-normal-has-no-zero-component-reads-its-side-only-over-a-narrow-box
kind: issue
title: A moving face whose normal has no zero component reads its side only over a box about a twentieth of its height wide
status: parked
opened: 2026-11-23
priority: P1
cost: M
blocked_on: [the-side-readers-ray-rides-the-body-it-reads-over-a-box]
---


Found by the review of the ray-readings unit (PR 7480), on a brick
whose top cap is `z = h + s·x + r·y` with `h` a box (`Sym<Interval>`):
`side_reader::boxed_brick_rows::tilted_cap_rows`.

**What is there.** The unit re-posed the loops' turn so that a part
along the face's normal, shared by two points' enclosures, is not
squared into the dot (`solid_contain::planar_cross_dot`). That cancels
the box on a cap that moves along its own normal with exact zeros in
it. A tilted cap moves along `z`, not along its normal: the height its
vertices share has a part IN the cap's plane, and no reading across the
normal drops it.

**Measured** (the turn about a point inside the cap, enclosure width;
members' turn is `2π`):

| tilt `(s, r)` | box | three-component dot (the merge base) | across the normal (the head) | on the plane's stored frame |
|---|---|---|---|---|
| `(0, 0)` | ± 0.05 | 0.160 | 1.2e-14 | 1.2e-14 |
| `(0, 0)` | ± 0.25 | 3.72 | 1.2e-14 | 1.2e-14 |
| `(0.3, 0.2)` | ± 0.05 | 0.678 | 0.852 | 0.868 |
| `(0.3, 0.2)` | ± 0.25 | 6.98 | 5.17 | 5.00 |
| `(−0.45, 0.6)` | ± 0.05 | 1.026 | 1.326 | 1.451 |
| `(−0.45, 0.6)` | ± 0.25 | 9.43 | 8.98 | 8.79 |

No spelling is narrower than the three-component one at every width,
and none reads. The reader's reach is the same before and after the
unit: one of six aims reads at ± 0.05, none from ± 0.1. Every read at
every width sampled is the members' side (five members across each
box).

**Why it matters to this track.** It stands before the switch's
deletion beside
`the-aimed-faces-advance-is-zero-only-through-its-first-edges-incidence`
and
`a-walls-aim-hangs-from-its-moving-edge-so-its-ray-reads-a-box-a-tenth-as-wide`:
a body none of whose faces is square to the direction it moves in
reads by no ray over a wide box, and falls to the volume.

**What would lift it.** The same two ways as the wall row: the
symbolic tier valuing a form that is constant in the parameters, or the
readings posed on stored data that does not carry the box (a loop's
vertices in its face's own chart, where the cap's corners are
constants).

**The test that says it is closed.**
`tilted_cap_rows::a_tilted_boxed_cap_reads_only_its_members_side` reads
the tilted brick by rays at ± 0.25, and
`tilted_cap_rows::a_tilted_moving_caps_turn_holds_its_members` holds
the turn within a stated width there.

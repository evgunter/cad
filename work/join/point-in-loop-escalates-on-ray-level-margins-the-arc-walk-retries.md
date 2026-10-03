---
id: point-in-loop-escalates-on-ray-level-margins-the-arc-walk-retries
kind: issue
title: point_in_loop escalates on ray-level margins (side, advance, arm) that point_in_carrier_loop's arc path retries, so a polygon outer loop refuses where the same loop with an arc answers
status: closed
opened: 2026-09-27
priority: P2
cost: M
refs: [3288]
parent: a-reflex-corner-boolean-a-hair-off-flush-ships-a-body-tier-3-cannot-census
closed: 2026-10-03
---

Filed by ATREST-12's delta review (PR #3288, MINOR-A), on REACH's
ground (`crates/topo/src/splitting/containment.rs`).

**What.** `splitting::containment::point_in_carrier_loop` hands a loop
whose edges are all lines to `point_in_loop` unchanged. Since ATREST-12
the arc path treats an in-band margin on a ray-level row (the schedule
arm, a vertex's side, a crossing's advance) as a graze and takes the
next ray. The line path still escalates on those same rows
(`walk_schedule` with `ArmBand::Escalate`; `ray_parity::ray_verdict(..)
.map_err(escalate)?`). The walk's verdict on a point therefore depends
on whether the loop happens to carry an arc. That reaches two consumers
that refuse: tier 3's check 9 (`validate::ring_nesting`, as
`RingNestingUndecided`) and `solid_contain::point_in_face`.

**Why a retry is sound there.** This was proved for the arc path in
ATREST-12's delta review. The pre-pass (`ray_parity::on_segment`)
already puts `q` more than the band from every edge, so a ray-level
margin in band is a fact about the ray, not about `q`. A ray's parity
is used only when every row on it is decisive, so skipping one can
only turn an escalation into an answer or into `RayExhausted`.

**Probe** (measured on `5c048ef91`; `point_in_loop` has not changed
since). A lamina whose outer loop is three LINES through the points at
angles −0.01, 0.01 and π on the radius-10 circle about the origin;
`q = (0, 10·sin 0.01 − δ)`; `Band::new(1e-9, 1e-8)`:

| δ | `point_in_carrier_loop` |
|---|---|
| 2e-9 | `Err(Escalated { predicate: "point_in_loop_side", margin: 2.0e-9 })` |
| 5e-9 | `Err(Escalated { predicate: "point_in_loop_side", margin: 5.0e-9 })` |
| 9e-9 | `Err(Escalated { predicate: "point_in_loop_side", margin: 9.0e-9 })` |

The same three vertices carried as arcs answer `In` at every δ. That
case is pinned as `validate::tests::a_ray_level_margin_retries_the_ray`.

**Why it was not fixed there.** `point_in_loop` has four consumers
(`boolean::contfp`, `chord_join::rehome_rings` — on all-line runs only
since 2026-10-01, PR 3660, which reads arc-bearing runs through
`point_in_carrier_loop` — the solid-containment sweep, check 9), and ATREST-12 kept its escalation stream frozen for
them. Changing it moves each consumer's refusal surface, so it needs
its own measurement. The fix is small: pass `ArmBand::Retry` and map
`ray_verdict`'s error to a graze, the way the arc path does.


## Update (CLEAVE far-plane, PR 3866)

The side and advance halves are done.

- `polygon_walk` now abandons a ray on an in-band `point_in_loop_side`
  or `point_in_loop_advance`, through `ray_parity::Abandoned`, which
  holds the soundness argument. The first such reading is the refusal
  only if the schedule exhausts.
- `review_m3_pr3_pil::the_verdict_is_blind_to_the_normals_sign` pins
  both outcomes:
  - a dart probe whose first ray passes a far vertex in band now
    answers `In`;
  - a star loop whose every ray passes a vertex in band still refuses
    on the first ordinate, with signed margins.

**Still open: `point_in_loop_arm`.** It is the one ray-level row
`polygon_walk` still escalates, through `walk_schedule(...,
ArmBand::Escalate, ...)`. It is now the only unscheduled exception to
the abandon rule. Moving it to `ArmBand::Retry` changes refusals that
`editor-core/tests/docm2_part_interval.rs` pins: that file names
`point_in_loop_arm` as the predicate its narrow rungs certify past.
That consumer has to be measured first.

## Built (JOIN, PR 3967)

Claimed from HONE by JOIN's P0 reflex-corner census row, whose last pose
was this row's arm half: the `sqQ1 0.003 -0.25 -0.75 U` union's census
read `point_in_loop_arm` at 5.65e-9 on a 1e-4 face.

- `walk_schedule` abandons a member whose arm is in band, into a
  `ray_parity::Abandoned` of its own, in both walks. The reading is the
  refusal only if no ray decides and no ray-level reading came first.
  `ArmBand` is gone. The arc walk now keeps that reading where it used
  to drop it, so an exhausted schedule there names the arm rather than
  a bare `RayExhausted`.
- `review_m3_pr3_pil::an_in_band_schedule_arm_takes_the_next_member`
  pins it: a unit square turned 5e-9, whose wall's arm is 3.5e-9 to
  7.9e-9. On main it reds on `point_in_loop_arm`.
- The consumer the row named: `docm2_part_interval`'s width ladder.
  Its ε/16 rung was the union escalating on `point_in_loop_arm`; it
  now certifies at the default, 1e-6 and 1e-12 ε rows. The ladder is
  re-baselined: ε/10 still escalates, and ε/16 is the floor.

Closed with PR 3967, 2026-10-03.

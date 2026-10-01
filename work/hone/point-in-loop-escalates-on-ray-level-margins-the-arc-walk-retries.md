---
id: point-in-loop-escalates-on-ray-level-margins-the-arc-walk-retries
kind: issue
title: point_in_loop escalates on ray-level margins (side, advance, arm) that point_in_carrier_loop's arc path retries, so a polygon outer loop refuses where the same loop with an arc answers
status: open
opened: 2026-09-27
priority: P2
cost: M
refs: [3288]
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
(`boolean::contfp`, `chord_join::rehome_rings`, the solid-containment
sweep, check 9), and ATREST-12 kept its escalation stream frozen for
them. Changing it moves each consumer's refusal surface, so it needs
its own measurement. The fix is small: pass `ArmBand::Retry` and map
`ray_verdict`'s error to a graze, the way the arc path does.


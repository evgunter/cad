---
id: main-red-two-sweep-rows-at-sym-and-the-certified-scalar-since-limb-2
kind: issue
title: Main red: two sweep rows (sym11 far placement at Sym<f64>, germarms2 meeting axes at the certified scalar) fail on main after PR 5728; bisect and fix
status: closed
opened: 2026-10-27
priority: P1
cost: M
closed: 2026-10-28
pr: 5893
---


Reported by the edge-extent lane (PR 5875, 2026-10-27). Both rows fail the
same way on its merge base `1483d37ded`, and no `work/` row records them:

- `sweep` `sym11_far_placement_rows::sym11_the_far_placement_is_a_counted_dispute_at_sym_f64`
- `sweep` `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`

`1483d37ded` follows PR 5728's merge (`b8974faabc`, limb 2 for every
carrier and the interval-box identity), and both rows touch limb 2's
territory: a far placement, and a certified-scalar build. So OFFSET owns
this until a bisect says otherwise.

**Owed:**
- bisect on main, using `b8974faabc^1` and `b8974faabc`;
- read each failure;
- fix it if it is OFFSET's, or hand it to its owner with the bisect.

## Closed

Both rows are red on `origin/main` (`96bc411481`); they have two
different causes, and only one is OFFSET's.

**`sym11_far_placement_rows::sym11_the_far_placement_is_a_counted_dispute_at_sym_f64`
— PR 5728, an honest move, re-baselined.** Green at `b8974faabc^1`,
red at `b8974faabc`. Exactly one cell of `TABLE` moved, at all three
ε rows and all three lanes (mapped with the asserts printing rather
than panicking): at `(1e-9, 3.7e7)` the `Sym<f64>` triangle refuses
`AnalyticRung3(Escalated { operand: Plane, limb: HullValue })` on a
measured end distance of `1.178e-9`, where it built. Limb 2 now
certifies the extrude's side-wall lines against their planes, and C2
(`crates/geom-brep/README.md`) decides a line against a plane on its
two ends' distances measured in certification arithmetic at every
scalar, escalating when an end's floor is above zero and inside the
ambiguity band. `Sym<f64>`'s witness is inexact, so it is a point
scalar (`crates/profile/README.md`): the free-`t` identity is read only
at `Witness::Exact` (`edge_nurbs::limb2`), and the ends are the stored
`f64` data, which rounding at `3.7e7` leaves `1.18e-9` off the plane.
The bare lift refuses the same triangle there too. The cell now reads
`Rung3Escalated(HullValue)`; `outcome` names a rung-3 refusal by its
variant and limb, since it carries no `check:`.

**`verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`
— not OFFSET's: PR 5664 (CHART).** Already red at `b8974faabc^1`. A
first-parent bisect from green `8b15425279` lands on `80a6f77af3`
(PR 5664, `chart/ringed-curved-doors`). Handed to CHARTTAIL, CHART's
successor, with the bisect:
`work/charttail/main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664.md`.


---
id: main-red-two-sweep-rows-at-sym-and-the-certified-scalar-since-limb-2
kind: issue
title: Main red: two sweep rows (sym11 far placement at Sym<f64>, germarms2 meeting axes at the certified scalar) fail on main after PR 5728; bisect and fix
status: open
opened: 2026-10-27
priority: P1
cost: M
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

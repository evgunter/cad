---
id: main-reds-on-plates-at-the-exact-support-of-a-frustum-and-a-torus
kind: issue
title: Main is red on sweep operand_gate_support_plates::plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op (touching operands built as disjoint)
status: dispatched
opened: 2026-10-11
priority: P0
cost: M
branch: cleave/support-plates-main-red
---


Filed by the NURBS orchestrator from build B's validation of PR 4684.
This is not NURBS's ground. It is filed on GERM's slate because GERM
last shaped the row (`e94ecd7ec4`, `889c4115bc`, PR 4418). The test
file itself is TCOST/TINT territory.

## What

`sweep::operand_gate_support_plates::plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op`
fails: "touching operands built as disjoint". It fails on pristine
`origin/main` `6e3985cb76`, freshly compiled, so no branch's change
is involved. The test file has not changed since 2026-10-09, which
means a kernel change on main moved the verdict.

## Not done

Not bisected. Main pushes skip the test job, so the per-PR gate shows
this red only on PRs that select the sweep suite.

Owed:
- bisect it on main;
- decide whether the touching verdict is right (and the row stale) or
  the kernel regressed.

## Dispatched (CLEAVE, 2026-10-11)

It blocks two CLEAVE PRs (4625, 4630), so CLEAVE's orchestrator has
taken it: lane `cleave-plates` on branch `cleave/support-plates-main-red`
bisects across the merge range PR 4681 names (#4667, #4598, #4597, #4636,
#4573, #4604, #4671) and fixes at the owning decision. The same red is
also filed as `contacthold/a-plate-at-a-frustums-support-unions-into-two-disjoint-lumps`
in PR 4681 (not yet merged); this row is the one being worked.

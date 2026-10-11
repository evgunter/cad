---
id: main-reds-on-plates-at-the-exact-support-of-a-frustum-and-a-torus
kind: issue
title: Main is red on sweep operand_gate_support_plates::plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op (touching operands built as disjoint)
status: open
opened: 2026-10-11
priority: P0
cost: M
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

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

## Built (branch cleave/support-plates-main-red)

The test's expectation was stale. The kernel was right.

- **Bisect.** Each step was a fresh worktree on a wiped target dir.
  `bf05e073dd` (the #4573 merge) passes: built 410, nothing wrong.
  `758bd7acb4` (the #4604 merge) fails: built 510, 80 wrong rows, the
  same as main. The commit is `08ba4968fa` ("carry records every
  non-incident pair; CurvedRestUnrecorded retires").
- **Cause.** Every frustum support is a point of a rim circle. Before
  #4604, ∪ at `δ ≤ 0` off the 270° cut refused with
  `CurvedRestUnrecorded`, which was a right answer under the row. Since
  #4604 it builds an `Assembly` of 2 solids holding one
  `EdgeFace(rim Circle, plate Plane)` record. That is D1's answer for a
  point touch. The row's exclusion had been narrowed to the frustum's ∩
  (`889c4115bc`) while ∪ still refused.
- **Closed form.** At `δ = 0` the plate's face lies on the support
  plane. The rim circle meets that plane only at the support point,
  which is the centre of the plate's face. No vertex sits there: the
  nearest is the plate's corner at 0.21·s.
- **Change.** `excluded = cut_touch || shape == Shape::Frustum`, plus
  the module doc. An excluded ∪ that records no contact is still wrong.
- **Measured** on a wiped target, green at ε 1e-9, 1e-6 and 1e-12
  (built 510, 405 and 440). A mutant that drops `EdgeFace` records turns
  the row red at 90 rows (ε 1e-9).

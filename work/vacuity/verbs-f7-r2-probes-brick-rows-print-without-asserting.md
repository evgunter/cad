---
id: verbs-f7-r2-probes-brick-rows-print-without-asserting
kind: issue
title: verbs_f7_r2_probes' two brick-operand rows print their boolean outcome and assert nothing
status: open
opened: 2026-09-19
priority: P3
cost: E
---

## Finding

- **Where**: `crates/sweep/tests/verbs_f7_r2_probes.rs`,
  `r2_cone_carries_the_pole_split_cap` and
  `r2_one_face_cap_via_kef_then_kev` — both build `brick_operand()`,
  hand it to `boolean_reduce`, and `println!` the outcome.
- **Importance**: medium — two rows in a review-probe file that cannot
  report a regression
- **Confidence**: sure, and measured rather than read
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19

Both rows take the `Ok`/`Err` of the union and print it. Neither
asserts on it, so the F7 door can open, shut or change its typed
refusal and both rows stay green.

**Measured, not read.** The lane folded `brick_operand` onto
`sweep::test_support::brick` and then planted three mutations in
`topo::test_support::brick` — `z.1 + 0.001`, the x extent halved, and
the whole box translated `+10` in x. Every other suite the lane folded
went red under at least one of the three;
`verbs_f7_r2_probes` stayed green under **all three**, which is what
told the lane the operand is not asserted on anywhere.

The file's third row (`the pole positive`) does assert, and does not
use `brick_operand`. So this is two rows of a three-row file, not the
file.

S-TINT's charter, not S-DUP's: a row that cannot go red is a coverage
defect. It is filed from a duplication unit because the duplication
unit's mutation instrument is what found it.

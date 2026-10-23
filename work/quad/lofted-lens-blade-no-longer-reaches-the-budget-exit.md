---
id: lofted-lens-blade-no-longer-reaches-the-budget-exit
kind: issue
title: the lofted lens blade certifies at the default eps, so its budget-exit rows test nothing
status: open
opened: 2026-10-22
refs: [5582, 5568]
priority: P1
cost: M
design: true
---

Two slow-set rows in
`crates/sweep/tests/a_lofted_lens_blade_certifies_its_sign.rs` are red
on main at the default ε:

- `a_lofted_lens_blade_certifies_its_sign_past_the_budget_exit`
- `the_volume_backstop_runs_no_round_past_the_budget_exit`

Both assert that the reporting door refuses the blade at round 0's
budget exit (`QuadratureBudget { rounds: 1, .. }`). It now certifies
instead: `Ok(MassProperties { volume: 0.034605949150085524,
volume_pad: 4.641116114016224e-7, .. })`.

Bisected: passes at `aaef9f58` (PR 5580), fails at `9d680e33`
(PR 5582, the rational quotient statement). That PR's `f_uu` block
hulls are much narrower on rational walls (12× on the arc-prism wall,
`rational-wall-pad-pins-trail-the-quotient-statement`), so the
blade's walls now meet `1024·ε`. The rows' premise, that no round of
the schedule can reach the target, no longer holds.

The kernel change is an improvement, not a defect. The defect is that
the two rows have lost their subject: Check 7 deciding a sign past a
budget-exit refusal, and the backstop paying only round 0's readings
for a refused face. They need a fixture that still refuses at round 0
at the default ε (a harsher blade, or a different refusing body),
weighed against the PR 5568 intent. Or, if no natural body refuses
any more, re-scoped.

The row also got slower: `a_lofted_lens_blade_certifies_its_sign_past_the_budget_exit`
ran in 68 s at `aaef9f58` and runs in 250 s at `9d680e33` (local,
dev profile), because the walls now refine to convergence.

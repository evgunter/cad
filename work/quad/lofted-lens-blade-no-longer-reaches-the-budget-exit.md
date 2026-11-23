---
id: lofted-lens-blade-no-longer-reaches-the-budget-exit
kind: issue
title: the lofted lens blade certifies at the default eps, so its budget-exit rows test nothing
status: closed
parent: the-measurement-door-refines-until-a-predicate-resolves
pr: 5589
opened: 2026-10-22
closed: 2026-10-22
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

**Closed by PR 5589** (the measure door's unit A). The budget exit is
gone, so the rows' premise is gone with it. Ruled: the subject stays
"a sign that settles while the number is still open".
- The blade's row now asserts that it certifies, number and sign, with
  main's bits: `0.034605949150085524 ± 4.64e-7` at 1e-9. The measured
  work wall gives the knot-cut walls their round 7, which the
  plain-rectangle wall had cut to round 6.
- The open number moves to an ε-scaled round tube (`round_tube`, the
  python suite's `oversized_round_loft`). Its gate admits the sign in
  about 6 s, and its walls stop at the work wall after 11 rounds.
- The backstop's count row moves to the tube as well:
  `the_volume_backstop_reads_a_refused_face_once_per_body`.

**2026-10-30 (QUAD per-span lane, PR 6092 review).** The reviewers read `the_volume_backstop_reads_a_refused_face_once_per_body` red at 1e-6 and 1e-12 (main 50 against 100, the head 94 against 64). Under nextest, one test per process, it passes at 1e-9, 1e-6 and 1e-12 on main (`round_tube` at `s = 64`) and on the head (`s = 512`), release, and main's nightly 41008276467 does not list it among its failures. Its counts come from `geom_brep::props::quad::rounds_run_by`, a process-wide counter whose doc says a reading is one test's only where that test has its process to itself; the likeliest reading of the red counts is a runner that shared a process between tests (`cargo test`'s threads), which adds other tests' rounds to either side. Not reproduced that way here; nothing to fix in the row under nextest.

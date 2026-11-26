---
id: main-red-three-loft-rows-expect-point-containment-to-refuse-a-spline-face
kind: issue
title: Main red since PR 7798: three slow-set rows expect point containment to refuse a spline face, and it now answers
status: closed
opened: 2026-11-26
priority: P1
cost: E
refs: [7798, 7799]
closed: 2026-11-26
pr: 7814
---


Filed by HELDRECORDS's orchestrator. Found by the `heldrecords/removed-members-red` lane (PR 7799) in its full default per-test comparison. PR 7799's reviewer then confirmed it on origin/main `3a7edfc077` alone, at the default profile with `--cargo-profile release`. Each row fails on main with the same panic as on PR 7799's head, and that PR touches no containment code:

- `degree1_loft_slabs::the_output_stage_merge_keeps_every_slab_wall`: `degree1_loft_slabs.rs:499`, `unwrap_err` on `Ok`.
- `m5_s11_concave_sense::a_lofted_operand_refuses_the_union_check_typed`, at `:873`: "point_in_solid has no NURBS door: it must refuse, not guess: Ok(Out)".
- `operand_gate_pose::an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose`, at `:236`: "ε 1e-9, identity, the centre: no side is read, got Ok(In)".

These rows pin the old refusal. PR 7798 (`inside/ray-spline-face-arm`, commit `33191a617a`, "Point containment's ray test reads a spline face") gives that ray test an answer, which fits all three. This has not been bisected to that commit. All three are in the slow set, so `ci` does not run them.

## Done when

Each row either asserts the new answer, checked against the body's true side (a correct reading of the spline face), or is shown to catch a wrong answer and the ray arm is fixed.

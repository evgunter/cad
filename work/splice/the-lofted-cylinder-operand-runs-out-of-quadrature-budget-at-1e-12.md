---
id: the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12
kind: issue
title: the lofted-cylinder operand of spline_spiric_operands runs out of quadrature budget at 1e-12
status: open
opened: 2026-10-26
priority: P2
cost: M
---


Filed by RESTREAD (PR 5704's local sweep run at `CAD_TOLERANCE_EPS=1e-12`), outside its fence.

## What

`spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong`
(`crates/sweep/tests/spline_spiric_operands.rs:397`) panics at 1e-12 in
`census` (`crates/sweep/tests/spline_spiric_operands.rs:193-195`),
measuring the bare lofted cylinder before any op runs
(`lofted_cylinder_census`, `:336-340`):

    the operand measures: Face { face: FaceKey(3v1), source: QuadratureBudget {
      width_len: 1.7011796697323247e-9, target_len: 1.024e-9, rounds: 11 } }

## Why it is main's

- The row came in with main's merge at PR 5704's 173271c. It is in the sweep slow set, so a PR run does not reach it.
- The failure is `topo::mass_properties` on the operand alone, built by `lofted_cylinder`. No boolean and no census has run yet. PR 5704 touches only the census.
- `work/splice/delete-the-boolean-operand-edge-gate.md` says the census was measured at 1e-12. This run says the operand does not measure there.
- Not run on main itself.

## Fix

Either the operand's quadrature reaches the 1e-12 target, or the row takes the operand's volume from its closed form (`2π`) at ε rows where the quadrature refuses typed, saying so.

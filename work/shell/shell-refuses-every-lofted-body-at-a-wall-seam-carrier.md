---
id: shell-refuses-every-lofted-body-at-a-wall-seam-carrier
kind: issue
title: topo::shell refuses every lofted body re-anchoring a spline wall seam at a cap corner, so no NURBS-walled body reaches the offset fit through shell
status: open
opened: 2026-09-25
---


Filed by ENCL's `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`
lane, whose Part 2 was to take the cost of shelling a NURBS-walled body
and found that no such body shells at any ε.

## Measured

`topo::shell(&body, 0.05, Tol::witness())` on three bodies lofted
through `sweep::loft_body`, at the default ε:

| body | faces | refusal | wall time |
|---|---|---|---|
| square prism (planar spline walls) | 2 plane, 4 nurbs | `ShellError::Face` on a cap, `CarrierLaneUnsupported` "a re-anchored carrier that is neither a line nor a circle" | < 1 ms |
| twisted loft, 0.3 rad (bilinear saddle walls) | 2 plane, 4 nurbs | the same | < 1 ms |
| circular vase, three circle sections, degree 2 | 2 plane, 2 nurbs | the same | < 1 ms |

The refusal is raised in `crates/topo/src/replace_face.rs`,
`plan_reanchors`, which walks the edges NOT on the replaced face's
boundary whose endpoints the offset moved. On a lofted body those are
the vertical wall-to-wall seams that end at a cap's corners, and their
carriers are lofted `Curve3::Nurbs`. The refused edge on the twisted
loft is a seam between two spline walls, asserted by
`crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_curved_loft_refuses_at_a_wall_seam_before_any_fit`.

So the shell verb never reaches a wall's offset fit on a lofted body,
and the wall-side refusals behind it (the fit's reach at the shell
thickness, then O4's fitted-boundary refusal) are unreachable through
it. Per face, `topo::replace_face_offset` on the same bodies at
`d = ±0.05`:

- the caps refuse as above;
- the twisted loft's saddle walls reach the fit, which refuses at the
  default ε (`BudgetExhausted`, achieved 4.14e-9, ~0.3 s per wall);
  at 1e-6 the fit certifies and the door refuses
  `FittedBoundaryUnsupported`;
- the vase's walls refuse at the fit's C⁰-crease gate (`PatchBound`:
  "NURBS direction with a C⁰ crease (interior multiplicity =
  degree)"), which is ENCL's ground, not this row's.

## What it blocks

The cost measurement ENCL's item asks for (a shell of a spline-walled
body at the default ε, mint plus tier 3's per-call re-derivation) has
no operand until this lane exists AND O4's fitted-boundary refusal is
answered. `encl_curved_loft_shell.rs` pins both boundaries so that
either moving reds and hands the next lane the operand.

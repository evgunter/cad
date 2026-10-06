---
id: arc-loft-native-volume-exhausts-the-quadrature-budget-at-eps-1e-12
kind: issue
title: nurbs_import's arc loft exhausts the quadrature budget at eps 1e-12 on main
status: closed
closed: 2026-10-06
branch: emit/nightly-eps-stale-pins
opened: 2026-10-03
---


## What

`step-import`'s slow-set row
`nurbs_import::arc_loft_natively_computes_its_rational_volume`
(`crates/step-import/tests/nurbs_import.rs`, the assertion "the only
surviving refusal is the quadrature budget, with its number") fails at
`CAD_TOLERANCE_EPS=1e-12`, the nightly's third eps row. The refusal is the
quadrature budget: "a face's certified contribution stayed 3.626e-7 m
wide after 1 refinement round(s), above the 1.024e-9 m this tolerance
targets".

Measured on `origin/main` 82b9ceb2 under the nightly's profile (test
opt-level 2), and identically on JOIN-3's merged head, so it is main's
and not a branch's. It is a red nightly row; the default and 1e-6 rows
pass.

Neighbour, same shape (a body whose mass exhausts the budget at 1e-12):
`work/show/lily-leaf-b-mass-exhausts-the-quadrature-budget-at-eps-1e-12.md`.

## Closed (2026-10-06, EMIT, branch `emit/nightly-eps-stale-pins`)

The budget refusal at 1e-12 is the expected outcome. All of the test's
typed checks pass: `PropsError::QuadratureBudget`, the bracket is kept,
and `volume_lo > 0`. Only a prose substring failed. PR 3942 (63df2069ce)
reworded the `QuadratureBudget` message in `geom-brep/src/props/mod.rs`,
and the pin at `step-import/tests/nurbs_import.rs:346` still quoted the
old wording.

`r1_dm1_probe.rs:148` held the same old string in a `!contains` check, so
since that PR it asserted nothing. Both now quote the current wording
("this tolerance targets"). They pass at 1e-9, 1e-6 and 1e-12.

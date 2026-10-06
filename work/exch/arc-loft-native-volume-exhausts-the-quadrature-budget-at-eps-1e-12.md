---
id: arc-loft-native-volume-exhausts-the-quadrature-budget-at-eps-1e-12
kind: issue
title: nurbs_import's arc loft exhausts the quadrature budget at eps 1e-12 on main
status: open
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

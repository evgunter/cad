---
id: r1-pxn-envelope-ceiling-red-since-convex-insert
kind: issue
title: r1_pxn_probes' envelope ceiling is red on main since PR 3524's convex insert_once_ring (1.0068x against 1.005x)
status: closed
opened: 2026-10-01
closed: 2026-10-02
pr: 3733
---


Found by `pcert/general-circle-fitted-route` (PR 3733), whose gate runs
`geom-brep`'s slow set because it touches the crate.

`geom-brep`'s `r1_pxn_probes::the_certified_sup_bounds_the_dense_sampled_true_sup`
(the ratio arm, `crates/geom-brep/tests/r1_pxn_probes.rs`, the
`claim.within(1.005, …)` call) fails on `main`, deterministically:

```text
CEILING: plane x NURBS envelope: certified 1.0068509641344932e-12 against a
sampled truth of 1.000088900582341e-12, a ratio of 1.0067614624542025 —
over this row's ceiling of 1.005x
```

A first-parent bisect of `main` (good `ca1ae6619`, bad `origin/main` at
`83312dc24`, the row as its oracle) names the merge of PR 3524
(`ba06ed4bf`, "PROPS: the convex form in insert_once_ring, with beta
derived from the knots") as the first bad commit. The convex insertion
form moves the knot-refined control data the tensor composite hulls, and
the `a = 1e-12` rung's envelope grows from inside the row's 1.005x ceiling
to 1.0068x. The row's own text says the ceiling exists to catch "the
envelope is no longer residual-scaled".

Owed: decide whether 1.0068x is the convex form's honest number (then
re-baseline the ceiling, saying what moved) or a regression in the
insertion's tightness (then fix the insert). PR 3524's gate did not run
this row, presumably because it is in the slow set of a crate that PR did
not touch.

## Closed

Green on `main` since PR 3737's fix pass (`c97bac01e`, "r1 ceiling
spelling", after `84557c784` met the Boehm convex step with its sources'
hull): the row passes on `origin/main` `c8e6aec14` and on PR 3733's
merged head. Closed by PR 3733, which filed it.

---
id: pxn-ratio-ceiling-reds-main-at-default-and-1e-6
kind: issue
title: r1_pxn_probes' new 1.005 ratio ceiling reds main: the certified sup reads 1.00676x the sampled truth at default eps and 1e-6
status: closed
opened: 2026-10-01
priority: P0
cost: E
closed: 2026-10-01
pr: 3737
branch: reach/pxn-envelope-red
---


## What

`crates/geom-brep/tests/r1_pxn_probes.rs`,
`the_certified_sup_bounds_the_dense_sampled_true_sup`, fails on `main`
at the default ε and at 1e-6. It passes at 1e-12.

- The failing check is its ratio arm's `claim.within(1.005, 0.0, …)`.
- The certified envelope reads **1.00676×** the sampled truth.
- At a = 1e-12 it certifies 1.00685e-12.

The 1.005 ceiling arrived in `9342403cc` ("SSI: fix pass — … ceilings
that see the fold", 2026-10-01). Its residual-bound change is the
preceding commit, `78f5d765b`.

## Who measured it

Two LINALG lanes reproduced it independently, on a clean `origin/main`
export (`597086f45`), with identical digits:
- #3725 (`linalg/adopt-doors`);
- #3727 (`linalg/certification-sqrt`).

Reverting each of #3727's changes one at a time does not move the row.
SSI's own #3730 is red in its `test` job as well.

## What is owed

This red is SSI's to fix. Either the envelope is coarser than the
ceiling's derivation assumed, and the ceiling's number is wrong; or the
fold the ceiling was written to see is not reaching this fixture.

Until it is fixed, every PR that seeds `geom-brep` reds in its eps
steps, and has to be merged over an inherited red. The slow-set step
still runs (it is gated on `!cancelled()`, not on the earlier steps).

Filed by the LINALG orchestrator.

## Closed

PR 3737 (`reach/pxn-envelope-red`), REACH's main-red lane.

**Root cause: a semantic merge conflict between #3524 and #3685, not
the ceiling's derivation.** Bisected first-parent on main: last good
`fe2447a440`, first bad `ba06ed4bf3` (#3524's merge). #3524 moved the
certified Boehm step from the lerp form `x + (y − x)·α` to the convex
form `β·x + α·y`, with `α` and `β` each rounded outward, so
`α_hi + β_hi > 1` and a constant column came out of every insertion as
a bracket around its value. On this fixture that bracket lands on the
weight channels (`W ≡ 1` on the carrier, the wall's weights constant
along `v`). #3685's limb-2 numerator `N_d·W_C − A_d·N_w` cancels
against them, so their width became residual width.

**The fix:** `algebra::convex_step` meets the convex step with the hull
of its two sources (`Certification::meet`), sound because the true
ratio lies in `[0, 1]`. A constant column stays its point. The row
reads 1.00166 at a = 1e-12, down from 1.00676, and its ceilings are
re-tightened to 1.003 and an additive floor of 2.5e-14.

**Not all of #3524's loosening is the constant columns.** Before #3524
the row read 1.000681 at a = 1e-12. The convex form on columns that
are not constant is the rest, and it is filed with its owner:
`work/props/the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column.md`.

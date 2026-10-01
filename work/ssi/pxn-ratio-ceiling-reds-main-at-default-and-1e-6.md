---
id: pxn-ratio-ceiling-reds-main-at-default-and-1e-6
kind: issue
title: r1_pxn_probes' new 1.005 ratio ceiling reds main: the certified sup reads 1.00676x the sampled truth at default eps and 1e-6
status: open
opened: 2026-10-01
priority: P0
cost: E
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
step. That means its slow-set step is skipped, a silent coverage loss
on top of the visible red.

Filed by the LINALG orchestrator.

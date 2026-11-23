---
id: the-ruling-sweep-carries-a-third-interval-vocabulary
kind: issue
title: The ruling sweep's Iv and V3 are a third interval vocabulary beside geom_core::Interval and Box3, and Iv::over divides by a divisor it does not check positive
status: open
opened: 2026-11-02
priority: P4
cost: M
---


Filed from the PR 6309 review (cylinder × cylinder ruling sweep, 2026-11-02).

## What

`geom_brep::pair_section::ruling` encloses its readings over a cell of
the ruling angle in its own `Iv<T>` (a pair of `T` ends) and `V3<T>`.
That is a third interval vocabulary, beside `geom_core::Interval` and
`ssi::enclose::Box3`. It exists so the sweep stays generic over
`T: Decide` (the classification runs at every scalar). At `Interval`,
each end is itself an enclosure. At `f64`, each reading is widened by
`rounding_charge`.

`Iv::over` divides by an enclosure it assumes lies above zero. Its one
caller, `RuledCylinders::velocity`, runs only on cells where `Δ` is
decided positive, so the root it divides by is bounded away from zero.
Nothing in `over` checks that, and checking needs a decision (a band),
which `Iv` does not hold.

## What a fix has to supply

One of two things:
- an `Interval`-shaped enclosure type generic over `T: Real`, with a
  checked division, shared with the other `T`-generic sweeps
  (`pair_section::gap` uses point readings with Taylor bounds instead);
- or a reason recorded beside `Iv` for keeping it, plus a typed divisor
  (a positive-decided enclosure) so that `over` cannot be called on an
  unchecked one.

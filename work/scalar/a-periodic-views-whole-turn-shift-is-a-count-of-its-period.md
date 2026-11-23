---
id: a-periodic-views-whole-turn-shift-is-a-count-of-its-period
kind: issue
title: A periodic view's whole-turn shift is a count of its period, built at T
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by `a-place-is-an-f64-argument-answered-on-the-value-channel`, which the unit file scoped `geom/src/curves/periodic_nurbs.rs` into; it is not a place.

`PeriodicNurbs3::restated` and `PeriodicNurbs3::reduced` take `T::from_computed(shift)`, where `shift` is a whole number of turns times the period computed in `f64` (`Period::at_or_above` on the value channel). That is a closed form of the period the curve holds at `T`: `self.period` times the count, the count an integer. As written, at a dual the shifted knots (`restated`, a curve the view returns) and the data parameter (`reduced`, evaluated at) carry none of the period's tangent. `restated`'s ends `s0`, `s1` (two more calls) are the asked span's ends, stored in the returned curve: a solved-or-asked value, not a place. At `f64` the count form may differ from the computed shift in the last bit, so a fix states what moves.

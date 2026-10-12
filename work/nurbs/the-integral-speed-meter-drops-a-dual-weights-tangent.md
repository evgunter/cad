---
id: the-integral-speed-meter-drops-a-dual-weights-tangent
kind: issue
title: The integral speed meter reads only the net, so at a Dual or Sym scalar it drops what a held weight carries beyond its value when the weights' values are equal
status: open
opened: 2026-10-12
priority: P4
cost: M
---

## What

`NurbsCurve3::speed_lower_bound` (`crates/geom/src/curves/nurbs.rs`)
chooses its integral arm when the weights' VALUE channel is constant,
and that arm reads only `control` and the knots. At `f64` and
`Interval` a weight is a point, so equal values are equal weights and
the arm is exact. At `Dual`, `SpanLocate::point_value` is the value
alone: held weights with equal values but different tangents describe a
curve whose derivative channel depends on them, and the integral arm
drops that dependence, so the meter's tangent is wrong.

`Sym` has the same gap: `Sym::point_value` (`crates/geom-core/src/sym.rs`,
the `SpanLocate for Sym<T>` impl) returns its value's point alone, so
equal-valued weights on different symbols take the integral arm and
the meter loses their symbolic dependence (found by PR 4804's FULL
review).

This held before for weights of value `1.0` exactly; the
rational-speed-meter PR widened the gate from "all `1.0`" to "all
equal", which widens the case to any equal value.

Found by the NURBS sweep of "has weights" read as "is rational".
`nurbs_iso` handles the same shape by deciding on the value channel and
then reading the held weights
(`the_collapsed_row_carries_a_held_weights_tangent`). No caller is
known to meter a carrier at `Dual` or `Sym`; if none does, the fix is a stated
restriction rather than code.

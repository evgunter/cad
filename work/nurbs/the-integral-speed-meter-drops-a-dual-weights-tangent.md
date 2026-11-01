---
id: the-integral-speed-meter-drops-a-dual-weights-tangent
kind: issue
title: The integral speed meter reads only the net, so at a Dual or Sym scalar it drops what a held weight carries beyond its value when the weights' values are equal
status: closed
opened: 2026-10-12
priority: P4
cost: M
branch: nurbs/held-constant-weights
closed: 2026-10-12
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
(`the_collapsed_row_carries_a_held_weights_tangent`). Carriers ARE
metered at `Sym`: the sweep thread-count digest's `Sym` rows move with
the meter (they moved under PR 4803's memo), so this is code to fix,
not a restriction to state.

## Closed (2026-10-12)

`SpanLocate` gains `plain_constant(self) -> Option<f64>`
(`crates/geom-core/src/spline/locate.rs`), declared at every scalar:
`f64` and a point `Interval` answer their value; `Dual` only with a
derivative of exactly zero; `Sym` only when its node is the literal of
its value (the `Lit` node's content hash, recomputed, so nothing is
interned); `Probe` as `f64`. `speed_lower_bound` takes the integral arm
only when every weight is the same plain constant, and the rational arm
otherwise. Rows, each red under the value-channel choice:
`a_dual_weight_with_a_tangent_takes_the_rational_arm` and
`a_computed_sym_weight_takes_the_rational_arm` (`nurbs.rs` tests), with
`plain_constant_reads_only_what_is_carried_as_a_constant` (`locate.rs`
tests) on each scalar.

The class this belongs to is INLANE's
`work/inlane/held-value-equalities-decided-on-the-value-channel.md`,
which prescribes a `T`-margin primitive that `Sym` discharges;
`plain_constant` is the conservative member and stands until that
primitive lands or subsumes it (noted on that row).

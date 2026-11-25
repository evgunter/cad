---
id: a-value-view-door-may-return-a-number-and-spline-walls-feet-do
kind: issue
title: Code a dual reaches through its value view may return a number: SplineWall's chart feet re-enter T through from_computed
status: open
opened: 2026-11-25
priority: P1
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


From the design fork ruled on `[ev]` PR 7719 (2026-11-25), after Ev asked whether handing a dual's base value to other arithmetic is sound ("this kind of thing is constantly sneaking in semantic errors into the dual code"). DL5.3 now says: "Code a dual reaches through its value view returns a selection and nothing else, by its return type: no `T` comes back, since a number solved on the value and re-entered as a `T` would claim a zero tangent."

Both designers audited every non-test caller of `value_view()`, `without_tangent()` and `value_channel()` by reading (nothing run). Selections, and right: `spline_incidence.rs`'s doors; `topo` `spline_trim.rs`, `spline_region.rs`; `pcurves_rings.rs` (an integer deck shift; inside `side`, a window half-width is entered as a `T` by `from_computed` to read arms over: a place held as a `T`); `sweep/src/skin.rs` (a sort order); `geom/src/curves/fit.rs` (a grouping by value equality); `geom/src/curves/banded.rs` (error payloads). Not classified: `geom/src/curves.rs` `clamped_view((t0.value_channel(), t1.value_channel()))`, a net cut at `f64` parameters, argued right to first order and unverified.

**The live instance:** `crates/topo/src/ring_path/spline_wall.rs`. Feet are found on an `f64` shadow chart, then entered with `T::from_computed(u)` into `outward` normals and the `piece` iso curves, so `IsoPiece<T>` and `Path<T>` exist as `T` values with a frozen foot. Today they are only decided on (path parity to a `RingSide` in `chord_join.rs`; the side of a normal), so no wrong tangent is live; any consumer that stored one would be silently wrong. Not every reader of `Path` was traced.

Asks: (1) `SplineWall`'s feet become `f64` places (DL5.2's place row), and what is built on them returns a selection; (2) the rule held by type: a sealed bound on what a value-view door may return (`Sign`, a side, `bool`, integers, plain enums), or a gate that reds on a `T` in such a return; (3) classify `clamped_view` and the `pcurves_rings` half-width.

---
id: hand-chebyshev-chains-are-not-yet-on-norm-inf
kind: issue
title: hand max-abs (Chebyshev) chains outside geom-core's linalg are not yet on Vec2/Vec3::norm_inf, and their f64::max folds drop a NaN coordinate
status: closed
opened: 2026-10-01
priority: P4
cost: E
closed: 2026-10-01
---

The residue of `point3-has-no-order-and-vec3-no-sup-norm-door`'s
max-abs half. `linalg/doors` (PR 3710) landed `Vec2`/`Vec3::norm_inf`
(the L∞ norm, folded through `Real::max`, so a NaN component poisons
it) and moved the in-fence chains onto it (`linalg/vec.rs`'s test
`max_abs3`, two `mat.rs` test bounds, `editor-core`'s
`fixture/seat.rs` `map_gap`, `demos/tour` `diechamfer.rs`). The chains
below are other programs' ground and stay as they are until a lane
that owns them adopts the door.

`norm_inf` is not `geom_core::interval::norm_sup`, which is a
certified upper bound on the Euclidean norm. None of the sites below
wants that bound, and none should be respelled toward it.

## The sites

- `crates/profile/src/seg.rs` `reach` (around :588): a live,
  non-test max-abs magnitude of a `Point2`. A point has no norm, so
  the door spelling is `(p - Point2::origin()).norm_inf()`, or a
  `Vec2` read of the point. Whether that reads better than the hand
  chain is the owner's call. Generic over `Real`, so its `max` already
  propagates NaN, and the move is spelling only.
- `crates/geom/tests/curves/review_m5_pr3_attack.rs` `sup3`, `supp`
  (around :68, :85), `crates/geom/tests/surfaces/review_m5_pr3_attack.rs`
  `supp`, `supv` (around :58, :65), and the inline chain in
  `crates/geom/tests/curves/review_m5_pr3_attack_interval.rs` (around
  :215).
- `crates/sweep/tests/common/mod.rs` `sup_dist` (around :539).
- `crates/sweep/tests/verbs_chamfer.rs`, the proximity filter (around
  :197).

## Why it is more than spelling in the tests

Every test-side copy is concrete `f64` and folds through the inherent
`f64::max`, which drops a NaN operand. A gap with one NaN coordinate
therefore reads as the max of the other two, and a `<= tol` gate on it
passes. `norm_inf` would make that gap NaN and fail the gate. Moving a
site onto the door tightens it, and a row that goes red on the move has
found a NaN it was passing.

## Closed (2026-10-01)

Closed by #3725. All 8 named sites moved, plus 6 siblings.

The NaN semantics changed. A NaN gap used to pass `<= tol` whenever one of its coordinates was finite. It now fails every gate. The differential shows the old chain passing 227,517 NaN-gap rows that the new one refuses, and nothing else differs. No assertion was weakened.

Filed: `work/tint/inverse-round-trip-row-drops-a-nan-entry-through-f64-max.md`.

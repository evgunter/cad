---
id: inverse-round-trip-row-drops-a-nan-entry-through-f64-max
kind: issue
title: geom-core's inverse round-trip row folds the deviation from I through f64::max, so a NaN entry of m·m⁻¹ reads as no deviation
status: open
opened: 2026-10-01
---


Found by `linalg/adopt-doors` while sweeping for hand max-abs chains
(the `hand-chebyshev-chains-are-not-yet-on-norm-inf` item's shape) one
size up, over a matrix.

`crates/geom-core/tests/review_m0_pr6.rs`,
`inverse_and_determinant_spot_checks` (around :317): the behavioural
check `m * m.inverse() ~ I` lists the nine `|p − I|` entries and folds
them with `.fold(0.0f64, f64::max)`, then asserts `idmax < 1e-15`. The
inherent `f64::max` drops a NaN operand, so an `m · m⁻¹` with a NaN
entry (and at least one finite one) reads as the largest finite
deviation and passes; only an all-NaN product fails. The same suite's
`f64` helpers for `Vec3` moved onto `Vec3::norm_inf`, which poisons;
there is no `Mat3` max-abs door, so this row stays a hand fold.

The repair is any fold that propagates NaN — `Real::max`, or the
columns of `p − I` through `Mat3::cols` and `Vec3::norm_inf` folded
the same way — so that a NaN entry fails `< 1e-15`. The fixture `m`
is fixed and finite, so the row is green today; what it cannot do is
go red on the NaN it is meant to catch.

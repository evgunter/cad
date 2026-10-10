---
id: second-derivative-sup-takes-a-control-net-beside-its-vector
kind: issue
title: nonrational_second_derivative_sup takes a control net beside its knot vector and re-mints its differenced level
status: open
opened: 2026-10-10
---


Found by the `coefficient-vector-pairing-survivors` sweep (NURBS).

`crates/geom/src/curves/second_derivative.rs`
`nonrational_second_derivative_sup(knots, control: &[Point3<f64>])` is
a public door taking a control net beside its vector, related by count;
inside, it differences each channel with `knots.difference_coeffs`,
builds the derivative vector itself (`KnotVector::clamped(
derivative_knot_slice, p − 1)`), and re-mints the level with
`kv1.with_coeffs(&q1).map_or_else(Interval::refused, …)` — a refusal
arm dead by construction.

Disposition: take the curve (or a pair minted by its caller), build each
channel with `KnotVector::with_coeffs_from_fn`, and read the second
level through `SplineCoeffs::derivative` → `SplineCoeffsBuf::pair` →
`derivative_domain_hull`; `KnotVector::derivative` is the vector it now
builds by hand. `KnotVector::difference_coeffs` then has no consumer
outside `crates/mesh/src/chords.rs` (filed on CHORD) and its own test,
and goes with the last one.

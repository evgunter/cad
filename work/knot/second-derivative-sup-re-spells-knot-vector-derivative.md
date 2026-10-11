---
id: second-derivative-sup-re-spells-knot-vector-derivative
kind: issue
title: nonrational_second_derivative_sup materialises the derived vector by hand beside KnotVector::derivative
status: open
opened: 2026-10-11
---


Found by the `degree-1-partial-has-four-spellings` sweep (ENCL).

`crates/geom/src/curves/second_derivative.rs`
`nonrational_second_derivative_sup` (`:44`) builds the derived vector
as `KnotVector::clamped(knots.derivative_knot_slice().to_vec(), p - 1)`
— `KnotVector::derivative`'s body, re-spelled. The two agree today (the
site gates `p < 2` first, so its refusal is the full-multiplicity
case), but a change to what `derivative` answers would not reach it.
`crates/mesh/src/chords.rs` `rational_carrier_m_bound` has the same
spelling and is already CHORD's
`rational-carrier-bound-differences-loose-vecs-beside-two-vectors`.

Disposition: `knots.derivative().ok_or(SecondDerivativeUnbounded::DerivativeKnotVector)`,
or take the second level from `SplineCoeffs::derivative` as
`props::quad`'s `DerivLadder` does.

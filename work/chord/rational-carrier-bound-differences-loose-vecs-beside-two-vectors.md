---
id: rational-carrier-bound-differences-loose-vecs-beside-two-vectors
kind: issue
title: rational_carrier_m_bound differences owned Vecs beside kv and kv1 through difference_coeffs
status: open
opened: 2026-10-10
---


Found by the `coefficient-vector-pairing-survivors` sweep (NURBS).

`crates/mesh/src/chords.rs` `rational_carrier_m_bound` builds `w_pts`
and each homogeneous channel as owned `Vec`s, differences them with
`kv.difference_coeffs`, and differences again against a `kv1` it
materialises by hand — two levels of the derivative ladder travelling
as loose `Vec`s beside two vectors, each re-minted by count.

Disposition: build each channel with `KnotVector::with_coeffs_from_fn`
and take the second level from `SplineCoeffs::derivative` (an owned
`SplineCoeffsBuf` beside `KnotVector::derivative`), as
`props::quad`'s `DerivLadder` now does.

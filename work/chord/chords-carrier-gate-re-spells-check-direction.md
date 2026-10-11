---
id: chords-carrier-gate-re-spells-check-direction
kind: issue
title: The B-spline chord count's C¹ gate re-spells patch_bound::check_direction
status: open
opened: 2026-10-11
---


Found by the `degree-1-partial-has-four-spellings` sweep (ENCL).

`crates/mesh/src/chords.rs` `nurbs_chord_count` (`:223`–`:247`) gates a
B-spline carrier with its own copy of the C¹ rule
`crates/geom-brep/src/patch_bound.rs` `check_direction` already homes:
degree 1 admitted only without interior knots (here answered as one
exact chord), degree ≥ 2 only with interior multiplicity ≤ p − 1. The
two agree today; `offset_fit`'s copy was folded onto `check_direction`
by `degree-1-partial-has-four-spellings`, and this is the one left.

Disposition: read `check_direction` and map its three refusals onto
`TessellateError::UnsupportedCurve` (keeping this site's notes and its
degree-1 single-chord answer), so the curve and patch gates cannot
drift. The degree-1 single-span case reads
`KnotVector::span_constant_breaks` if it needs the span structure.

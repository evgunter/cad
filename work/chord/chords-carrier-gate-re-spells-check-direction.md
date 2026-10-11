---
id: chords-carrier-gate-re-spells-check-direction
kind: issue
title: The B-spline chord count's C¹ gate re-spells the rule KnotVector::derived carries
status: open
opened: 2026-10-11
---


Found by the `degree-1-partial-has-four-spellings` sweep (ENCL).

`crates/mesh/src/chords.rs` `nurbs_chord_count` (`:223`–`:247`) gates a
B-spline carrier with its own copy of the C¹ rule
`crates/geom-brep/src/patch_bound.rs` `check_direction` also spells:
degree 1 admitted only without interior knots (here answered as one
exact chord), degree ≥ 2 only with interior multiplicity ≤ p − 1. The
two agree today; `offset_fit`'s copy was folded onto `check_direction`
by `degree-1-partial-has-four-spellings`, and this is the one left.

Disposition: read the rule from its geom-core home, a match on
`KnotVector::derived` (`geom_core::spline::Derived`): `Vector` admits,
`SpanConstant(breaks)` admits only with `breaks.len() == 2` (here the one
exact chord), `Discontinuous` is the C⁰ kink. `patch_bound::check_direction`
reads the same match since PR 4754, so the curve and patch gates can no
longer drift; mapping onto `TessellateError::UnsupportedCurve` keeps this
site's notes. `rational_carrier_m_bound`'s `kv1` (CHORD's
`rational-carrier-bound-differences-loose-vecs-beside-two-vectors`)
is `derived`'s `Vector` arm.

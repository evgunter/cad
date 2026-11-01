---
id: curved-trim-e2e-fixture-waits-for-a-producer
kind: issue
title: No end-to-end body exercises the trimmed lane's chord machinery; the only General image at rest is a 2e-16 rectangle
status: open
opened: 2026-09-18
priority: P3
cost: D
---


## What

TRIM-2 PR-1 lifted `mass_properties` on a face whose trim region is what
its chart IMAGE bounds. The end-to-end evidence for it is E1
(`sweep/tests/m8_4_intersection_iso.rs::a_degree_two_widening_measures_against_the_oracle`),
and **E1 cannot see the chord machinery at all.**

Measured by both arms of the v6 dual on `0c7cc6637`, independently:

- the fixture's `General` image runs `u ∈ [2 − 2.2e-16, 2]`, so its
  trim region differs from the rectangle `[1,2] × [0,1]` by 2e-16;
- every non-trivial chord of that polygon is a horizontal cap rim with
  `slope = 0`, so the trimmed face's flux is the two rims' `G_f`;
- E1 stays GREEN under the mutant that forces every chord's slope to
  zero (M1), and goes red only under sign-class mutants (M3: volume
  `6.6227e-9` against the oracle's `7.4506e-9`).

So the evidence for the chords, the cuts, the lunes, the monotone row
and the area rule is `geom-brep`'s own module rows (Q1–Q13), which are
hand-built charts. That is a real coverage boundary and it is disclosed
in the PR body; this file is so that it does not stop being disclosed
when that PR closes.

## Why there is no better fixture today

`Pcurve::General` is minted at exactly one site at rest —
`nurbs_iso_derive`'s `Intersection` arm on an interior column
(`derive_general_image`) — and `edge_nurbs::PXN_IMAGE_DEGREE = 1`, so
every image is a degree-1 interpolant of 33 feet of a seam that is very
nearly a column. A non-degenerate curved trim needs a producer that
mints an image with real chart curvature. Two candidates, neither
scheduled here:

- **#264's banked edge work** raises `PXN_IMAGE_DEGREE`, which is what
  the spec's §1 names as the day the lune step acquires a live input.
- **STEP import of a trimmed NURBS** (`step-import/adopt.rs` can mint
  `General` at import) — outside the corpus and outside TRIM-2's fence.

PR-2's tessellation seam adds neither.

## What would close this

One at-rest body whose face carries a `General` image with chart
curvature, run through `mass_properties` against an independent oracle
(the rectangle lane on a complementary region is the shape Q8 uses).
Until then the module rows are the evidence and the PR says so.

## Home

TRIM's, filed by TRIM-2 PR-1's fix pass at the moment the boundary was
measured (the v6 dual's R1 NOTE-5 and R2 N-1, converged).

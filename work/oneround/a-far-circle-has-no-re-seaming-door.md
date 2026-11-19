---
id: a-far-circle-has-no-re-seaming-door
kind: issue
title: A circle has no re-seaming door (Curve3::seamed_near is None for one), so a far circle's arc is near its seam only where its mint put it there
status: open
opened: 2026-11-19
priority: P3
cost: M
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Found reading the tree for the phase-1 measurement of
`oneround/conic-vertex` (2026-11-19, merge base `b80352cd9`). Not
probed: a finding by reading.

- `Curve3::seamed_near` (`crates/geom/src/curves.rs`) answers
  `as_ellipse().map(|e| e.seamed_near(theta))`, so it is `None` for a
  circle; `geom::curves::tests` pins it
  (`circle.seamed_near(1.0).is_none()`). The conic module's convention
  ("a caller seams an ellipse at the principal vertex nearest the part
  of it in use") is stated for an ellipse only.
- The seam is the one parameter at which a conic's point is exact in
  its stored terms (`c + u_ref·semi_u`, no transcendental). The
  near-gap families of
  `the_near_gap_families_keep_their_certified_answers` recover fully
  (7 → 40 of 40) when the point is read from the seam vertex, and
  their circles are seamed at the graze only because the fixture
  builds them so (`tangent_circle`: `u_ref = toward`).
- A circle whose centre lies far from the arc in use, minted with its
  seam elsewhere, gets none of that: away from the seam the vertex
  form's offsets are as long as the radius.

What to weigh: re-seaming a circle moves every parameter on it, so it
is a re-mint of the edges that hold spans on it, not a reading. It
belongs with the stored-form design
(`work/honepair/the-section-table-admits-a-curve-on-its-resolution.md`:
"anchored on the arc its edge uses: parameter zero there").

Not established: which mints hand a far circle to a reader with its
seam away from the arc. The circle mints were not surveyed.

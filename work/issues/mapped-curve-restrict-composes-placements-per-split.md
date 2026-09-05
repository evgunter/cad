---
id: mapped-curve-restrict-composes-placements-per-split
kind: issue
title: MappedCurve::restrict composes the anchored rotation into the stored placement per split, re-applying rotation_about's diagonal enclosure each time — compose in the parameter, keep one placement
status: open
opened: 2026-09-05
refs: [rotation-about-diagonal-width-floor, 1277]
---

`MappedCurve::restrict` (`crates/geom-brep/src/mapped.rs:232`) advances
the map's start by composing the `s0` motion INTO the stored placement:
the `RevolvedPoint` arm writes
`place: Affine3::rotation_about_axis(axis_origin, axis_dir, s0 * angle) * place`
(`crates/geom-brep/src/mapped.rs:251`), and the `ExtrudedPoint` arm the
same shape with a translation (`:240`). Every split therefore applies
`Mat3::rotation_about`'s diagonal enclosure to the stored placement
once more — the `8.88e-16`-wide entry at an exact angle that
`rotation_about`'s doc decomposes (`crates/geom-core/src/linalg/mat.rs`,
the width-floor paragraph) — and the stored width grows LINEARLY in the
split count, with no convergence. Measured on an exact-axis fixture by
the law row
`crates/geom-brep/tests/revolved_point_anchor.rs:208`
(`stored_restriction_width_grows_linearly_in_the_split_count`, doc at
`:176`): `eval(0)`'s width is `2.66e-15` at zero splits and rises by
`3.552713678800501e-15` per split, the same to the bit at every step.
The row pins the law (linearity, a slope of the order of the diagonal
enclosure times the coordinate scale), not the digits.

**The fix is composition-side, not a respell of `rotation_about`.**
`work/props/rotation-about-diagonal-width-floor.md` rules that the
diagonal's floor is the backend's `cos` enclosure and is not respelled
(a half-angle `t` and `c` recovers at most a sixth of it). What grows
here is not the floor but the NUMBER OF TIMES it is paid, and that is
the restriction's choice: **compose in the PARAMETER — restrict the
domain, keep one placement.** A restricted `RevolvedPoint` carries the
original `place` and a start offset in the parameter (a start angle
`s0·angle`, or the sub-range `(s0, s1)` against the original sweep) so
that `eval` applies ONE anchored rotation,
`rotation_about_axis(axis_origin, axis_dir, θ₀ + s·θ)`, for any split
count; the `ExtrudedPoint` arm the same with a start displacement
inside the parameter. In exact arithmetic
`restrict(s0, s1).eval(s) = eval(s0 + (s1 − s0)·s)` either way (the
contract at `:222`); at `Interval` the parameter form pays the diagonal
enclosure once. The caller's re-certification against
`carrier_matches_mapped_source` is unchanged. `SketchSegment::restrict`
(`:104`) has the sibling shape at the endpoints (its own anchoring note
at `:98`) and is out of this item's scope.

Filed from the rotation-floor unit (PR 1980's rider, split out of
`rotation-about-diagonal-width-floor`, where the two travelled as one
entry). `mapped.rs` is in no program's `paths:` at this head, so it
waits here for its owner.

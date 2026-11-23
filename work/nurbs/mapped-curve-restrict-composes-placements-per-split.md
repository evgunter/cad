---
id: mapped-curve-restrict-composes-placements-per-split
kind: issue
title: MappedCurve::restrict composes the anchored rotation into the stored placement per split, re-applying rotation_about's diagonal enclosure each time — compose in the parameter, keep one placement
status: closed
opened: 2026-09-05
refs: [1277]
priority: P1
cost: H
closed: 2026-10-10
pr: 4441
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
contract at `:228`); at `Interval` the parameter form pays the diagonal
enclosure once. The caller's re-certification against
`carrier_matches_mapped_source` is unchanged. `SketchSegment::restrict`
(`:104`) has the sibling shape at the endpoints (its own anchoring note
at `:98`) and is out of this item's scope.

Filed from the rotation-floor unit (PR 1980's rider, split out of
`rotation-about-diagonal-width-floor`, where the two travelled as one
entry). `mapped.rs` is in no program's `paths:` at this head, so it
waits here for its owner.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/props/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). Filed from a PROPS lane (PR 1980's rider); `crates/geom-brep/src/mapped.rs` is in no program's `paths` and PROPS — enclosure certificates and interval honesty — is its natural owner. PROPS draws the fence on `mapped.rs` in the PR that takes it.

## A reference that outlived its program (2026-10-03)

`rotation-about-diagonal-width-floor` was PROPS' and was deleted with `work/props/` when that program closed.
It is dropped from this row's `refs:` because `refs` names live items; the
finding is unchanged and readable at `git show 63df2069c:work/props/<id>.md`,
and PROPS' done-state of record is `docs/doc-ledger/props-leaves-the-tracker.md`.

## Closed (2026-10-10, PR 4441)

`MappedCurve::RevolvedPoint` and `ExtrudedPoint` keep the whole sweep's `angle`/`vec` and placement as built, and carry a `SweepRange`: the sub-range of the whole sweep's normalized parameter `u ∈ [0, 1]` they cover.
- `restrict` narrows that range and never touches `place`. A dyadic split is exact in `u`.
- `eval` applies one motion, at `range.at(s)·angle` or `vec·range.at(s)`.
- On a whole range the output is main's, bit for bit, at f64, Interval and `Sym`.
- `offset_axial::reauthor` re-authors in the parameter. An unmoved start stores `place⁻¹(p)`, which is width 0 for an exact corner, and a turned start reads through main's composite.

Measured over 64 splits (Interval, far placement):
- end-anchored chains stay flat, where main grew 9–65×;
- every chain is at or below main, except two interior chains with an inexact start at 1.06× and 1.03×, which are one outward rounding per split;
- a turned-start reauthor is 1.2× main at f64 at 1e3, and 1.5× (3 ulps against 2) at 1e5, with its stored point bit-identical to main's.

Review: a FULL review (one MAJOR: reauthor stored a rotation enclosure), then a delta review (which disproved the first pass's "no stored form keeps both ends flat"), then this second pass on that delta review's proposed form.

Rows filed:
- `revolved-point-eval-levers-angle-width-by-the-coordinates` (open: the eval's anchoring is the remaining lever);
- `sketch-segment-restrict-re-derives-endpoints-per-split`.

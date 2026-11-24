---
id: the-arc-held-cover-is-looser-than-the-centre-forms-on-a-long-window
kind: issue
title: An arc-held ellipse's cover is cut into four chord pieces whatever its length, so over a long window it is looser than the centre form by a piece's bulge either way, and it is not met with its whole conic's
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Filed by `oneround/conic-readers` (2026-11-23), from both reviews of
PR 7510.

## What is true today

`geom::cover::Cover::arc_of` (`crates/geom/src/cover.rs`, the piece
`HeldArc`) covers an ellipse held on its arc through its own points: the
window is cut into four pieces whatever its length, and each piece is
bounded out to its bulge `(1 − cos h)·a`. `Cover::arc`, the centre
form, reads a mean circle about the centre and is met with the whole
conic's cover. So on a long window the arc-held cover claims less, by a
piece's bulge on both sides, and a reader can read past the whole
conic's extreme by that much. Every ellipse `plane_cone_section` mints
is arc-held, at every size, so this reaches ordinary sections.

It is the safe direction: no point of the arc is outside the cover
(`cover::tests::held_arc_rows`, `held_arc_sweep`).

## Measured (the PR's reviewers, at `bd07a867e`)

- A full turn, `a = 0.9`, `b = 0.4`, from a point 0.57 m off the arc:
  the distance range `[0.458, 1.860]` centre-held, `[0.347, 2.006]`
  arc-held; the ball's radius 0.900 against 0.985.
- `a = 2.2e4`, window `[0, 0.3]`, the same site: the nearest distance
  0.458 centre-held, 0 arc-held.
- Over 1,008 arc-held `f64` distance ranges of one reviewer's dump: the
  nearest tighter in 515, lost to exactly 0 in 182, the farthest looser
  in 430. Of 14,544 arc-held readings dumped, 11,559 moved.

No verdict of the fast or the filtered slow set moved on it at ε 1e-6,
1e-9 or 1e-12.

## What to try

- The count of pieces chosen from the window's length, so a piece's
  bulge stays a set fraction of its chord.
- A whole-conic statement from the stored point to meet with: the
  centre form's mean circle posed about the anchor, its `off_axis − ρ`
  read as `(|w|² + 2·w·R)/(off_axis + ρ)` with `w` the site from the
  anchor and `R` the anchor from the centre, which does not cancel. Not
  built, not measured.

`Cover::arc`'s sentence "no reader of a cover built here reads past its
whole conic" is true of that door and says that `arc_of` is not met so.

## Its band, and one more looseness (PR 7510's last review)

P2 as filed. One reviewer's argument for more, for whoever takes this
row: arguably P1, because `Arc` and `HeldArc` are two covers of one
locus: the same ellipse reads a different region according to how it
is held, and a reader cannot tell which it was handed.

`HeldArc::pieces` and `Arc::far` cut the same four pieces in two places
and share only `bulge_factor`; the tightening here is where they get
one home.

At `Interval` the support along the ellipse's own stored normal is
wide by about nine roundings of the SEMI-AXIS (4.3e-11 m at 2.2e4 m
over a 2 mm window), as the centre form's is: the harmonic's two
coefficients enclose zero at that width
(`held_arc_rows::the_interval_support_along_the_arcs_own_normal_is_a_rounding_of_its_semi_axis`).
It is not the arc's own scale.

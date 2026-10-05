---
id: near-parallel-planes-refuse-under-a-false-predicate
kind: issue
title: Two planar rests a hair past parallel classify as a line and then refuse a NON-FINITE translation margin under mate_member_translation_in_plane — a false cause
status: closed
opened: 2026-09-20
priority: P0
cost: H
parent: MSOLVE-12
pr: 3698
closed: 2026-10-01
---



## What

Found by MSOLVE-8's boundary sweep (PR 2896,
`crates/editor-core/tests/msolve8_levered_clash.rs`,
`c2_parallel_boundary_direct`). Two planar cosets whose normals differ
by a tilt whose levered sine sits just past K·ε — so
`intersect_subgroups` (`crates/editor-core/src/mate/coset.rs`) rightly
calls them NON-parallel and returns the prismatic line — then reach
`candidate_translation` with two nearly coincident plane constraints
and a residual along their line. The assembled system
`free1 · P2 · free1 + P1 + outer(d, d)` is singular to rounding for
that pair (`free1 · P2 · free1` is of the order of the squared sine),
`inverse3` yields non-finite entries, and the candidate's membership
refuses the added mate with `Indeterminate { margin: Invalid,
predicate: "mate_member_translation_in_plane" }` — "a case split
could not be decided" naming a predicate that measured nothing, for a
cause that is the conditioning of the translation system. The refusal
reports a false cause: the planes ARE decidably non-parallel (the
table just said so), and the in-plane translation is not what could
not be decided.

This is pre-existing: the same pair takes the same path on `main`
(`parallel` Positive → prismatic → the same system). MSOLVE-8's one
decision at `mate_axes_parallel` changed which side of K·ε a pair two
ulps from it falls on, not this stage.

## Reproduction

The row's search: `n1 = ẑ`, `arm = 3.7`, tilt `n1` by `s ≈ 2.849e-9`
along an in-plane shape so `‖(n1 × n2)·arm‖` lands a few ulps above
K·ε at `Tol::witness()`; `intersect(Planar{n1} @ I, Planar{n2} @ I,
band, arm)` refuses as above while `intersect_subgroups` answers
`Prismatic`.

## What it wants

Either the translation stage decides its own conditioning under a
name of its own (a `mate_translation_system_singular` split on the
determinant, levered honestly) and refuses with that cause, or the
table's non-parallel verdict at the edge is what the translation
stage consumes and the system is solved in the line's own frame
where it is not singular. MSOLVE's charter (one refusal that reports
a false cause); `coset.rs`'s `candidate_translation` and
`inverse3`'s "a singular input would yield non-finite entries, which
the membership check then refuses" sentence, which is the behaviour
this row measured and is honest about the mechanism but not the
cause the user reads.

## Closed (2026-10-01, PR 3698)

Final state (a), solving in the frame of the remaining freedom. The
measurement on the tree (MSOLVE-12's census over every table entry
whose verdict turns on a sine or a cosine, at the ulps around the
band's edge) found the class wider than this row: the planes, a plane
with a near-perpendicular line or prismatic, and the three
near-parallel line/prismatic pairs all reached a non-finite candidate,
refused as `Indeterminate { margin: Invalid }` under
`mate_member_translation_in_plane`, `mate_member_point_on_axis` or
`mate_member_translation_along`. `candidate_translation`
(`crates/editor-core/src/mate/coset.rs`) now solves each shape of the
held freedom the residual does not keep in closed form, dividing only
by the levered sine or cosine the table decided away from zero
(re-spelled bit for bit) or by a number near one; `inverse3` is gone.
A candidate or membership length the format cannot hold refuses
`FoldStop::OutOfRange`, which the solve reports as the new
`MateFault::PoseOutOfRange`, carrying `RANGE_RECOURSE`.

Rows: `msolve8_levered_clash::c2_parallel_boundary_direct` (this row's
search; the full door now agrees with the table at all 134 400 pairs),
`msolve12_honest_translation::c1_every_separated_pair_measures_what_it_refuses`,
`c2_the_pose_is_the_independent_intersection`,
`out_of_range_at_the_coset`, `out_of_range_through_the_doors`. The
residue the census found past the session's range is filed as
`a-far-meeting-point-fails-membership-by-its-own-rounding`.

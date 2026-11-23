---
id: a-needle-cone-through-its-apex-takes-most-of-an-hour-to-mesh-one-cell
kind: issue
title: a 0.001° cone through its apex takes more than 40 minutes to mesh one cell
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Reported by the review of PR 7054 (`a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`); not reproduced by the unit that files it.

## Finding

A cone of half-angle 0.001° through its apex (radius `1.7·10⁻⁵`, height
1) ran more than 40 minutes in one cell of the review's grid, on that
PR's base and its head alike: the time is not the unit's change.

## Not measured

Which pass the time is in (the chord pass's counts, the lattice's rows
graded toward the apex, the CDT, or the retry loop), at which δ, and
whether it ends in a mesh or a refusal. `trimmed::MAX_SPHERE_EDGE_CHORDS`
bounds one face's boundary on the sphere arm for this reason; the cone
arm has no such budget.

## 2026-11-18 — unit 4c (`tesslane/ruled-density`)

The review of PR 7096 ran the base over 17 minutes on a cell of a 0.001°
cone through its apex (and on a half frustum split at `1 − 10⁻⁸`, placed);
the unit's head answers both at once. On that head a 0.001° cone of
height 1, a full turn, its line edge split `10⁻⁸` from its apex end and
placed obliquely, answers in about a tenth of a second at δ 0.1 and 0.01
(the whole row, `split_rulings::a_split_beside_the_apex_of_a_needle_cone_refuses_undecided`,
in 0.17 s): the face the walk takes is written row by row, and the one it
does not (its apex undecided) refuses `SingularUndecided` before its
lattice is laid. Whether that is this row's cell, and where the base
spent its time, was not measured; the item stays open for a cone face the
route does not walk.


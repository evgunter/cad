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

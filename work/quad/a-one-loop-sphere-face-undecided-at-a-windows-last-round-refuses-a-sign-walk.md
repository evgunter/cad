---
id: a-one-loop-sphere-face-undecided-at-a-windows-last-round-refuses-a-sign-walk
kind: issue
title: A one-loop sphere face whose sides are undecided at a window's last round refuses where the schedule measures it
status: open
opened: 2026-11-11
priority: P2
cost: M
---


## What

Found in review of PR 6808 (SPIRICARM stage 3), probing whether the
torus's one-loop regression has siblings on the sphere and the
cylinder. It does on the sphere, and on main.

`geom_brep::props::quad::rotational_cut_face_rounds` reads a sphere
face's sides at the round a window ends (`require_sphere_sides`, main
`a0f53219e4` `crates/geom-brep/src/props/quad/rotational.rs:1467`,
reached through `Run::ringed` at `:1390`): each loop's outside as a hole
(`props_ring_winding`) and the face's area as its whole
(`props_chart_area_side`). A face of ONE loop is asked both. At a
single-round window (`RoundWindow::at(k)`, the sign walk's) the coarse
round's enclosure of a small disc's area, or of the outside of a
sphere less a small disc, straddles the band, and `Run::ringed`
(`crates/geom-brep/src/props/quad/run.rs:403`) lets that escalation
stand at the window's last round. The schedule measures the same face.

This is `a-ring-undecided-at-a-windows-last-round-refuses-a-sign-walk`'s
mechanism, reached by a face with no ring: the sphere asks a one-loop
face which side it is, and the cylinder's and the torus's readings do
not (the cylinder decides a one-loop face's whole only, which a harmonic
disc's exact terms decide at every round; the torus reads its side as a
count window, PR 6808).

## Witness

The sphere fixtures of
`props::quad::rotational::swap_rows::sphere::tiny_and_near_full_faces_hold_or_refuse`
(ball radius `0.7`, the circle `tilted(ball, (1.0, 0.2), θ, _)`, its
image projected), read at `RoundWindow::at(k)` for `k = 0..5` and at the
schedule, on PR 6808's head (whose sphere sides read is main's,
refactored without a change of value):

| face | escalates at | schedule |
|---|---|---|
| disc `θ = 1e-2` | `at(0)`, `props_chart_area_side` | measures |
| disc `θ = 1e-3` / `1e-4` | `at(0..1)` / `at(0..2)`, `props_chart_area_side` | measures |
| sphere less disc `θ = 1e-2 … 1e-4` | `at(0)` … `at(0..2)`, `props_ring_winding` | measures |
| sphere less the polar cap `θ = 1e-5` | `at(0..4)`, `props_ring_winding` | measures |
| sphere less the polar cap `θ = 1e-6` | `at(0..5)`, `props_ring_winding` | measures |
| disc of radius `π − 1e-2` / `π − 1e-3` | `at(0)` / `at(0..1)`, `props_ring_winding` | measures |

A disc of `θ = 1e-5` or less is `DegenerateFace` from `at(4)` on and
under the schedule; that is not this row. The cylinder's one-loop
harmonic disc (`rectangle_with_circle`'s ring alone, radius `1e-1` to
`1e-6`) decides at every `at(k)`.

## The shape of a fix

The fix the ring row names covers this one: an outcome for a window
that ends on an undecided in-band decision with rounds remaining. Or,
narrower: a one-loop sphere face's side read as a count window the way
the torus's is (the face's area held for both sides, deciding nothing),
which no window end can leave undecided.

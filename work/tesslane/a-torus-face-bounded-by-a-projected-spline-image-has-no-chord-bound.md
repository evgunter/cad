---
id: a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound
kind: issue
title: a torus face bounded by a spline section's projected image refuses in the chord pass, so a rod or a ball through a donut builds and does not mesh
status: open
opened: 2026-11-17
priority: P2
cost: M
---


Filed from PR 7052 (SECTARMS, the closed-loop carrier), whose rows meet
it; the torus half of
`cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm`,
which names only the sphere. (`trimmed-tessellation-lacks-torus-and-plane-arms`
closed on 2026-11-14 with PR 6950 and does not cover it.)

## Measured

`crates/mesh/tests/quadric_pair_sections.rs`,
`a_ball_and_a_rod_against_a_donut_build_on_periodic_loops`, at δ 5e-3:
the donut `(0.6, 0.2)` about `y` with a rod of radius 0.1 along `z`
through `(0.62, 0.03)` (∪, ∩, donut ∖ rod), and a ball of radius 0.5
against that donut turned 0.7 about `x` and moved `(0.1, 0.35, 0.2)`
(∪, ∩). Every op builds; the section edges are spans of periodic
cubics whose readings hold simple interior knots only. Each then
refuses `UnsupportedCurve` from `mesh::chords::chart_image`
(`crates/mesh/src/chords.rs`): "trimmed analytic face bounded by the
projected image of a spline or line carrier — the trimmed lane bounds
the UV speed of a conic's image only". The refused edge lies between a
torus face and the cylinder or sphere face.

## What a fix owes

A speed bound for a spline's `Pcurve::Projected` image on a torus
chart that the chord pass can read, as the sphere row needs on a sphere
chart (`a-projected-spline-images-speed-bound-on-a-sphere-is-eleven-orders-loose`
measures how loose today's is there).

(LATHE, 2026-11-23, PR for `lathe/mesh-budget-one-band`) The row is renamed
`a_rod_through_a_donut_builds_and_a_ball_against_it_refuses_its_interior_loop`:
with every full revolve one face round its axis, the ball and the donut refuse
upstream of the mesh at every op
(`work/lathe/a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate.md`),
so only the donut and the rod still reach the torus face's projected image here.

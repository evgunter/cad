---
id: a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound
kind: issue
title: a torus face bounded by a spline section's projected image refuses in the chord pass, so a rod or a ball through a donut builds and does not mesh
status: open
opened: 2026-11-17
priority: P0
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

## Re-banded P0 (2026-11-25)

The teapot's handle union builds, and the joined pot does not mesh: the
section edge between the vessel's sphere and the handle's torus refuses
`UnsupportedCurve` from `mesh::chords::chart_image` ("trimmed analytic
face bounded by the projected image of a spline or line carrier — the
trimmed lane bounds the UV speed of a conic's image only"), measured on
the tour's `tess-budget` sweep at δ 2e-4 (`teapot/teapotvessel`). Teapot
wall 2 (`demos/tour/src/teapot.rs`, `wall_probes`) pins it, so this row
is a stop of the P0 umbrella `splice/curved-demo-joins-build` and
carries its band. The joined pot needs both this row and its sphere half, `cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm`.

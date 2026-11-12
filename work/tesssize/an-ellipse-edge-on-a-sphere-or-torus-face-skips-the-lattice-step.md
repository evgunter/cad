---
id: an-ellipse-edge-on-a-sphere-or-torus-face-skips-the-lattice-step
kind: issue
title: The chord pass steps an ellipse edge by its curvature alone, never the sphere or torus lattice's boundary step, though an in-band ellipse now certifies on both charts
status: open
opened: 2026-11-04
priority: P3
cost: E
---


Filed by PCERT (`pcert/ellipse-general-lane`, the ellipse's general
lane).

## What

`mesh::chords`' edge schedule steps a circle edge by its sagitta and
then tightens it to every adjacent face's lattice step
(`grid_boundary_step`, the sphere's `sphere_arm_tighten` among them), so
the trim polygon and the face's lattice are sized from one value. Its
`Curve3::Ellipse` arm takes `ellipse_step` alone, on the stated ground
that "an ellipse never lies on a torus chart of this kernel's
constructions".

An ellipse within the band of a sphere or a torus now certifies on that
chart through the projected image (D3's lane choice), and the sphere
arm's speeds already read its image generically
(`chords::sphere_uv_speeds`, through `FramedCarrier::Conic`). So a
sphere or torus face bounded by such an ellipse, from an import or a
producer that stores one, is stepped without its lattice's tightening.
No kernel constructor mints one today.

## Fix

Run the adjacent faces' `grid_boundary_step` on the ellipse arm too (it
reads the face, not the carrier's kind), and drop the comment's
premise.

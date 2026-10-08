---
id: spiric-bounded-face-area-is-unimplemented
kind: issue
title: A face bounded by a Curve3::Spiric has no area or volume lane: loop_vector_area and the torus wall's closed-form parse refuse Unimplemented, which is where the hollowed klein elbow and the sectioned torus vessel now stop
status: open
priority: P2
cost: H
refs: [c5-plane-torus-cone-cylinder-arms]
opened: 2026-10-01
---

Filed by CURVED at its close (2026-10-01).

## What

`Curve3::Spiric` (the exact axis-parallel plane×torus section, CURVED's
spiric unit, PRs 2566 and 2861) bounds the planar caps and the torus
walls of a hollowed partial torus. Mass properties refuse on both:

- a cap bounded by a spiric: `props/loop_area.rs`'s `loop_vector_area`
  answers `PropsError::Unimplemented` — the area under a spiric is an
  elliptic integral;
- a torus wall bounded by a spiric: `props/curved.rs:torus_boundary`'s
  named `Spiric` arm refuses `NotIsoRectangle`, and the trimmed
  quadrature lane refuses on a spiric half-edge.

**Who stops here**: the sealed klein elbow (`torax_axial`,
`verbs_shell`'s sealed arm, `shell7_seam_corner`, `spiric_rim`,
`torax_interval` — all pinned at check 7's `VolumeUncomputable { Face
{ …, Unimplemented } }`), the tour's `torusvessel` wall 1, and the
Klein demo's wall-pair re-authoring (rows 3/4/8, now on GERM's
`c5-plane-torus-cone-cylinder-arms`). Each row's doc names this door.

## Shape

The spiric's closed form is `P(v) = c + n·d + m·f(v) + a·r·sin v`,
`f = √((R + r cos v)² − d²)`; its chart images are exact
(`Pcurve::Spiric`, `SpiricImage::{Cap, Wall}`). A certified area needs
either a quadrature of the cap's Green integral over the spiric arc
(a certified `f` bound is in `geom::spiric_f_range`; C9 now admits
`√`, Ev #3517) or the wall's flux through the trimmed lane with the
spiric image as a chord-able curve. The design doc
(`docs/CURVED-SPIRIC-DESIGN.md`, Q5) recorded "the props lane funded as a separate numeric unit right
behind" the carrier; this is that unit.

---
id: second-order-cap-window-escalates-small-drafts
kind: issue
title: a fillet cut off at a slightly tilted plane end face escalates over a window that grows as the radius shrinks and the tolerance coarsens
status: open
opened: 2026-10-06
priority: P3
---


## What

A plane–plane fillet, or a ruled band, cut off at a plane end face
tilted `θ` off square ends in an ellipse with semi-axes `r` and
`r·sec θ` (`crates/sweep/src/blend/battery.rs`, `cap_transverse`). Two
decisions read the tilt: `fillet3_cap_transverse` on its first-order
departure `sin θ` levered at the link's length, and the ellipse door
(`geom::Curve3::ellipse`, `ellipse_axes_distinct`) on the axes'
difference `r·(sec θ − 1) ≈ r·θ²/2`. A tilt the first decides definite
can still leave the second under the band's upper edge `e`, and that
window escalates as `BlendDecision::CapEllipse`, up to a tilt of about
`√(2e/r)`. It is much wider than the departure's own band, and widens
as `r` shrinks and `ε` coarsens.

Measured (PR 4173's review probe `probe_draft_window`, a trapezoid's
top edge filleted at its drafted end walls):

- default ε (band `(1e-9, 1e-8)`): `r = 1 mm` escalates at a draft of
  0.25° and builds at 0.5°; `r = 1 cm` escalates at 0.05°;
  `r = 0.1` builds from 0.05°.
- ε = 1e-6: `r = 0.1` escalates up to 0.5°; `r = 1 cm` and `r = 1 mm`
  escalate through 2°, every draft tried.

Pinned at the default ε by `band_planar_oblique_fillet.rs`
`a_drafted_wall_escalates_on_its_ellipses_axes`, which reads each draft
against the run's band.

None of this regresses: before PR 4173 every oblique end refused
`END_FACE_OBLIQUE`.

## The shape of a fix

Building the circle in the window would take two decisions to pick one
kind, and a circle stored in the tilted plane is read as coaxial by
`topo::props`, which misreads the volume to first order in the tilt.
What could narrow the window is an ellipse carrier whose kind does not
hinge on `ellipse_axes_distinct` at the band's scale — a representation
question for the ellipse door, not for the blend.

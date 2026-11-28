---
id: the-outward-normal-at-a-point-has-no-spindle-sheet-arm
kind: issue
title: The outward normal at a point, and the foot-normal clearance, read no spindle sheet face
status: open
opened: 2026-11-25
priority: P3
cost: E
refs: [the-point-door-places-no-point-on-a-spindle-sheet-face]
---


## Finding

Found by lane `inside-spindle-sheet` (2026-11-25), in the sweep for
siblings of the sheet arm it gave `topo::contain`'s
`curved_interior_placement`.

`topo::face_normal`'s point arm answers `Ok(None)` for
`geom::Surface::Spindle` (its comment: "A spindle sheet's arm is not
written yet"), and its foot-normal clearance answers
`FootNormalError::Unread` for one. The sheet's normal is the torus
formula's with `R` signed (`geom_brep::implicit_gradient` already reads
`Torus | Spindle` alike, and `topo::torus_region` takes its outward
normal there), so the arm is the torus's, with the clearance asked of the
sheet's own singular points: its apexes on the axis, where it has no
normal, rather than a ring's tube circle.

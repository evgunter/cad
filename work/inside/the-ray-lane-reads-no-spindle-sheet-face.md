---
id: the-ray-lane-reads-no-spindle-sheet-face
kind: issue
title: The point-in-solid ray lane crosses no spindle sheet face, though the torus region reading now places a point on one
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [the-point-door-places-no-point-on-a-spindle-sheet-face]
---


## Finding

Found by lane `inside-spindle-sheet` (2026-11-25), in the sweep for
siblings of the sheet arm it gave `topo::contain`'s
`curved_interior_placement`.

`topo::solid_contain`'s `ray_crosses` answers `false` for
`SurfaceKind::Spindle`, and the face dispatch refuses a sheet through
`uncrossable` (the `Surface::Spindle { .. } | Surface::Nurbs(_) |
Surface::Approx(_)` arm of its face reading). So a point-in-solid query
on a body with a sheet face (a lemon or an apple revolved by
`sweep::revolve`) refuses, where the same body's ring-torus twin
answers.

The face's region is no longer what is missing:
`topo::torus_region::torus_face_region` reads a sheet face, edges that
end at an apex included (its meridian legs end at the apex, and a root
there is dropped before its plane section is asked: module docs
"# A spindle sheet", `topo::region::landings`). What
the ray lane would still need is the ray's crossings with the sheet:
the torus's quartic with `R` signed, each root kept where it lies on the
sheet (`ρ ≥ 0` on its own arc, `R + r·cos v ≥ 0`), and an apex, where
the surface is a cone's point, read as a cone's apex is.

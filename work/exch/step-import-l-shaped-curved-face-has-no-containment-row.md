---
id: step-import-l-shaped-curved-face-has-no-containment-row
kind: issue
title: No row imports a STEP solid with an L-shaped torus or cone face and asks point_in_solid at its notch
status: open
opened: 2026-09-29
---


Found by CONTACT-11's review (MINOR-4).

## The door

`step-import` adopts `TOROIDAL_SURFACE`, `CONICAL_SURFACE` and
`SPHERICAL_SURFACE` faces with arbitrary certified loops
(`crates/step-import/src/entities.rs`, the `TOROIDAL_SURFACE` and
`CONICAL_SURFACE` arms), and `topo::point_in_solid` is public. So a
CAD file with an L-shaped torus or cone face reaches the trims'
chart-box checks, `torus_chart_windows` and `cone_trimmed_window` in
`crates/topo/src/boolean/solid_contain.rs`. Today no boolean can mint
such a face: `topo::subtract` refuses the torus × plane pair at
`RevertRoster` (`crates/sweep/tests/contact11_torus_chart_l.rs`).

## What is guarded, and what is missing

CONTACT-11 made both checks linear and metric
(`solid_contain::chart_polygon_box`), so an imported L now refuses as
`PartialTorusFace` / `PartialConeFace` rather than being served its
bounding box. The rows that pin this build the face by hand with Euler
ops (`section_cert_rows.rs`,
`a_small_notch_in_a_torus_face_refuses_at_every_size`,
`a_small_notch_in_a_cone_face_refuses_at_every_size`). No row drives
the same shape in through the import door. Doing so needs a closed shell
authored in STEP: for example, a quarter-turn torus sector with one
quadrant of its outer wall cut back by an axial plane and a horizontal
plane, which gives the wall an L. The row would then ask
`point_in_solid` at a point in the notch and expect `Out` or a typed
refusal, never `In`. That is more than CONTACT-11's fence
covered.

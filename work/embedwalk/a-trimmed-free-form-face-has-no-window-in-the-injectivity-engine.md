---
id: a-trimmed-free-form-face-has-no-window-in-the-injectivity-engine
kind: issue
title: The injectivity engine opens no window on a trimmed free-form face, so a finished body holding a NURBS or approximated face cut off its iso boundary refuses Unsupported at the at-rest gate
status: review
opened: 2026-11-25
priority: P0
cost: H
branch: embedwalk/trimmed-freeform-window
pr: 7787
---


## Finding

`topo::clearance`'s window builder (`window_of`) admits a free-form
face only where it is the whole patch (`iso_bounded_domain`): no inner
loop, every boundary image an iso by variant, run once around the knot
domain. Every other NURBS face, and since an approximated face is read
as its fit (OFFSET-DESIGN O2) every other `Approx` face, refuses
`Unsupported` naming which test it failed. No clause says a finished
body may not hold a trimmed free-form face, and boolean, split and
shell doors mint them (a fitted cap cut by a plane, a vase cap moved
onto a wall row), so each such refusal is a valid body refused at the
at-rest gate.

Measured on `embed/gate-measure` (`6eb881a63`), `sweep::all`, with and
without the approximated-face window (lane `embedwalk-approx`,
2026-11-25):

- `encl_curved_loft_shell::a_moved_plane_meets_a_fitted_cap_along_their_certified_section`:
  `Unsupported { carrier: "a free-form face with a boundary edge that is not an iso of its chart", face: FaceKey(1v1) }`
  once the approximated cap has a window (it refused as
  "an approximated face" before); the cap's boundary holds the moved
  plane's section, which is no iso.
- `encl_curved_loft_shell::shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_moves_alone_onto_a_wall_row`:
  `Unsupported { carrier: "a free-form face whose iso boundary does not run once around its knot domain", face: FaceKey(3v1) }`,
  with and without it.

## What is needed

A window on a trimmed free-form face: the knot domain as a superset
(`Region::Trimmed`), each cell's region read by the point-in-face doors
on the spline chart (`spline_region`'s placement, which containment
already reads for `Nurbs` and `Approx`), as every analytic carrier's
window is read.

## Evidence: the window (lane `embedwalk-trimmed`, 2026-11-25)

Branch `embedwalk/trimmed-freeform-window`. A NURBS face that is not its
whole patch takes its knot domain as a `Region::Trimmed` window; its
self-pairs stay on the walk (the monotone arm reads chart rectangles, a
superset of the face's part), and a cell's ball reads the carrier's
narrowing `rect_enclosure`, so a spline cell's region decides.

Measured on `embed/gate-measure` with the window patched in and the
approximated-face window on, over `sweep` and `step-import`: the fast
set's gate asks name no free-form window test; the slow set's do in
five rows, and each moves from `Unsupported` to
`Uncertified { arm: Departure }` at an edge its trimmed spline face
shares. Their remainders are filed:
`work/embed/a-spline-chart-image-bounding-a-trimmed-spline-face-is-presented-by-no-departure-arm.md`
(the fitted cap cut by a plane, and three step-import rows) and
`work/embed/a-trimmed-spline-walls-seam-side-misses-its-edge-by-the-rim-rows-rounding-at-their-corner.md`
(the vase's moved cap).

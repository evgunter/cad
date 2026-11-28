---
id: a-spline-chart-image-bounding-a-trimmed-spline-face-is-presented-by-no-departure-arm
kind: issue
title: The departure arm presents no trimmed spline face along a spline chart image, so a fitted cap cut by a plane refuses Uncertified at their section
status: open
opened: 2026-11-25
priority: P0
cost: H
refs: [a-trimmed-free-form-face-has-no-window-in-the-injectivity-engine]
---


## Finding

Found by lane `embedwalk-trimmed` (branch
`embedwalk/trimmed-freeform-window`), measuring.

With a trimmed spline face given a window (the knot domain, read as
`Region::Trimmed`), the at-rest gate's ask on
`encl_curved_loft_shell::a_moved_plane_meets_a_fitted_cap_along_their_certified_section`
(sweep, the moved box: an approximated cap read as its fit, a NURBS
face, and the moved `x = 2 − T` side) gets past the window and refuses

`Uncertified { a: FaceKey(1v1), b: FaceKey(4v1), shared: Edge(EdgeKey(10v1)), arm: Departure }`

at the first cell pair across their section edge, a plane × fit trace
whose image on the cap's chart is a spline (`Pcurve::Fitted` or
`General`), no iso. `departure` (`crates/topo/src/injective.rs`) reads
`present` of both pieces `Err(Unpresented::Structural)`, so `stuck`
holds on both and the pair refuses at depth 1 (receipt: 1 split, 1
refused, 12 abandoned).

The cap's piece is `whole == false`; `present_chart`'s curved
presentation (`curved_walls`) reads a harmonic or spiric wall image
only, and a coordinate-line use is read through its `iso`, so a spline
image on a spline chart has no presentation. The plane's piece
refuses `Structural` too: its curved presentation reads no spline
section either.

Three step-import rows move the same way, from `Unsupported` ("a
free-form face with a boundary edge that is not an iso of its chart")
to `Uncertified { arm: Departure }` at an edge their trimmed spline
face shares (measured on `embed/gate-measure` with the window patched
in):

- `a_foreign_carrier_splits_at_each_c0_knot::a_knot_vertex_nothing_else_meets_survives_the_closing_join`
  (faces 5 and 6, edge 8);
- `a_foreign_carrier_splits_at_each_c0_knot::a_trim_crossing_the_knot_line_four_times_imports_in_three_pieces`
  (faces 1 and 4, edge 22);
- `an_off_iso_line_on_a_spline_wall_imports_its_general_row::the_tetrahedron_imports_and_its_diagonal_takes_a_general_row`
  (faces 1 and 2, edge 2).

## What is needed

A presentation of a face's part of a cell along a spline chart image:
the image as a graph over one chart axis across the cell (its
monotone run), with a separator vanishing on the section (the other
face's carrier where it is a plane, as `implicit_of` reads a quadric).
Until then every boolean, split or offset door that cuts a spline face
along a non-iso section mints a body the gate refuses here.

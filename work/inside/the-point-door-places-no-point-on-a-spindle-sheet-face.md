---
id: the-point-door-places-no-point-on-a-spindle-sheet-face
kind: issue
title: The curved point door places no point inside a spindle sheet face, so a crossing through a sheet is confirmed only where its stations land on the face's boundary
status: closed
opened: 2026-11-25
priority: P1
cost: M
refs: [a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine]
closed: 2026-11-25
pr: 7790
branch: inside/spindle-sheet-placement
---


## Finding

Found by lane `embedwalk-spindle` (2026-11-25), giving a spindle sheet
its window in the injectivity engine; narrowed by that PR's review.

`topo::contain`'s `curved_interior_placement` reads a cylinder, cone,
sphere and torus face through its carrier's closed form. For
`geom::Surface::Spindle` it answers `Ok(CurvedPlacement::Trim(None))`
(the arm it shares with the plane, which `face_placement` reads before
it). The boundary pre-pass (`curved_boundary_containment`) comes first
and does place a point on a sheet face's boundary. So
`topo::cell_region::station_region` places a sheet station on its
meridians and rims, and nowhere inside the face.

The verdict door's verifier (`topo::clearance::embedded`, its
`on_face_stations`) therefore confirms a crossing through a sheet only
where a pair of boundary stations lands on it. Two lemons through one
another moved 1.2 read `Crosses { faces: (1v1, 3v1), stations:
[(-5e-324, 0.6435), (-5e-324, -0.6435)], .. }`, both stations on the
seam meridian `u = 0`. Two lemons moved 0.3, and two apples moved 1.2,
refuse on the walk's budget instead, never `Embedded`. Pinned:
`sweep::all self_intersection_proves_injectivity::footballs_through_one_another_never_read_embedded`.
Once an interior sheet station is placed, that row can ask `Crosses` of
every case.

The station's `u = −5e-324` is `0.0f64.next_down()`, the low end of the
whole turn every periodic window takes (`injective::full_turn`, one ulp
past `0` so its rounding never makes a band too narrow). A station at a
window's corner is that value, the same chart point as `u = 0`. Intended,
not a defect.

What is needed: the sheet's arm in `curved_interior_placement`. The
sheet's chart is the torus's formula with `R` signed
(`geom::Surface::Spindle`), and its inverse is in
`geom::chart_point` (`ChartArm::Spindle`), so `torus_face_containment`
with a signed major radius may serve. Its region legs decide the ring
convention (`geom::require_ring_torus`, `torus_face_region`) and refuse
a sheet today, so that is where to look first.
